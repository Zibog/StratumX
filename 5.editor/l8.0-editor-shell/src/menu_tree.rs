//! Menu Tree - Declarative menu structure.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuEntry {
    pub id: &'static str,
    pub label: &'static str,
    pub shortcut: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuSection {
    pub id: &'static str,
    pub label: &'static str,
    pub entries: &'static [MenuEntry],
}

pub const FILE_MENU: &[MenuEntry] = &[
    MenuEntry {
        id: "file.new_project",
        label: "New Project",
        shortcut: Some("Ctrl+N"),
    },
    MenuEntry {
        id: "file.open_world",
        label: "Open World",
        shortcut: Some("Ctrl+O"),
    },
    MenuEntry {
        id: "file.save",
        label: "Save",
        shortcut: Some("Ctrl+S"),
    },
];

pub const RUNTIME_MENU: &[MenuEntry] = &[
    MenuEntry {
        id: "world.play",
        label: "Play",
        shortcut: Some("F5"),
    },
    MenuEntry {
        id: "world.pause",
        label: "Pause",
        shortcut: Some("Shift+F5"),
    },
    MenuEntry {
        id: "world.stop",
        label: "Stop",
        shortcut: Some("Ctrl+F5"),
    },
    MenuEntry {
        id: "world.simulate",
        label: "Simulate",
        shortcut: None,
    },
];

pub const PANEL_MENU: &[MenuEntry] = &[
    MenuEntry {
        id: "panel.viewport",
        label: "Viewport",
        shortcut: Some("Ctrl+1"),
    },
    MenuEntry {
        id: "panel.outliner",
        label: "Outliner",
        shortcut: Some("Ctrl+2"),
    },
    MenuEntry {
        id: "panel.inspector",
        label: "Inspector",
        shortcut: Some("Ctrl+3"),
    },
    MenuEntry {
        id: "panel.diagnostics",
        label: "Diagnostics",
        shortcut: Some("Ctrl+4"),
    },
    MenuEntry {
        id: "panel.terrain",
        label: "Terrain",
        shortcut: Some("Ctrl+5"),
    },
    MenuEntry {
        id: "panel.environment",
        label: "Environment",
        shortcut: Some("Ctrl+6"),
    },
];

pub const MENU_SECTIONS: &[MenuSection] = &[
    MenuSection {
        id: "file",
        label: "File",
        entries: FILE_MENU,
    },
    MenuSection {
        id: "runtime",
        label: "Runtime",
        entries: RUNTIME_MENU,
    },
    MenuSection {
        id: "panels",
        label: "Panels",
        entries: PANEL_MENU,
    },
];
