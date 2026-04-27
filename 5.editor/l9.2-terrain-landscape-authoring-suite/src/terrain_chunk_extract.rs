//! Terrain preview cache chunk extraction helpers.

use std::collections::HashMap;

/// Extract a summary of heightmap data for a specific chunk.
/// Returns a vector of min, max, mean, and variance of heights in the chunk.
pub fn extract_chunk_heightmap_summary(
    samples: &[f32],
    resolution: [u32; 2],
    chunk_x: u32,
    chunk_y: u32,
    chunk_size: usize,
) -> Vec<f32> {
    let res_x = resolution[0] as usize;
    let res_y = resolution[1] as usize;

    let start_x = (chunk_x as usize * chunk_size).min(res_x);
    let start_y = (chunk_y as usize * chunk_size).min(res_y);
    let end_x = ((chunk_x as usize + 1) * chunk_size).min(res_x);
    let end_y = ((chunk_y as usize + 1) * chunk_size).min(res_y);

    let mut min_height = f32::MAX;
    let mut max_height = f32::MIN;
    let mut sum = 0.0f32;
    let mut count = 0usize;

    for y in start_y..end_y {
        for x in start_x..end_x {
            let index = y * res_x + x;
            if let Some(&height) = samples.get(index) {
                min_height = min_height.min(height);
                max_height = max_height.max(height);
                sum += height;
                count += 1;
            }
        }
    }

    if count == 0 {
        return vec![0.0, 0.0, 0.0, 0.0];
    }

    let mean = sum / count as f32;
    let mut variance_sum = 0.0f32;
    for y in start_y..end_y {
        for x in start_x..end_x {
            let index = y * res_x + x;
            if let Some(&height) = samples.get(index) {
                let diff = height - mean;
                variance_sum += diff * diff;
            }
        }
    }
    let variance = variance_sum / count as f32;

    vec![min_height, max_height, mean, variance]
}

/// Extract dominant texture layer IDs for a chunk.
/// Returns a list of layer ID strings, sorted by coverage.
pub fn extract_chunk_texture_layers(
    layer_ids: &[u16],
    resolution: [u32; 2],
    chunk_x: u32,
    chunk_y: u32,
    chunk_size: usize,
) -> Vec<String> {
    let res_x = resolution[0] as usize;
    let res_y = resolution[1] as usize;

    let start_x = (chunk_x as usize * chunk_size).min(res_x);
    let start_y = (chunk_y as usize * chunk_size).min(res_y);
    let end_x = ((chunk_x as usize + 1) * chunk_size).min(res_x);
    let end_y = ((chunk_y as usize + 1) * chunk_size).min(res_y);

    let mut layer_counts: HashMap<u16, usize> = HashMap::new();
    for y in start_y..end_y {
        for x in start_x..end_x {
            let index = y * res_x + x;
            if let Some(&layer_id) = layer_ids.get(index) {
                *layer_counts.entry(layer_id).or_insert(0) += 1;
            }
        }
    }

    let mut sorted_layers: Vec<(u16, usize)> = layer_counts.into_iter().collect();
    sorted_layers.sort_by(|a, b| b.1.cmp(&a.1));

    sorted_layers
        .into_iter()
        .map(|(id, _count)| id.to_string())
        .collect()
}
