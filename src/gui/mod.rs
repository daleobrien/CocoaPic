//! The native frontend: one gpui window driving [`crate::pic::Pic`]
//! in-process, built the way StorySearch's is — gpui for the window, a
//! system-following palette, and the menu bar and keys wired to actions.

mod actions;
mod simulator;
mod theme;
mod widgets;

use std::path::PathBuf;

use anyhow::Result;
use gpui::{
    Action, App, AppContext as _, Entity, KeyBinding, Menu, MenuItem, TitlebarOptions,
    WindowBounds, WindowOptions, point, px, size,
};

use self::actions::*;
use self::simulator::Simulator;
use self::widgets::text_input as ti;

/// Open the window and run until it closes.
pub fn run(file: Option<PathBuf>) -> Result<()> {
    gpui_platform::application().run(move |cx| {
        theme::set(cx.window_appearance());

        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();

        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.on_action(|_: &CloseWindow, cx| {
            if let Some(win) = cx.active_window() {
                win.update(cx, |_root, window, _cx| window.remove_window())
                    .ok();
            }
        });
        cx.bind_keys(keys());
        cx.set_menus(menus());

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(1240.), px(820.)), cx)),
            window_min_size: Some(size(px(900.), px(560.))),
            titlebar: Some(TitlebarOptions {
                title: Some("CocoaPic".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(14.), px(14.))),
            }),
            ..Default::default()
        };

        let mut view: Option<Entity<Simulator>> = None;
        let opened = cx.open_window(options, |window, cx| {
            let sim = cx.new(|cx| Simulator::new(file, cx));
            let handle = sim.read(cx).focus_handle_of();
            window.focus(&handle, cx);
            view = Some(sim.clone());
            sim
        });
        let (Ok(window), Some(sim)) = (opened, view) else {
            eprintln!("cocoapic: could not open a window");
            cx.quit();
            return;
        };

        // Every command goes to the one simulator, wherever focus sits — an
        // action that only fires when focus happens to be on the right element
        // is one that mysteriously does nothing.
        bind::<Open>(&sim, cx, |v, cx| v.open(cx));
        bind::<Save>(&sim, cx, |v, cx| v.save(cx));
        bind::<SaveAs>(&sim, cx, |v, cx| v.save_as(cx));
        bind::<LoadDemo>(&sim, cx, |v, cx| v.load_demo(cx));
        bind::<Reset>(&sim, cx, |v, cx| v.reset(cx));
        bind::<Step>(&sim, cx, |v, cx| v.step(cx));
        bind::<Run>(&sim, cx, |v, cx| v.run(cx));
        bind::<Halt>(&sim, cx, |v, cx| v.halt(cx));
        bind::<RealTime>(&sim, cx, |v, cx| v.toggle_real_time(cx));
        bind::<ToggleBreakpoint>(&sim, cx, |v, cx| v.toggle_breakpoint(cx));
        bind::<ClearBreakpoints>(&sim, cx, |v, cx| v.clear_breakpoints(cx));
        bind::<CancelEdit>(&sim, cx, |v, cx| v.cancel_edit(cx));

        window
            .update(cx, |_root, window, _cx| {
                window
                    .observe_window_appearance(|window, cx| {
                        theme::set(window.appearance());
                        cx.refresh_windows();
                    })
                    .detach();
            })
            .ok();

        cx.activate(true);
    });
    Ok(())
}

fn bind<A: Action>(
    sim: &Entity<Simulator>,
    cx: &mut App,
    f: fn(&mut Simulator, &mut gpui::Context<Simulator>),
) {
    let weak = sim.downgrade();
    cx.on_action(move |_: &A, cx| {
        weak.update(cx, f).ok();
    });
}

fn menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "CocoaPic".into(),
            items: vec![MenuItem::action("Quit CocoaPic", Quit)],
            disabled: false,
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("Open…", Open),
                MenuItem::action("Open Demo Program", LoadDemo),
                MenuItem::separator(),
                MenuItem::action("Save", Save),
                MenuItem::action("Save As…", SaveAs),
                MenuItem::separator(),
                MenuItem::action("Close Window", CloseWindow),
            ],
            disabled: false,
        },
        Menu {
            name: "Debug".into(),
            items: vec![
                MenuItem::action("Run", Run),
                MenuItem::action("Halt", Halt),
                MenuItem::action("Step", Step),
                MenuItem::action("Reset", Reset),
                MenuItem::separator(),
                MenuItem::action("Real Time", RealTime),
                MenuItem::separator(),
                MenuItem::action("Toggle Breakpoint", ToggleBreakpoint),
                MenuItem::action("Clear All Breakpoints", ClearBreakpoints),
            ],
            disabled: false,
        },
    ]
}

fn keys() -> Vec<KeyBinding> {
    vec![
        KeyBinding::new("cmd-q", Quit, None),
        KeyBinding::new("cmd-w", CloseWindow, None),
        KeyBinding::new("cmd-o", Open, None),
        KeyBinding::new("cmd-s", Save, None),
        KeyBinding::new("cmd-shift-s", SaveAs, None),
        KeyBinding::new("cmd-r", Run, None),
        KeyBinding::new("cmd-.", Halt, None),
        KeyBinding::new("f7", Step, None),
        KeyBinding::new("cmd-'", Step, None),
        KeyBinding::new("cmd-shift-r", Reset, None),
        KeyBinding::new("cmd-b", ToggleBreakpoint, None),
        KeyBinding::new("escape", CancelEdit, Some("TextInput")),
        // The text input's own keys, scoped to its key context.
        KeyBinding::new("backspace", ti::Backspace, Some("TextInput")),
        KeyBinding::new("delete", ti::Delete, Some("TextInput")),
        KeyBinding::new("left", ti::Left, Some("TextInput")),
        KeyBinding::new("right", ti::Right, Some("TextInput")),
        KeyBinding::new("shift-left", ti::SelectLeft, Some("TextInput")),
        KeyBinding::new("shift-right", ti::SelectRight, Some("TextInput")),
        KeyBinding::new("cmd-a", ti::SelectAll, Some("TextInput")),
        KeyBinding::new("cmd-v", ti::Paste, Some("TextInput")),
        KeyBinding::new("cmd-c", ti::Copy, Some("TextInput")),
        KeyBinding::new("cmd-x", ti::Cut, Some("TextInput")),
        KeyBinding::new("home", ti::Home, Some("TextInput")),
        KeyBinding::new("end", ti::End, Some("TextInput")),
        KeyBinding::new(
            "ctrl-cmd-space",
            ti::ShowCharacterPalette,
            Some("TextInput"),
        ),
        KeyBinding::new("enter", ti::Submit, Some("TextInput")),
    ]
}
