//! The one window: a toolbar, the program listing, the machine's state beside
//! it, and a status line.
//!
//! The listing is the old app's ROM table — a breakpoint gutter, the address
//! or its label, the instruction, and a note — and is edited the same way:
//! double-click a cell and type. The side panel replaces the register and RAM
//! tables, and adds the pins, which the old app never showed. A value that has
//! changed since the machine last stopped is drawn red, as it was there.
//!
//! The machine lives behind a mutex so a run can go on a thread of its own:
//! the window reads a copy of it each frame and never holds the lock across a
//! render.

use std::ops::Range;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gpui::prelude::*;
use gpui::{
    AnyElement, ClickEvent, Context, Entity, FocusHandle, Focusable, MouseButton, Render,
    ScrollStrategy, SharedString, UniformListScrollHandle, Window, div, px, uniform_list,
};

use crate::gui::theme;
use crate::gui::widgets::text_input::TextInput;
use crate::isa;
use crate::pic::{self, Pic, Stop};
use crate::project::{self, Annotations};

const ROW: f32 = 20.;
const MONO: &str = "Menlo";
/// How often the window repaints while a run is going.
const FRAME: Duration = Duration::from_millis(33);
/// Cycles a running thread executes per lock: small enough that the window's
/// frame never waits long for the machine, large enough to amortise the lock.
const CHUNK_FAST: u64 = 400_000;
/// In real time the chunk is a millisecond of the part's time.
const CHUNK_REAL: u64 = 1_000;

/// A cell being edited, and what Enter commits it to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Edit {
    Instruction(u16),
    Label(u16),
    Note(u16),
    RamValue(u8),
    RamName(u8),
    Analog(usize),
    W,
    Pc,
}

/// A run on its own thread.
struct Running {
    stop: Arc<AtomicBool>,
    done: Arc<Mutex<Option<Stop>>>,
    started: Instant,
    start_cycles: u64,
}

pub struct Simulator {
    pic: Arc<Mutex<Pic>>,
    /// The copy this frame draws from.
    shown: Pic,
    /// The machine as it was when it last started moving — what "changed" is
    /// measured against.
    before: Pic,
    ann: Annotations,
    path: Option<PathBuf>,
    running: Option<Running>,
    real_time: bool,
    /// Instructions per second of the last run, for the status line.
    speed: Option<f64>,
    status: SharedString,
    selected: Option<u16>,
    editing: Option<Edit>,
    editor: Entity<TextInput>,
    program_scroll: UniformListScrollHandle,
    ram_scroll: UniformListScrollHandle,
    /// The RAM panel's rows: every SFR in address order, then the GPRs.
    ram_rows: Vec<u8>,
    focus: FocusHandle,
}

impl Simulator {
    pub fn new(file: Option<PathBuf>, cx: &mut Context<Self>) -> Self {
        let editor = {
            let this = cx.entity().downgrade();
            cx.new(move |cx| {
                TextInput::new("", cx).bare().on_submit(move |_window, cx| {
                    let view = this.clone();
                    cx.defer(move |cx| {
                        view.update(cx, |view, cx| view.commit_edit(cx)).ok();
                    });
                })
            })
        };
        let mut ram_rows: Vec<u8> = pic::SFRS.iter().map(|(a, _)| *a).collect();
        ram_rows.sort_unstable();
        ram_rows.extend(0x20..=0x5F);

        let mut view = Simulator {
            pic: Arc::new(Mutex::new(Pic::new())),
            shown: Pic::new(),
            before: Pic::new(),
            ann: Annotations::default(),
            path: None,
            running: None,
            real_time: true,
            speed: None,
            status: "".into(),
            selected: None,
            editing: None,
            editor,
            program_scroll: UniformListScrollHandle::new(),
            ram_scroll: UniformListScrollHandle::new(),
            ram_rows,
            focus: cx.focus_handle(),
        };
        match file {
            Some(path) => view.open_path(path, cx),
            None => view.load_demo(cx),
        }
        view
    }

    fn machine(&self) -> std::sync::MutexGuard<'_, Pic> {
        self.pic.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Take a fresh copy of the machine for drawing.
    fn refresh(&mut self) {
        let copy = self.machine().clone();
        self.shown = copy;
    }

    fn set_status(&mut self, text: impl Into<SharedString>) {
        self.status = text.into();
    }

    fn follow_pc(&mut self) {
        let pc = self.shown.pc as usize % pic::ROM_WORDS;
        self.program_scroll
            .scroll_to_item(pc, ScrollStrategy::Center);
    }

    // ---- files ---------------------------------------------------------

