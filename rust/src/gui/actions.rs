//! The window's actions: what a menu item names and a keybinding fires.

gpui::actions!(
    cocoapic,
    [
        Quit,
        CloseWindow,
        Open,
        Save,
        SaveAs,
        LoadDemo,
        Reset,
        Step,
        Run,
        Halt,
        RealTime,
        ToggleBreakpoint,
        ClearBreakpoints,
        CancelEdit,
    ]
);
