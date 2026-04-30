//! Command palette filtering and fuzzy matching.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandPaletteEntry {
    pub id: &'static str,
    pub label: &'static str,
    pub keywords: &'static [&'static str],
}

pub const COMMAND_PALETTE_ENTRIES: &[CommandPaletteEntry] = &[
    CommandPaletteEntry {
        id: "file.new_project",
        label: "New Project",
        keywords: &["project", "new", "create"],
    },
    CommandPaletteEntry {
        id: "file.open_world",
        label: "Open World",
        keywords: &["world", "open", "load"],
    },
    CommandPaletteEntry {
        id: "file.save",
        label: "Save World",
        keywords: &["save", "world", "project"],
    },
    CommandPaletteEntry {
        id: "world.play",
        label: "Play Runtime",
        keywords: &["play", "runtime"],
    },
    CommandPaletteEntry {
        id: "world.pause",
        label: "Pause Runtime",
        keywords: &["pause", "runtime"],
    },
    CommandPaletteEntry {
        id: "world.stop",
        label: "Stop Runtime",
        keywords: &["stop", "runtime"],
    },
    CommandPaletteEntry {
        id: "world.simulate",
        label: "Simulate Runtime",
        keywords: &["simulate", "runtime"],
    },
    CommandPaletteEntry {
        id: "panel.viewport",
        label: "Toggle Viewport Panel",
        keywords: &["panel", "viewport", "view"],
    },
    CommandPaletteEntry {
        id: "panel.outliner",
        label: "Toggle Outliner Panel",
        keywords: &["panel", "outliner"],
    },
    CommandPaletteEntry {
        id: "panel.inspector",
        label: "Toggle Inspector Panel",
        keywords: &["panel", "inspector"],
    },
    CommandPaletteEntry {
        id: "panel.diagnostics",
        label: "Toggle Diagnostics Panel",
        keywords: &["panel", "diagnostics"],
    },
    CommandPaletteEntry {
        id: "panel.terrain",
        label: "Toggle Terrain Panel",
        keywords: &["panel", "terrain"],
    },
    CommandPaletteEntry {
        id: "panel.environment",
        label: "Toggle Environment Panel",
        keywords: &["panel", "environment", "sky"],
    },
];

pub fn filter_command_palette(query: &str) -> Vec<&'static CommandPaletteEntry> {
    let normalized_query = normalize(query);
    let mut filtered = COMMAND_PALETTE_ENTRIES
        .iter()
        .filter(|entry| {
            normalized_query.is_empty()
                || fuzzy_match(&normalized_query, &normalize(entry.label))
                || fuzzy_match(&normalized_query, &normalize(entry.id))
                || entry
                    .keywords
                    .iter()
                    .any(|keyword| fuzzy_match(&normalized_query, &normalize(keyword)))
        })
        .collect::<Vec<_>>();

    filtered.sort_by_key(|entry| entry.label);
    filtered
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_whitespace() && *ch != '.' && *ch != '_' && *ch != '-')
        .flat_map(char::to_lowercase)
        .collect()
}

fn fuzzy_match(query: &str, candidate: &str) -> bool {
    if query.is_empty() {
        return true;
    }

    let mut query_chars = query.chars();
    let mut current = query_chars.next();
    for candidate_char in candidate.chars() {
        if Some(candidate_char) == current {
            current = query_chars.next();
            if current.is_none() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_palette_filter_matches_keywords_and_fuzzy_query() {
        let filtered = filter_command_palette("diag");
        assert!(filtered.iter().any(|entry| entry.id == "panel.diagnostics"));

        let fuzzy = filter_command_palette("nprj");
        assert!(fuzzy.iter().any(|entry| entry.id == "file.new_project"));
    }
}
