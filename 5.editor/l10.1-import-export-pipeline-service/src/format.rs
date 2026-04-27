#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetFormat {
    GLTF,
    FBX,
    OBJ,
    PNG,
    JPG,
    WAV,
    MP3,
}

impl AssetFormat {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "gltf" | "glb" => Some(AssetFormat::GLTF),
            "fbx" => Some(AssetFormat::FBX),
            "obj" => Some(AssetFormat::OBJ),
            "png" => Some(AssetFormat::PNG),
            "jpg" | "jpeg" => Some(AssetFormat::JPG),
            "wav" => Some(AssetFormat::WAV),
            "mp3" => Some(AssetFormat::MP3),
            _ => None,
        }
    }
}