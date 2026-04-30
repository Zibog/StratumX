use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MaterialBehaviorFlags {
    pub can_crack: bool,
    pub can_crumble: bool,
    pub can_chip: bool,
    pub can_shatter: bool,
    pub can_deform: bool,
    pub can_burn: bool,
    pub can_melt: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MechanicalClass {
    Brittle,
    Ductile,
    Elastic,
    Plastic,
    Composite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FractureMode {
    None,
    Segment,
    Crumble,
    Shatter,
    Chip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcousticSurfaceClass {
    Hard,
    Soft,
    Metallic,
    Ceramic,
    Wood,
    Fabric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FireResponse {
    Inert,
    Combustible,
    Flammable,
    Explosive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuoyancyResponse {
    Sink,
    Float,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StructuralResponse {
    LoadBearing,
    Decorative,
    Protective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentationMode {
    None,
    Grid { rows: u8, cols: u8 },
    Voronoi { seed_count: u8 },
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentState {
    Intact,
    Cracked,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyDomain {
    Physical,
    Thermal,
    Fluid,
    Structural,
    Acoustic,
    Appearance,
}
