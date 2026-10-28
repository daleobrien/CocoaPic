//! A text input: one line by default, wrapping and growing downward when
//! [`TextInput::wrap`] asks it to.
//!
//! Ported from gpui's own `input` example (0.2.2) — the crate's IME-correct
//! reference implementation. gpui ships no `TextInput` (PLAN.md §4), and the
//! plan's risk list names IME/selection on this widget as the single biggest
//! unknown, so it is built first and from the example rather than invented.
//!
//! The caret, selection, mouse hit-testing and the platform input handler all
//! live in the custom [`TextElement`] below; [`TextInput`] itself is the
//! state. Two things are added to the example: a theme-driven palette, and an
//! `on_submit` hook so Enter commits the edit.
//!
//! Copied from StorySearch's widget of the same name, unchanged but for this
//! note, so the two apps edit text the same way. A third, the wrapping mode,
//! lets a long value (the concept terms) break across lines and grow the field
//! downward instead of running past its edge: `prepaint` measures the width the
//! layout gives the field, computes the height the text needs there, and
//! `request_layout` reads that height back on the next frame. It never reports a
//! content width, so the text cannot push its column wider.

use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, Element, ElementId, ElementInputHandler,
    Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId, IntoElement, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render,
    SharedString, Style, TextAlign, TextRun, UTF16Selection, UnderlineStyle, Window, WrappedLine,
    actions, div, fill, point, prelude::*, px, relative, size,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::gui::theme;

actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
        Submit,
    ]
);

/// An editable field: one line of text, or several once wrapped.
pub struct TextInput {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<WrappedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    last_line_height: Pixels,
    is_selecting: bool,
    /// Wrap the text and grow the field downward instead of letting it run
    /// past the field's edge. Off by default: the search form is one line.
    wrap: bool,
    /// Draw without the field's own frame — no border, fill or padding — so it
    /// can sit inside a container that draws them (the concept tag box).
    bare: bool,
    /// For a wrapping field, the width it was last laid out at and the height
    /// its text needed there. `prepaint` measures the width and stores the
    /// height; `request_layout` reads that height back on the next frame. It is
    /// the one way to wrap without letting the text's intrinsic width push the
    /// field — and the column around it — wider than the space it is given.
    wrapped: Option<(Pixels, Pixels)>,
    on_submit: Option<Box<dyn Fn(&mut Window, &mut Context<TextInput>)>>,
    on_change: Option<Box<dyn Fn(&mut Context<TextInput>)>>,
}

impl TextInput {
    pub fn new(placeholder: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        TextInput {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: placeholder.into(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            last_line_height: px(0.),
            is_selecting: false,
            wrap: false,
            bare: false,
            wrapped: None,
            on_submit: None,
            on_change: None,
        }
    }

    /// Wrap the text and let the field grow downward to fit, rather than
    /// running past its edge. For a value that is a long list — the concept
    /// terms — this keeps every term visible and inside the field.
    pub fn wrap(mut self) -> Self {
        self.wrap = true;
        self
    }

    /// Draw without its own frame so the field can live inside a container
    /// that draws the border and fill — the concept tag box. The text, caret
    /// and selection are unchanged; only the chrome goes.
    pub fn bare(mut self) -> Self {
        self.bare = true;
        self
    }

    /// Called on Enter. Every field in the search form uses it, so return runs
    /// the search from wherever the caret is.
    pub fn on_submit(mut self, f: impl Fn(&mut Window, &mut Context<TextInput>) + 'static) -> Self {
        self.on_submit = Some(Box::new(f));
        self
    }

    /// Called whenever the text changes. The search form uses it so its
    /// controls that appear "only when they apply" re-evaluate as you type.
    pub fn on_change(mut self, f: impl Fn(&mut Context<TextInput>) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// The text as it stands. The search view reads it when a query runs.
    pub fn content(&self) -> String {
        self.content.to_string()
    }

    /// Replace the whole line (a preset, or a row being filled in). Does not
    /// fire `on_change`: callers set the content and then notify themselves.
    pub fn set_content(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.content = text.into();
        self.selected_range = self.content.len()..self.content.len();
        self.marked_range = None;
        cx.notify();
    }

    fn changed(&self, cx: &mut Context<Self>) {
        if let Some(f) = &self.on_change {
            f(cx);
        }
    }

    fn submit(&mut self, _: &Submit, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(f) = &self.on_submit {
            f(window, cx);
        }
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text.replace('\n', " "), window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        let local = point(position.x - bounds.left(), position.y - bounds.top());
        match line.index_for_position(local, self.last_line_height) {
            Ok(index) | Err(index) => index.min(self.content.len()),
        }
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;

        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }

        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;

        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }

        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }
}

