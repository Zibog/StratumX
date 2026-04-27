use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChunkHeader {
    pub magic: [u8; 4],
    pub version: u32,
    pub width: u32,
    pub height: u32,
}

impl ChunkHeader {
    pub const MAGIC: [u8; 4] = *b"SXCH";
    pub const VERSION: u32 = 1;
    pub const SIZE: usize = 16;

    pub fn new(width: u32, height: u32) -> Self {
        Self {
            magic: Self::MAGIC,
            version: Self::VERSION,
            width,
            height,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC && self.version == Self::VERSION
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkData {
    pub header: ChunkHeader,
    pub heights: Vec<f32>,
    pub material_weights: Vec<[f32; 4]>,
}

impl ChunkData {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            header: ChunkHeader::new(width, height),
            heights: vec![0.0; size],
            material_weights: vec![[1.0, 0.0, 0.0, 0.0]; size],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.magic);
        bytes.extend_from_slice(&self.header.version.to_le_bytes());
        bytes.extend_from_slice(&self.header.width.to_le_bytes());
        bytes.extend_from_slice(&self.header.height.to_le_bytes());
        for h in &self.heights {
            bytes.extend_from_slice(&h.to_le_bytes());
        }
        for weights in &self.material_weights {
            for w in weights {
                bytes.extend_from_slice(&w.to_le_bytes());
            }
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < ChunkHeader::SIZE {
            return Err("Chunk data too small".to_string());
        }
        let mut magic = [0u8; 4];
        magic.copy_from_slice(&bytes[0..4]);
        let version = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let width = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let height = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        let header = ChunkHeader {
            magic,
            version,
            width,
            height,
        };
        if !header.is_valid() {
            return Err(format!(
                "Invalid chunk header: magic={:?}, version={}",
                magic, version
            ));
        }
        let size = (width * height) as usize;
        let expected_size = ChunkHeader::SIZE + size * 4 + size * 16;
        if bytes.len() < expected_size {
            return Err(format!(
                "Chunk data incomplete: expected {} bytes, got {}",
                expected_size,
                bytes.len()
            ));
        }
        let mut heights = Vec::with_capacity(size);
        let mut offset = ChunkHeader::SIZE;
        for _ in 0..size {
            let h = f32::from_le_bytes([
                bytes[offset],
                bytes[offset + 1],
                bytes[offset + 2],
                bytes[offset + 3],
            ]);
            heights.push(h);
            offset += 4;
        }
        let mut material_weights = Vec::with_capacity(size);
        for _ in 0..size {
            let mut weights = [0.0f32; 4];
            for w in &mut weights {
                *w = f32::from_le_bytes([
                    bytes[offset],
                    bytes[offset + 1],
                    bytes[offset + 2],
                    bytes[offset + 3],
                ]);
                offset += 4;
            }
            material_weights.push(weights);
        }
        Ok(Self {
            header,
            heights,
            material_weights,
        })
    }
}
