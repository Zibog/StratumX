use crate::material_types::{
    AcousticSurfaceClass, BuoyancyResponse, FireResponse, FractureMode, MaterialArchetype,
    MaterialArchetypeId, MaterialBehaviorFlags, MechanicalClass, SegmentationMode,
    StructuralResponse,
};

pub fn ceramic_tile() -> MaterialArchetype {
    MaterialArchetype {
        id: MaterialArchetypeId(1),
        label: "ceramic_tile".into(),
        behavior: MaterialBehaviorFlags {
            can_crack: true,
            can_shatter: true,
            ..Default::default()
        },
        mechanical_class: MechanicalClass::Brittle,
        fracture_mode: FractureMode::Segment,
        acoustic_class: AcousticSurfaceClass::Ceramic,
        fire_response: FireResponse::Inert,
        buoyancy: BuoyancyResponse::Sink,
        structural: StructuralResponse::Decorative,
        segmentation: SegmentationMode::Grid { rows: 4, cols: 4 },
        density_kg_m3: 2300.0,
        hardness_mohs: 7.0,
        fracture_energy_j_m2: 10.0,
        bond_strength_mpa: 5.0,
        thickness_mm: 8.0,
    }
}

pub fn plaster() -> MaterialArchetype {
    MaterialArchetype {
        id: MaterialArchetypeId(2),
        label: "plaster".into(),
        behavior: MaterialBehaviorFlags {
            can_crumble: true,
            can_crack: true,
            ..Default::default()
        },
        mechanical_class: MechanicalClass::Brittle,
        fracture_mode: FractureMode::Crumble,
        acoustic_class: AcousticSurfaceClass::Soft,
        fire_response: FireResponse::Inert,
        buoyancy: BuoyancyResponse::Sink,
        structural: StructuralResponse::Decorative,
        segmentation: SegmentationMode::None,
        density_kg_m3: 1200.0,
        hardness_mohs: 2.0,
        fracture_energy_j_m2: 2.0,
        bond_strength_mpa: 1.5,
        thickness_mm: 15.0,
    }
}

pub fn concrete() -> MaterialArchetype {
    MaterialArchetype {
        id: MaterialArchetypeId(3),
        label: "concrete".into(),
        behavior: MaterialBehaviorFlags {
            can_chip: true,
            can_crack: true,
            ..Default::default()
        },
        mechanical_class: MechanicalClass::Composite,
        fracture_mode: FractureMode::Chip,
        acoustic_class: AcousticSurfaceClass::Hard,
        fire_response: FireResponse::Inert,
        buoyancy: BuoyancyResponse::Sink,
        structural: StructuralResponse::LoadBearing,
        segmentation: SegmentationMode::None,
        density_kg_m3: 2400.0,
        hardness_mohs: 5.0,
        fracture_energy_j_m2: 100.0,
        bond_strength_mpa: 30.0,
        thickness_mm: 200.0,
    }
}