    pub fn load_demo(&mut self, cx: &mut Context<Self>) {
        self.halt(cx);
        let mut pic = Pic::new();
        match project::assemble_source(project::DEMO, &mut pic) {
            Ok(ann) => {
                pic.reset();
                self.ann = ann;
                *self.machine() = pic;
                self.path = None;
                self.set_status("Demo: GP0 toggles on every TMR0 overflow. Press Run.");
            }
            Err(e) => self.set_status(format!("demo failed to assemble: {e}")),
        }
        self.refresh();
        self.before = self.shown.clone();
        self.follow_pc();
        cx.notify();
    }

    pub fn open(&mut self, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Open a .hex or .asm program".into()),
        });
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = paths.await
                && let Some(path) = paths.into_iter().next()
            {
                this.update(cx, |view, cx| view.open_path(path, cx)).ok();
            }
        })
        .detach();
    }

    pub fn open_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        self.halt(cx);
        let mut pic = Pic::new();
        match project::open(&path, &mut pic) {
            Ok(ann) => {
                self.ann = ann;
                *self.machine() = pic;
                self.set_status(format!("Opened {}", path.display()));
                self.path = Some(path);
            }
            Err(e) => self.set_status(format!("Could not open {}: {e:#}", path.display())),
        }
        self.refresh();
        self.before = self.shown.clone();
        self.follow_pc();
        cx.notify();
    }

    pub fn save(&mut self, cx: &mut Context<Self>) {
        match self.path.clone() {
            Some(path) => self.save_to(path, cx),
            None => self.save_as(cx),
        }
    }

    pub fn save_as(&mut self, cx: &mut Context<Self>) {
        let dir = self
            .path
            .as_ref()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("."));
        let name = self
            .path
            .as_ref()
            .and_then(|p| p.file_stem())
            .map(|s| format!("{}.hex", s.to_string_lossy()))
            .unwrap_or_else(|| "program.hex".into());
        let path = cx.prompt_for_new_path(&dir, Some(&name));
        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(path))) = path.await {
                this.update(cx, |view, cx| view.save_to(path, cx)).ok();
            }
        })
        .detach();
    }

    fn save_to(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let pic = self.machine().clone();
        match project::save(&path, &pic, &self.ann) {
            Ok(written) => {
                self.set_status(format!(
                    "Saved {} (labels and notes in {})",
                    written.display(),
                    project::sidecar(&written)
                        .file_name()
                        .map(|n| n.to_string_lossy().into_owned())
                        .unwrap_or_default()
                ));
                self.path = Some(written);
            }
            Err(e) => self.set_status(format!("Could not save: {e:#}")),
        }
        cx.notify();
    }

    // ---- running -------------------------------------------------------

    pub fn reset(&mut self, cx: &mut Context<Self>) {
        self.halt(cx);
        self.machine().reset();
        self.refresh();
        self.before = self.shown.clone();
        self.speed = None;
        self.set_status("Reset");
        self.follow_pc();
        cx.notify();
    }

    pub fn step(&mut self, cx: &mut Context<Self>) {
        if self.running.is_some() {
            return;
        }
        self.before = self.shown.clone();
        let cycles = self.machine().step();
        self.refresh();
        self.set_status(format!(
            "Stepped: {cycles} cycle{}",
            if cycles == 1 { "" } else { "s" }
        ));
        self.follow_pc();
        cx.notify();
    }

    pub fn run(&mut self, cx: &mut Context<Self>) {
        if self.running.is_some() {
            return;
        }
        self.cancel_edit(cx);
        self.before = self.shown.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let done = Arc::new(Mutex::new(None));
        let start_cycles = self.shown.cycles;
        {
            let pic = self.pic.clone();
            let stop = stop.clone();
            let done = done.clone();
            let real_time = self.real_time;
            std::thread::spawn(move || {
                let began = Instant::now();
                let chunk = if real_time { CHUNK_REAL } else { CHUNK_FAST };
                let reason = loop {
                    let (reason, cycles) = {
                        let mut pic = pic.lock().unwrap_or_else(|e| e.into_inner());
                        let r = pic.run(chunk, &|| stop.load(Ordering::Relaxed));
                        (r, pic.cycles)
                    };
                    if reason != Stop::Budget {
                        break reason;
                    }
                    if stop.load(Ordering::Relaxed) {
                        break Stop::Halted;
                    }
                    if real_time {
                        let due = Duration::from_secs_f64(
                            (cycles - start_cycles) as f64 / pic::CYCLES_PER_SECOND as f64,
                        );
                        if let Some(wait) = due.checked_sub(began.elapsed()) {
                            std::thread::sleep(wait);
                        }
                    }
                };
                *done.lock().unwrap_or_else(|e| e.into_inner()) = Some(reason);
            });
        }
        self.running = Some(Running {
            stop,
            done,
            started: Instant::now(),
            start_cycles,
        });
        self.set_status("Running…");

        // The frame clock: repaint while the thread runs, and notice when it
        // stops on its own (a breakpoint).
        let background = cx.background_executor().clone();
        cx.spawn(async move |this, cx| {
            loop {
                background.timer(FRAME).await;
                let going = this.update(cx, |view, cx| view.poll(cx)).unwrap_or(false);
                if !going {
                    break;
                }
            }
        })
        .detach();
        cx.notify();
    }

    /// One frame of a run: refresh the copy, and wind up if the thread is done.
    /// Returns whether the run is still going.
    fn poll(&mut self, cx: &mut Context<Self>) -> bool {
        let Some(run) = &self.running else {
            return false;
        };
        let finished = *run.done.lock().unwrap_or_else(|e| e.into_inner());
        let elapsed = run.started.elapsed().as_secs_f64();
        let start_cycles = run.start_cycles;
        self.refresh();
        if elapsed > 0.2 {
            self.speed = Some((self.shown.cycles - start_cycles) as f64 / elapsed);
        }
        if let Some(reason) = finished {
            self.running = None;
            self.stopped(reason);
        } else if self.real_time {
            self.follow_pc();
        }
        cx.notify();
        finished.is_none()
    }

    pub fn halt(&mut self, cx: &mut Context<Self>) {
        let Some(run) = self.running.take() else {
            return;
        };
        run.stop.store(true, Ordering::Relaxed);
        // The thread lets go of the lock within one chunk; taking it here
        // waits for that, so the copy below is of a machine at rest.
        drop(self.machine());
        self.refresh();
        self.stopped(Stop::Halted);
        cx.notify();
    }

    fn stopped(&mut self, reason: Stop) {
        self.set_status(match reason {
            Stop::Breakpoint(at) => format!("Stopped at breakpoint 0x{at:03X}"),
            Stop::Halted => "Halted".into(),
            Stop::Budget => "Stopped".into(),
        });
        self.follow_pc();
    }

    pub fn toggle_run(&mut self, cx: &mut Context<Self>) {
        if self.running.is_some() {
            self.halt(cx);
        } else {
            self.run(cx);
        }
    }

    pub fn toggle_real_time(&mut self, cx: &mut Context<Self>) {
        let was_running = self.running.is_some();
        self.halt(cx);
        self.real_time = !self.real_time;
        if was_running {
            self.run(cx);
        }
        cx.notify();
    }

    pub fn toggle_breakpoint_at(&mut self, addr: u16, cx: &mut Context<Self>) {
        {
            let mut pic = self.machine();
            let slot = &mut pic.breakpoints[addr as usize % pic::ROM_WORDS];
            *slot = !*slot;
        }
        self.refresh();
        cx.notify();
    }

    pub fn toggle_breakpoint(&mut self, cx: &mut Context<Self>) {
        let addr = self.selected.unwrap_or(self.shown.pc);
        self.toggle_breakpoint_at(addr, cx);
    }

    pub fn clear_breakpoints(&mut self, cx: &mut Context<Self>) {
        self.machine().breakpoints = vec![false; pic::ROM_WORDS];
        self.refresh();
        self.set_status("Cleared all breakpoints");
        cx.notify();
    }

    fn drive_pin(&mut self, pin: u8, cx: &mut Context<Self>) {
        {
            let mut pic = self.machine();
            let level = pic.pin_level(pin);
            pic.drive(pin, !level);
        }
        self.refresh();
        cx.notify();
    }

    // ---- editing -------------------------------------------------------

    fn start_edit(&mut self, edit: Edit, window: &mut Window, cx: &mut Context<Self>) {
        let pic = &self.shown;
        let text = match edit {
            Edit::Instruction(a) => {
                let banks = isa::banks(&pic.rom);
                let (m, ops) =
                    isa::disassemble(pic.rom[a as usize], &self.ann.names, banks[a as usize]);
                format!("{} {ops}", m.to_ascii_lowercase())
                    .trim()
                    .to_string()
            }
            Edit::Label(a) => self.ann.names.label(a).unwrap_or("").to_string(),
            Edit::Note(a) => self.ann.notes.get(&a).cloned().unwrap_or_default(),
            Edit::RamValue(a) => format!("0x{:02X}", pic.peek(a)),
            Edit::RamName(a) => self
                .ann
                .names
                .registers
                .get(&a)
                .cloned()
                .unwrap_or_default(),
            Edit::Analog(ch) => pic.analog[ch].to_string(),
            Edit::W => format!("0x{:02X}", pic.w),
            Edit::Pc => format!("0x{:03X}", pic.pc),
        };
        self.editing = Some(edit);
        self.editor
            .update(cx, |input, cx| input.set_content(text, cx));
        let handle = self.editor.focus_handle(cx);
        window.focus(&handle, cx);
        cx.notify();
    }

    pub fn cancel_edit(&mut self, cx: &mut Context<Self>) {
        if self.editing.take().is_some() {
            cx.notify();
        }
    }

    fn commit_edit(&mut self, cx: &mut Context<Self>) {
        let Some(edit) = self.editing.take() else {
            return;
        };
        let text = self.editor.read(cx).content().trim().to_string();
        let number = |max: u32| -> Result<u32, String> {
            match isa::number(&text) {
                Some(v) if v <= max => Ok(v),
                _ => Err(format!("'{text}' is not a number up to {max}")),
            }
        };
        let result: Result<(), String> = match edit {
            Edit::Instruction(a) => {
                if text.is_empty() {
                    Ok(())
                } else {
                    isa::assemble(&text, &self.ann.names).map(|w| {
                        self.machine().rom[a as usize] = w;
                    })
                }
            }
            Edit::Label(a) => {
                if text.is_empty() {
                    self.ann.names.labels.remove(&a);
                } else {
                    self.ann.names.labels.insert(a, text.clone());
                }
                Ok(())
            }
            Edit::Note(a) => {
                if text.is_empty() {
                    self.ann.notes.remove(&a);
                } else {
                    self.ann.notes.insert(a, text.clone());
                }
                Ok(())
            }
            Edit::RamValue(a) => number(0xFF).map(|v| self.machine().poke(a, v as u8)),
            Edit::RamName(a) => {
                if text.is_empty() {
                    self.ann.names.registers.remove(&(a & 0x7F));
                } else {
                    self.ann.names.registers.insert(a & 0x7F, text.clone());
                }
                Ok(())
            }
            Edit::Analog(ch) => number(1023).map(|v| self.machine().analog[ch] = v as u16),
            Edit::W => number(0xFF).map(|v| self.machine().w = v as u8),
            Edit::Pc => number(pic::ROM_WORDS as u32 - 1).map(|v| self.machine().pc = v as u16),
        };
        match result {
            Ok(()) => {
                if matches!(edit, Edit::Instruction(_)) {
                    self.set_status("");
                }
            }
            Err(e) => self.set_status(e),
        }
        self.refresh();
        window_focus_root(self, cx);
        cx.notify();
    }

    // ---- drawing -------------------------------------------------------

    fn editor_cell(&self) -> AnyElement {
        div()
            .flex_1()
            .min_w(px(0.))
            .px_1()
            .rounded_sm()
            .border_1()
            .border_color(theme::accent())
            .bg(theme::bg())
            .child(self.editor.clone())
            .into_any_element()
    }

    fn header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let running = self.running.is_some();
        div()
            .id("header")
            .flex()
            .flex_none()
            .items_center()
            .gap_2()
            .h(px(42.))
            .pl(px(84.))
            .pr_3()
            .border_b_1()
            .border_color(theme::border())
            .bg(theme::panel())
            .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
            .child(
                div()
                    .font_weight(gpui::FontWeight::SEMIBOLD)
                    .child("CocoaPic"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme::muted())
                    .child("PIC12F675"),
            )
            .child(div().w(px(12.)))
            .child(button(
                "reset",
                "Reset",
                !running,
                cx.listener(|v, _, _, cx| v.reset(cx)),
            ))
            .child(button(
                "step",
                "Step",
                !running,
                cx.listener(|v, _, _, cx| v.step(cx)),
            ))
            .child(button(
                "run",
                if running { "Halt" } else { "Run" },
                true,
                cx.listener(|v, _, _, cx| v.toggle_run(cx)),
            ))
            .child(toggle(
                "realtime",
                "Real time",
                self.real_time,
                cx.listener(|v, _, _, cx| v.toggle_real_time(cx)),
            ))
            .child(div().flex_1())
            .child(button(
                "open",
                "Open…",
                true,
                cx.listener(|v, _, _, cx| v.open(cx)),
            ))
            .child(button(
                "save",
                "Save",
                true,
                cx.listener(|v, _, _, cx| v.save(cx)),
            ))
    }

    fn program(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let heading = div()
            .flex()
            .flex_none()
            .h(px(24.))
            .items_center()
            .px_2()
            .gap_2()
            .text_xs()
            .text_color(theme::muted())
            .border_b_1()
            .border_color(theme::border())
            .child(div().w(px(18.)).child("●"))
            .child(div().w(px(40.)).child("addr"))
            .child(div().w(px(96.)).child("label"))
            .child(div().w(px(240.)).child("instruction"))
            .child(div().flex_1().child("notes"));
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w(px(0.))
            .h_full()
            .child(heading)
            .child(
                uniform_list(
                    "program",
                    pic::ROM_WORDS,
                    cx.processor(|this, range: Range<usize>, _window, cx| {
                        let banks = isa::banks(&this.shown.rom);
                        range
                            .map(|i| this.program_row(i, banks[i], cx))
                            .collect::<Vec<_>>()
                    }),
                )
                .track_scroll(&self.program_scroll)
                .flex_1(),
            )
    }

    fn program_row(&self, i: usize, bank1: bool, cx: &mut Context<Self>) -> AnyElement {
        let addr = i as u16;
        let pic = &self.shown;
        let word = pic.rom[i];
        let at_pc = pic.pc as usize % pic::ROM_WORDS == i;
        let breakpoint = pic.breakpoints[i];
        let (mnem, ops) = isa::disassemble(word, &self.ann.names, bank1);
        let erased = word == pic::ERASED;
        let label = self.ann.names.label(addr).map(str::to_string);
        let note = self.ann.notes.get(&addr).cloned().unwrap_or_default();
        let edit = self.editing;

        let bg = if at_pc {
            theme::accent_tint(0.22)
        } else if self.selected == Some(addr) {
            theme::tab_active()
        } else if i % 2 == 1 {
            theme::stripe()
        } else {
            theme::bg()
        };
        let text = if at_pc {
            theme::fg()
        } else if erased {
            theme::faint()
        } else {
            theme::fg()
        };

        let gutter = div()
            .id(("bp", i))
            .w(px(18.))
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .child(
                div()
                    .size(px(10.))
                    .rounded_full()
                    .when(breakpoint, |d| d.bg(theme::breakpoint()))
                    .when(!breakpoint, |d| d.hover(|s| s.bg(theme::faint()))),
            )
            .on_click(
                cx.listener(move |v, _: &ClickEvent, _, cx| v.toggle_breakpoint_at(addr, cx)),
            );

        let cell = |id: &'static str, width: Option<f32>, which: Edit, content: AnyElement| {
            let base = div()
                .id((id, i))
                .h_full()
                .flex()
                .items_center()
                .overflow_hidden();
            let base = match width {
                Some(w) => base.w(px(w)).flex_none(),
                None => base.flex_1().min_w(px(0.)),
            };
            base.child(content)
                .on_click(cx.listener(move |v, e: &ClickEvent, window, cx| {
                    v.selected = Some(addr);
                    if e.click_count() >= 2 {
                        v.start_edit(which, window, cx);
                    }
                    cx.notify();
                }))
        };

        let label_el = if edit == Some(Edit::Label(addr)) {
            self.editor_cell()
        } else {
            div()
                .text_color(theme::accent())
                .child(label.unwrap_or_default())
                .into_any_element()
        };
        let instr_el = if edit == Some(Edit::Instruction(addr)) {
            self.editor_cell()
        } else {
            div()
                .flex()
                .gap_2()
                .font_family(MONO)
                .text_color(text)
                .child(
                    div()
                        .w(px(64.))
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .child(mnem),
                )
                .child(ops)
                .into_any_element()
        };
        let note_el = if edit == Some(Edit::Note(addr)) {
            self.editor_cell()
        } else {
            div()
                .text_color(theme::muted())
                .truncate()
                .child(note)
                .into_any_element()
        };

        div()
            .id(("row", i))
            .flex()
            .items_center()
            .h(px(ROW))
            .px_2()
            .gap_2()
            .bg(bg)
            .text_size(px(12.))
            .child(gutter)
            .child(
                div()
                    .w(px(40.))
                    .flex_none()
                    .font_family(MONO)
                    .text_color(if at_pc {
                        theme::accent()
                    } else {
                        theme::faint()
                    })
                    .child(format!("{i:03X}")),
            )
            .child(cell("label", Some(96.), Edit::Label(addr), label_el))
            .child(cell("instr", Some(240.), Edit::Instruction(addr), instr_el))
            .child(cell("note", None, Edit::Note(addr), note_el))
            .into_any_element()
    }

    fn side(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_none()
            .w(px(380.))
            .h_full()
            .border_l_1()
            .border_color(theme::border())
            .bg(theme::panel())
            .child(self.core_panel(cx))
            .child(self.pins_panel(cx))
            .child(self.ram_panel(cx))
    }

    fn core_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pic = &self.shown;
        let before = &self.before;
        let status = pic.status();
        let flags = ["C", "DC", "Z", "PD", "TO", "RP0", "RP1", "IRP"];
        let flag_row = div().flex().gap_1().children((0..8).rev().map(|b| {
            let on = status & (1 << b) != 0;
            let changed = (status ^ before.status()) & (1 << b) != 0;
            div()
                .px_1()
                .rounded_sm()
                .text_size(px(11.))
                .font_family(MONO)
                .when(on, |d| d.bg(theme::tab_active()).text_color(theme::fg()))
                .when(!on, |d| d.text_color(theme::faint()))
                .when(changed, |d| d.text_color(theme::changed()))
                .child(flags[b])
        }));

        let stack = pic.stack_view();
        let stack_text = if stack.is_empty() {
            "empty".to_string()
        } else {
            stack
                .iter()
                .map(|a| format!("{a:03X}"))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let secs = pic.cycles as f64 / pic::CYCLES_PER_SECOND as f64;

        let w_cell = if self.editing == Some(Edit::W) {
            self.editor_cell()
        } else {
            mono(format!("0x{:02X}", pic.w), pic.w != before.w).into_any_element()
        };
        let pc_cell = if self.editing == Some(Edit::Pc) {
            self.editor_cell()
        } else {
            mono(format!("0x{:03X}", pic.pc), false).into_any_element()
        };

        panel("Core")
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(field("W", w_cell).id("w").on_click(cx.listener(
                        |v, e: &ClickEvent, window, cx| {
                            if e.click_count() >= 2 {
                                v.start_edit(Edit::W, window, cx);
                            }
                        },
                    )))
                    .child(field("PC", pc_cell).id("pc").on_click(cx.listener(
                        |v, e: &ClickEvent, window, cx| {
                            if e.click_count() >= 2 {
                                v.start_edit(Edit::Pc, window, cx);
                            }
                        },
                    )))
                    .child(field(
                        "TMR0",
                        mono(
                            format!("{:3}", pic.peek(pic::TMR0)),
                            pic.peek(pic::TMR0) != before.peek(pic::TMR0),
                        )
                        .into_any_element(),
                    ))
                    .child(field(
                        "WDT",
                        mono(
                            if pic.config.wdt() {
                                format!("{:3.0}%", pic.wdt_fraction() * 100.)
                            } else {
                                "off".into()
                            },
                            false,
                        )
                        .into_any_element(),
                    )),
            )
            .child(field("STATUS", flag_row.into_any_element()))
            .child(field(
                "Stack",
                mono(stack_text.to_string(), false).into_any_element(),
            ))
            .child(div().text_xs().text_color(theme::muted()).child(format!(
                "{} cycles · {:.6} s at 4 MHz{}",
                group_digits(pic.cycles),
                secs,
                if pic.sleeping { " · asleep" } else { "" }
            )))
    }

    fn pins_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pic = &self.shown;
        let analog = pic.analog_pins();
        let pins = div().flex().gap_2().children((0..6u8).map(|pin| {
            let out = pic.is_output(pin);
            let level = pic.pin_level(pin);
            let is_analog = analog & (1 << pin) != 0;
            let kind = if out {
                "out"
            } else if is_analog {
                "analog"
            } else {
                "in"
            };
            div()
                .id(("pin", pin as usize))
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .w(px(52.))
                .py_1()
                .rounded_md()
                .border_1()
                .border_color(theme::border())
                .bg(theme::bg())
                .when(!out, |d| d.cursor_pointer().hover(|s| s.bg(theme::hover())))
                .child(
                    div()
                        .text_xs()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .child(format!("GP{pin}")),
                )
                .child(
                    div()
                        .size(px(14.))
                        .rounded_full()
                        .border_1()
                        .border_color(theme::border())
                        .when(level, |d| d.bg(theme::high()))
                        .when(!level, |d| d.bg(theme::panel())),
                )
                .child(
                    div()
                        .text_size(px(10.))
                        .text_color(theme::muted())
                        .child(kind),
                )
                .when(!out, |d| {
                    d.on_click(cx.listener(move |v, _: &ClickEvent, _, cx| v.drive_pin(pin, cx)))
                })
        }));

        let channels = div().flex().gap_3().children((0..4usize).map(|ch| {
            let cell = if self.editing == Some(Edit::Analog(ch)) {
                self.editor_cell()
            } else {
                mono(format!("{}", pic.analog[ch]), false).into_any_element()
            };
            field(&format!("AN{ch}"), cell)
                .id(("an", ch))
                .on_click(cx.listener(move |v, e: &ClickEvent, window, cx| {
                    if e.click_count() >= 2 {
                        v.start_edit(Edit::Analog(ch), window, cx);
                    }
                }))
        }));

        panel("Pins")
            .child(pins)
            .child(
                div()
                    .text_xs()
                    .text_color(theme::muted())
                    .child("Click an input to drive it high or low. Double-click an AN value (0–1023) to set it."),
            )
            .child(channels)
    }

    fn ram_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self.ram_rows.len();
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h(px(0.))
            .border_t_1()
            .border_color(theme::border())
            .child(
                div()
                    .flex()
                    .flex_none()
                    .h(px(24.))
                    .items_center()
                    .px_3()
                    .gap_2()
                    .text_xs()
                    .text_color(theme::muted())
                    .child(div().w(px(32.)).child("addr"))
                    .child(div().w(px(96.)).child("register"))
                    .child(div().w(px(40.)).child("hex"))
                    .child(div().w(px(32.)).child("dec"))
                    .child(div().flex_1().child("binary")),
            )
            .child(
                uniform_list(
                    "ram",
                    rows,
                    cx.processor(|this, range: Range<usize>, _window, cx| {
                        range.map(|i| this.ram_row(i, cx)).collect::<Vec<_>>()
                    }),
                )
                .track_scroll(&self.ram_scroll)
                .flex_1()
                .bg(theme::bg()),
            )
    }

    fn ram_row(&self, i: usize, cx: &mut Context<Self>) -> AnyElement {
        let addr = self.ram_rows[i];
        let pic = &self.shown;
        let v = pic.peek(addr);
        let changed = v != self.before.peek(addr);
        let gpr = addr >= 0x20;
        let name = if gpr {
            self.ann
                .names
                .registers
                .get(&addr)
                .cloned()
                .unwrap_or_default()
        } else {
            pic::sfr_name(addr).unwrap_or("").to_string()
        };
        let value_color = if changed {
            theme::changed()
        } else {
            theme::fg()
        };

        let name_el = if self.editing == Some(Edit::RamName(addr)) {
            self.editor_cell()
        } else {
            div()
                .text_color(if gpr { theme::accent() } else { theme::fg() })
                .child(name)
                .into_any_element()
        };
        let value_el = if self.editing == Some(Edit::RamValue(addr)) {
            self.editor_cell()
        } else {
            div()
                .font_family(MONO)
                .text_color(value_color)
                .child(format!("{v:02X}"))
                .into_any_element()
        };

        div()
            .id(("ram", i))
            .flex()
            .items_center()
            .h(px(ROW))
            .px_3()
            .gap_2()
            .text_size(px(12.))
            .when(i % 2 == 1, |d| d.bg(theme::stripe()))
            .child(
                div()
                    .w(px(32.))
                    .font_family(MONO)
                    .text_color(theme::faint())
                    .child(format!("{addr:02X}")),
            )
            .child(
                div()
                    .id(("ram-name", i))
                    .w(px(96.))
                    .flex()
                    .child(name_el)
                    .when(gpr, |d| {
                        d.on_click(cx.listener(move |v, e: &ClickEvent, window, cx| {
                            if e.click_count() >= 2 {
                                v.start_edit(Edit::RamName(addr), window, cx);
                            }
                        }))
                    }),
            )
            .child(
                div()
                    .id(("ram-value", i))
                    .w(px(40.))
                    .flex()
                    .child(value_el)
                    .on_click(cx.listener(move |v, e: &ClickEvent, window, cx| {
                        if e.click_count() >= 2 {
                            v.start_edit(Edit::RamValue(addr), window, cx);
                        }
                    })),
            )
            .child(
                div()
                    .w(px(32.))
                    .font_family(MONO)
                    .text_color(value_color)
                    .child(format!("{v}")),
            )
            .child(
                div()
                    .flex_1()
                    .font_family(MONO)
                    .text_color(if changed {
                        theme::changed()
                    } else {
                        theme::muted()
                    })
                    .child(format!("{:04b} {:04b}", v >> 4, v & 0x0F)),
            )
            .into_any_element()
    }

    fn status_bar(&self) -> impl IntoElement {
        let pic = &self.shown;
        let speed = match self.speed {
            Some(ips) => format!(" · {:.1} MIPS", ips / 1e6),
            None => String::new(),
        };
        let file = self
            .path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| "demo".into());
        div()
            .flex()
            .flex_none()
            .items_center()
            .justify_between()
            .gap_4()
            .h(px(26.))
            .px_3()
            .border_t_1()
            .border_color(theme::border())
            .bg(theme::panel())
            .text_xs()
            .text_color(theme::muted())
            .child(
                div()
                    .truncate()
                    .flex_1()
                    .min_w(px(0.))
                    .child(self.status.clone()),
            )
            .child(div().flex_none().child(format!(
                "{file} · PC 0x{:03X} · {}{}",
                pic.pc,
                if self.running.is_some() {
                    if self.real_time {
                        "running (real time)"
                    } else {
                        "running (full speed)"
                    }
                } else {
                    "stopped"
                },
                speed
            )))
    }
}

