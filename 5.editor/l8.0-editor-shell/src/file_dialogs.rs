//! File dialog ownership for shell-level file and import flows.

use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDialogKind {
    ExistingFolder,
    WorldFolder,
    HeightmapFile,
    TextureFile,
}

pub fn show_file_dialog(kind: FileDialogKind) -> Option<PathBuf> {
    match kind {
        FileDialogKind::ExistingFolder => rfd::FileDialog::new().pick_folder(),
        FileDialogKind::WorldFolder => rfd::FileDialog::new()
            .set_title("Select world package directory")
            .pick_folder(),
        FileDialogKind::HeightmapFile => rfd::FileDialog::new()
            .set_title("Select heightmap")
            .add_filter("Heightmaps", &["png", "raw", "r16"])
            .pick_file(),
        FileDialogKind::TextureFile => rfd::FileDialog::new()
            .set_title("Select texture")
            .add_filter("Textures", &["png", "jpg", "jpeg", "tga", "dds"])
            .pick_file(),
    }
}
