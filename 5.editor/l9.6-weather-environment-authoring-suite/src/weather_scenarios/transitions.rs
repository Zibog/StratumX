// Weather cell placement strategies

pub enum CellPlacementStrategy {
    Random { center: [f32; 3], radius_km: f32 },
    Grid { center: [f32; 3], spacing_km: f32 },
    Line { start: [f32; 3], end: [f32; 3] },
    Circle { center: [f32; 3], radius_km: f32 },
}

impl CellPlacementStrategy {
    pub fn generate_positions(&self, count: usize) -> Vec<[f32; 3]> {
        match self {
            Self::Random { center, radius_km } => (0..count)
                .map(|i| {
                    let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
                    let r = radius_km * 1000.0 * (i as f32 / count as f32).sqrt();
                    [
                        center[0] + r * angle.cos(),
                        center[1],
                        center[2] + r * angle.sin(),
                    ]
                })
                .collect(),
            Self::Grid { center, spacing_km } => {
                let grid_size = (count as f32).sqrt().ceil() as usize;
                let mut positions = Vec::new();
                for i in 0..grid_size {
                    for j in 0..grid_size {
                        if positions.len() >= count {
                            break;
                        }
                        let x =
                            center[0] + (i as f32 - grid_size as f32 / 2.0) * spacing_km * 1000.0;
                        let z =
                            center[2] + (j as f32 - grid_size as f32 / 2.0) * spacing_km * 1000.0;
                        positions.push([x, center[1], z]);
                    }
                }
                positions
            }
            Self::Line { start, end } => (0..count)
                .map(|i| {
                    let t = i as f32 / (count - 1).max(1) as f32;
                    [
                        start[0] + (end[0] - start[0]) * t,
                        start[1] + (end[1] - start[1]) * t,
                        start[2] + (end[2] - start[2]) * t,
                    ]
                })
                .collect(),
            Self::Circle { center, radius_km } => (0..count)
                .map(|i| {
                    let angle = (i as f32 / count as f32) * std::f32::consts::TAU;
                    let r = radius_km * 1000.0;
                    [
                        center[0] + r * angle.cos(),
                        center[1],
                        center[2] + r * angle.sin(),
                    ]
                })
                .collect(),
        }
    }
}
