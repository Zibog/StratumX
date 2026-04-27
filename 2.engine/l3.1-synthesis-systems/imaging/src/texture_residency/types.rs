use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextureFormat {
    RGBA8,
    RGB8,
    RG8,
    R8,
    RGBA16F,
    RGB16F,
    BC1,
    BC3,
    BC7,
}

impl TextureFormat {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            TextureFormat::RGBA8 => 4,
            TextureFormat::RGB8 => 3,
            TextureFormat::RG8 => 2,
            TextureFormat::R8 => 1,
            TextureFormat::RGBA16F => 8,
            TextureFormat::RGB16F => 6,
            TextureFormat::BC1 => 0,
            TextureFormat::BC3 => 1,
            TextureFormat::BC7 => 1,
        }
    }

    pub fn calculate_size(&self, width: u32, height: u32) -> usize {
        match self {
            TextureFormat::BC1 => ((width * height) / 2) as usize,
            TextureFormat::BC3 | TextureFormat::BC7 => (width * height) as usize,
            _ => (width * height) as usize * self.bytes_per_pixel(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextureDescriptor {
    pub texture_id: u64,
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub mip_levels: u32,
    pub size_bytes: usize,
}

impl TextureDescriptor {
    pub fn new(texture_id: u64, width: u32, height: u32, format: TextureFormat) -> Self {
        let mut size_bytes = 0;
        let mut w = width;
        let mut h = height;

        let mip_levels = (width.max(height) as f32).log2().floor() as u32 + 1;

        for _ in 0..mip_levels {
            size_bytes += format.calculate_size(w, h);
            w = (w / 2).max(1);
            h = (h / 2).max(1);
        }

        Self {
            texture_id,
            width,
            height,
            format,
            mip_levels,
            size_bytes,
        }
    }
}
