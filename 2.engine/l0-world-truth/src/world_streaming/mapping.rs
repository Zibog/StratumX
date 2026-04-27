// Region mapping - world position to region key

pub type RegionKey = (i32, i32, i32);

pub const REGION_SIZE: f32 = 100.0;

pub fn world_pos_to_region_key(pos: [f32; 3]) -> RegionKey {
    (
        (pos[0] / REGION_SIZE).floor() as i32,
        (pos[1] / REGION_SIZE).floor() as i32,
        (pos[2] / REGION_SIZE).floor() as i32,
    )
}
