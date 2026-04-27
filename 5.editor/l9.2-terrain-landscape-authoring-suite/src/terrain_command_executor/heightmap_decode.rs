type HeightmapData = (Vec<f32>, u32, u32);

pub(crate) fn decode_raw_heightmap(data: &[u8]) -> Result<HeightmapData, String> {
    let total_pixels = data.len();
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("RAW heightmap must be square".to_string());
    }

    let samples = data
        .iter()
        .map(|value| *value as f32 / 255.0 * 1000.0)
        .collect();
    Ok((samples, size, size))
}

pub(crate) fn decode_r16_heightmap(data: &[u8]) -> Result<HeightmapData, String> {
    if data.len() % 2 != 0 {
        return Err("R16 heightmap byte count must be even".to_string());
    }

    let total_pixels = data.len() / 2;
    let size = (total_pixels as f64).sqrt() as u32;
    if (size * size) as usize != total_pixels {
        return Err("R16 heightmap must be square".to_string());
    }

    let mut samples = Vec::with_capacity(total_pixels);
    for index in 0..total_pixels {
        let offset = index * 2;
        let value = u16::from_le_bytes([data[offset], data[offset + 1]]);
        samples.push(value as f32 / 65535.0 * 1000.0);
    }
    Ok((samples, size, size))
}

pub(crate) fn decode_png_heightmap(data: &[u8]) -> Result<HeightmapData, String> {
    let image = image::load_from_memory(data)
        .map_err(|error| format!("Failed to decode PNG heightmap: {}", error))?;
    let grayscale = image.to_luma8();
    let width = grayscale.width();
    let height = grayscale.height();
    let samples = grayscale
        .pixels()
        .map(|pixel| pixel[0] as f32 / 255.0 * 1000.0)
        .collect();
    Ok((samples, width, height))
}
