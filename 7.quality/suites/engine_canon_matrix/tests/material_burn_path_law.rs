// Material Burn Path Law Tests
//
// Tests for BLKR-MAT-BURN-01 (MAT-04):
// - Wet material blocks ignition
// - Sustained heat dries first
// - Dry material ignites after threshold
// - Burn consequence event
// - Downstream publication metadata
// - Invalid transition denial
// - Deterministic burn result

use engine_material::fire_weather::{CombustibleMaterial, CombustibleObject, FireState};

include!("material_burn_path_law/cases_01.rs");
include!("material_burn_path_law/cases_02.rs");
