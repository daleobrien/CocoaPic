//! The window's colours, following the system appearance — the same scheme as
//! StorySearch: two palettes of the standard macOS greys and accent blue, one
//! chosen by a flag that `theme::set` flips from the appearance observer.

use std::sync::atomic::{AtomicBool, Ordering};

use gpui::{Rgba, WindowAppearance, rgb};

static DARK: AtomicBool = AtomicBool::new(false);

pub fn set(appearance: WindowAppearance) {
    let dark = matches!(
        appearance,
        WindowAppearance::Dark | WindowAppearance::VibrantDark
    );
    DARK.store(dark, Ordering::Relaxed);
}

struct Palette {
    bg: u32,
    panel: u32,
    border: u32,
    fg: u32,
    muted: u32,
    faint: u32,
    accent: u32,
    hover: u32,
    tab_active: u32,
    stripe: u32,
    /// A value that changed since the machine last stopped — the old app's red.
    changed: u32,
    /// A breakpoint dot.
    breakpoint: u32,
    /// A pin driven high.
    high: u32,
}

static LIGHT: Palette = Palette {
    bg: 0xFFFFFF,
    panel: 0xF5F5F7,
    border: 0xD8D8DC,
    fg: 0x1D1D1F,
    muted: 0x6E6E73,
    faint: 0xAEAEB2,
    accent: 0x007AFF,
    hover: 0xEFEFF1,
    tab_active: 0xE3E3E8,
    stripe: 0xF7F7FB,
    changed: 0xD70015,
    breakpoint: 0xE0383E,
    high: 0x28A745,
};

static DARK_PALETTE: Palette = Palette {
    bg: 0x1E1E1E,
    panel: 0x2C2C2E,
    border: 0x3A3A3C,
    fg: 0xE8E8ED,
    muted: 0x98989D,
    faint: 0x636366,
    accent: 0x0A84FF,
    hover: 0x343436,
    tab_active: 0x48484A,
    stripe: 0x242426,
    changed: 0xFF6961,
    breakpoint: 0xFF453A,
    high: 0x32D74B,
};

fn palette() -> &'static Palette {
    if DARK.load(Ordering::Relaxed) {
        &DARK_PALETTE
    } else {
        &LIGHT
    }
}

pub fn bg() -> Rgba {
    rgb(palette().bg)
}

pub fn panel() -> Rgba {
    rgb(palette().panel)
}

pub fn border() -> Rgba {
    rgb(palette().border)
}

pub fn fg() -> Rgba {
    rgb(palette().fg)
}

pub fn muted() -> Rgba {
    rgb(palette().muted)
}

pub fn faint() -> Rgba {
    rgb(palette().faint)
}

pub fn accent() -> Rgba {
    rgb(palette().accent)
}

pub fn hover() -> Rgba {
    rgb(palette().hover)
}

pub fn tab_active() -> Rgba {
    rgb(palette().tab_active)
}

pub fn stripe() -> Rgba {
    rgb(palette().stripe)
}

pub fn changed() -> Rgba {
    rgb(palette().changed)
}

pub fn breakpoint() -> Rgba {
    rgb(palette().breakpoint)
}

pub fn high() -> Rgba {
    rgb(palette().high)
}

/// The accent at a given opacity — the selection behind the text input, and
/// the band across the row the PC is on.
pub fn accent_tint(alpha: f32) -> Rgba {
    let mut c = accent();
    c.a = alpha;
    c
}