impl EntityInputHandler for TextInput {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        self.changed(cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        self.changed(cx);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        let start = last_layout.position_for_index(range.start, self.last_line_height)?;
        let end = last_layout.position_for_index(range.end, self.last_line_height)?;
        Some(Bounds::from_corners(
            point(bounds.left() + start.x, bounds.top() + start.y),
            point(bounds.left() + end.x, bounds.top() + end.y),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        assert_eq!(last_layout.text, self.content);
        let utf8_index = match last_layout.index_for_position(line_point, self.last_line_height) {
            Ok(index) | Err(index) => index,
        };
        Some(self.offset_to_utf16(utf8_index))
    }
}

/// The painted input: the wrapped lines, a caret and selection rectangles.
struct TextElement {
    input: Entity<TextInput>,
}

struct PrepaintState {
    lines: Vec<WrappedLine>,
    line_height: Pixels,
    cursor: Option<PaintQuad>,
    selection: Vec<PaintQuad>,
}

/// Shape a field's text at the current style: the placeholder, or the content
/// with the marked (IME) range underlined. `wrap_width` wraps when known;
/// `None` keeps a single line, which is what an unwrapped field is.
fn shape_input(
    input: &Entity<TextInput>,
    wrap_width: Option<Pixels>,
    window: &mut Window,
    cx: &App,
) -> (Vec<WrappedLine>, Pixels) {
    let input = input.read(cx);
    let content = input.content.clone();
    let style = window.text_style();

    let (display_text, text_color) = if content.is_empty() {
        (input.placeholder.clone(), theme::muted().into())
    } else {
        (content, style.color)
    };

    let run = TextRun {
        len: display_text.len(),
        font: style.font(),
        color: text_color,
        background_color: None,
        underline: None,
        strikethrough: None,
    };
    let runs = if let Some(marked_range) = input.marked_range.as_ref() {
        vec![
            TextRun {
                len: marked_range.start,
                ..run.clone()
            },
            TextRun {
                len: marked_range.end - marked_range.start,
                underline: Some(UnderlineStyle {
                    color: Some(run.color),
                    thickness: px(1.0),
                    wavy: false,
                }),
                ..run.clone()
            },
            TextRun {
                len: display_text.len() - marked_range.end,
                ..run
            },
        ]
        .into_iter()
        .filter(|run| run.len > 0)
        .collect()
    } else {
        vec![run]
    };

    let font_size = style.font_size.to_pixels(window.rem_size());
    let lines: Vec<WrappedLine> = window
        .text_system()
        .shape_text(display_text, font_size, &runs, wrap_width, None)
        .map(|lines| lines.into_iter().collect())
        .unwrap_or_default();

    let line_height = window.line_height();
    (lines, line_height)
}

impl TextElement {
    fn new(input: Entity<TextInput>) -> Self {
        TextElement { input }
    }
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let input = self.input.read(cx);
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        // Never report a content width: the field takes the width it is given
        // (and wraps inside it), rather than letting a long value push itself
        // — and the column it sits in — wider.
        style.min_size.width = px(0.).into();
        style.size.height = if input.wrap {
            // The height is measured in `prepaint`, where the width actually
            // assigned by the layout is known, and read back here next frame.
            input
                .wrapped
                .map(|(_, height)| height)
                .unwrap_or_else(|| window.line_height())
        } else {
            window.line_height()
        }
        .into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let (lines, line_height) = if self.input.read(cx).wrap {
            // Re-shape at the width the layout actually gave us; then remember
            // the height that needs, so the next frame's `request_layout` can
            // grow (or shrink) the field to fit.
            let (lines, line_height) =
                shape_input(&self.input, Some(bounds.size.width), window, cx);
            let needed = lines
                .iter()
                .map(|line| line.size(line_height).height)
                .fold(px(0.), |total, height| total + height);
            let width = bounds.size.width;
            if self.input.read(cx).wrapped != Some((width, needed)) {
                self.input.update(cx, |input, cx| {
                    input.wrapped = Some((width, needed));
                    cx.notify();
                });
            }
            (lines, line_height)
        } else {
            shape_input(&self.input, None, window, cx)
        };

        let (cursor, selected_range) = {
            let input = self.input.read(cx);
            (input.cursor_offset(), input.selected_range.clone())
        };

        let line = lines.first();
        let cursor = line
            .and_then(|line| line.position_for_index(cursor, line_height))
            .map(|pos| {
                fill(
                    Bounds::new(
                        point(bounds.left() + pos.x, bounds.top() + pos.y),
                        size(px(2.), line_height),
                    ),
                    theme::accent(),
                )
            });

        let mut selection = Vec::new();
        if !selected_range.is_empty()
            && let Some(line) = line
            && let (Some(start), Some(end)) = (
                line.position_for_index(selected_range.start, line_height),
                line.position_for_index(selected_range.end, line_height),
            )
        {
            let first = (start.y / line_height) as usize;
            let last = (end.y / line_height) as usize;
            for ix in first..=last {
                let left = if ix == first { start.x } else { px(0.) };
                let right = if ix == last { end.x } else { bounds.size.width };
                let width = if right > left { right - left } else { px(0.) };
                selection.push(fill(
                    Bounds::new(
                        point(bounds.left() + left, bounds.top() + line_height * ix as f32),
                        size(width, line_height),
                    ),
                    theme::accent_tint(0.20),
                ));
            }
        }

        PrepaintState {
            lines,
            line_height,
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        for quad in prepaint.selection.drain(..) {
            window.paint_quad(quad);
        }
        let mut origin = bounds.origin;
        for line in &prepaint.lines {
            let _ = line.paint(
                origin,
                prepaint.line_height,
                TextAlign::Left,
                Some(bounds),
                window,
                cx,
            );
            origin.y += line.size(prepaint.line_height).height;
        }

        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = prepaint.lines.first().cloned();
            input.last_bounds = Some(bounds);
            input.last_line_height = prepaint.line_height;
        });
    }
}

impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .key_context("TextInput")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::show_character_palette))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_action(cx.listener(Self::submit))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .w_full()
            .child(
                div()
                    .when(!self.bare, |d| {
                        d.px_3()
                            .py_2()
                            .rounded_md()
                            .border_1()
                            .border_color(theme::border())
                            .bg(theme::panel())
                            .when(!self.wrap, |d| d.h(px(34.)))
                            .when(self.wrap, |d| d.min_h(px(34.)))
                    })
                    .when(self.bare, |d| d.min_h(px(18.)))
                    .w_full()
                    .min_w(px(0.))
                    .text_color(theme::fg())
                    .line_height(px(18.))
                    .text_size(px(14.))
                    .child(TextElement::new(cx.entity())),
            )
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