/// After an edit, hand focus back to the window so the menu's keys work.
fn window_focus_root(view: &Simulator, cx: &mut Context<Simulator>) {
    let handle = view.focus.clone();
    cx.defer(move |cx| {
        if let Some(window) = cx.active_window() {
            window
                .update(cx, |_, window, cx| window.focus(&handle, cx))
                .ok();
        }
    });
}

impl Simulator {
    pub fn focus_handle_of(&self) -> FocusHandle {
        self.focus.clone()
    }
}

impl Focusable for Simulator {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for Simulator {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.running.is_none() {
            self.refresh();
        }
        div()
            .key_context("Simulator")
            .track_focus(&self.focus)
            .flex()
            .flex_col()
            .size_full()
            .bg(theme::bg())
            .text_color(theme::fg())
            .text_size(px(13.))
            .child(self.header(cx))
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h(px(0.))
                    .child(self.program(cx))
                    .child(self.side(cx)),
            )
            .child(self.status_bar())
    }
}

// ---- small pieces ------------------------------------------------------

fn button(
    id: &'static str,
    label: &'static str,
    enabled: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> AnyElement {
    let base = div()
        .id(id)
        .px_3()
        .py_1()
        .rounded_md()
        .border_1()
        .border_color(theme::border())
        .bg(theme::bg())
        .text_sm()
        .child(label);
    if !enabled {
        return base.text_color(theme::faint()).into_any_element();
    }
    base.cursor_pointer()
        .hover(|s| s.bg(theme::hover()))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(on_click)
        .into_any_element()
}

fn toggle(
    id: &'static str,
    label: &'static str,
    on: bool,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut gpui::App) + 'static,
) -> AnyElement {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap_1()
        .px_2()
        .py_1()
        .rounded_md()
        .text_sm()
        .cursor_pointer()
        .hover(|s| s.bg(theme::hover()))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .child(
            div()
                .size(px(12.))
                .rounded_sm()
                .border_1()
                .border_color(if on { theme::accent() } else { theme::faint() })
                .when(on, |d| d.bg(theme::accent())),
        )
        .child(label)
        .on_click(on_click)
        .into_any_element()
}

fn panel(title: &'static str) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .flex_none()
        .gap_2()
        .p_3()
        .border_b_1()
        .border_color(theme::border())
        .child(
            div()
                .text_xs()
                .font_weight(gpui::FontWeight::SEMIBOLD)
                .text_color(theme::muted())
                .child(title.to_uppercase()),
        )
}

fn field(label: &str, value: AnyElement) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .gap(px(2.))
        .child(
            div()
                .text_size(px(10.))
                .text_color(theme::muted())
                .child(label.to_string()),
        )
        .child(div().flex().min_h(px(18.)).child(value))
}

fn mono(text: String, changed: bool) -> gpui::Div {
    div()
        .font_family(MONO)
        .text_size(px(13.))
        .text_color(if changed {
            theme::changed()
        } else {
            theme::fg()
        })
        .child(text)
}

fn group_digits(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{point, size};

    fn draw(view: &Entity<Simulator>, cx: &mut gpui::VisualTestContext) {
        cx.draw(point(px(0.), px(0.)), size(px(1240.), px(820.)), |_, _| {
            view.clone().into_any_element()
        });
    }

    #[gpui::test]
    async fn the_window_draws_steps_and_edits(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_window, cx| Simulator::new(None, cx));
        cx.run_until_parked();
        draw(&view, cx);

        // Step through the demo's set-up: TRISIO ends up with GP0 an output.
        view.update(cx, |v, cx| {
            for _ in 0..8 {
                v.step(cx);
            }
            assert_eq!(v.shown.pc, 8);
            assert!(v.shown.is_output(0));
        });
        draw(&view, cx);

        // Edit an instruction in place, the way a double-click then Enter does.
        view.update_in(cx, |v, window, cx| {
            v.start_edit(Edit::Instruction(0x20), window, cx);
            v.editor.update(cx, |e, cx| e.set_content("goto loop", cx));
            v.commit_edit(cx);
            assert_eq!(v.machine().rom[0x20], 0x280B);
        });
        draw(&view, cx);

        // A bad instruction leaves the word alone and says why.
        view.update_in(cx, |v, window, cx| {
            v.start_edit(Edit::Instruction(0x20), window, cx);
            v.editor.update(cx, |e, cx| e.set_content("movlw 999", cx));
            v.commit_edit(cx);
            assert_eq!(v.machine().rom[0x20], 0x280B);
            assert!(v.status.contains("8 bits"));
        });

        // Breakpoints toggle, and a run stops at one.
        view.update(cx, |v, cx| {
            v.toggle_breakpoint_at(0x0D, cx);
            v.real_time = false;
            v.run(cx);
        });
        for _ in 0..200 {
            cx.background_executor.advance_clock(FRAME);
            cx.run_until_parked();
            if view.read_with(cx, |v, _| v.running.is_none()) {
                break;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        view.update(cx, |v, cx| {
            v.halt(cx);
            assert_eq!(v.shown.pc, 0x0D, "{}", v.status);
            assert!(v.status.contains("breakpoint"));
        });
        draw(&view, cx);
    }

    #[gpui::test]
    async fn pins_and_ram_edit(cx: &mut gpui::TestAppContext) {
        let (view, cx) = cx.add_window_view(|_window, cx| Simulator::new(None, cx));
        cx.run_until_parked();
        view.update_in(cx, |v, window, cx| {
            v.start_edit(Edit::RamValue(0x20), window, cx);
            v.editor.update(cx, |e, cx| e.set_content("b'1010'", cx));
            v.commit_edit(cx);
            assert_eq!(v.shown.peek(0x20), 10);
            v.drive_pin(3, cx);
            assert!(v.shown.pin_level(3));
        });
        draw(&view, cx);
    }
}
