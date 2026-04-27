use engine_material::*;

// === Basic ID Type Tests ===

#[test]
fn test_material_id_creation() {
    let id = MaterialId(42);
    assert_eq!(id, MaterialId(42));
}

#[test]
fn test_material_id_equality() {
    assert_eq!(MaterialId(1), MaterialId(1));
    assert_ne!(MaterialId(1), MaterialId(2));
}

#[test]
fn test_material_stack_id_creation() {
    let id = MaterialStackId(100);
    assert_eq!(id, MaterialStackId(100));
}

#[test]
fn test_material_archetype_id_creation() {
    let id = MaterialArchetypeId(50);
    assert_eq!(id, MaterialArchetypeId(50));
}

#[test]
fn test_response_profile_id_creation() {
    let id = ResponseProfileId(75);
    assert_eq!(id, ResponseProfileId(75));
}

// === WeatherRegime Tests ===

#[test]
fn test_weather_regime_clear_equals_itself() {
    assert_eq!(WeatherRegime::Clear, WeatherRegime::Clear);
}

#[test]
fn test_weather_regime_clear_ne_heavy_storm() {
    assert_ne!(WeatherRegime::Clear, WeatherRegime::HeavyStorm);
}

// === MaterialLayer Tests ===

#[test]
fn test_material_layer_creation() {
    let layer = MaterialLayer {
        archetype_id: MaterialArchetypeId(1),
        thickness_mm: 10.0,
        coverage: 1.0,
    };
    assert_eq!(layer.thickness_mm, 10.0);
    assert_eq!(layer.coverage, 1.0);
}

#[test]
fn test_material_layer_equality() {
    let a = MaterialLayer {
        archetype_id: MaterialArchetypeId(1),
        thickness_mm: 10.0,
        coverage: 1.0,
    };
    let b = MaterialLayer {
        archetype_id: MaterialArchetypeId(1),
        thickness_mm: 10.0,
        coverage: 1.0,
    };
    assert_eq!(a, b);
}

// === MaterialStack Tests ===

#[test]
fn test_material_stack_creation() {
    let stack = MaterialStack {
        id: MaterialStackId(1),
        label: "Test".to_string(),
        layers: vec![MaterialLayer {
            archetype_id: MaterialArchetypeId(1),
            thickness_mm: 10.0,
            coverage: 1.0,
        }],
    };
    assert_eq!(stack.layers.len(), 1);
}

// === LayerDamageState Tests ===

#[test]
fn test_layer_damage_state_creation() {
    let state = LayerDamageState {
        layer_index: 0,
        integrity: 1.0,
        accumulated_energy_j: 0.0,
        segment_states: vec![],
    };
    assert_eq!(state.integrity, 1.0);
}

// === SegmentState Tests ===

#[test]
fn test_segment_state_variants() {
    let states = vec![
        SegmentState::Intact,
        SegmentState::Cracked,
        SegmentState::Released,
    ];
    assert_eq!(states.len(), 3);
}

// === ImpactResponseInput Tests ===

#[test]
fn test_impact_response_input_creation() {
    let input = ImpactResponseInput {
        entry_energy_j: 100.0,
        incidence_angle_deg: 45.0,
        projectile_diameter_mm: 10.0,
    };
    assert_eq!(input.entry_energy_j, 100.0);
}

// === MaterialBehavior Flags Tests ===

#[test]
fn test_material_behavior_flags_creation() {
    let flags = MaterialBehaviorFlags::default();
    let _ = flags;
}

// === PropertyDomain Tests ===

#[test]
fn test_property_domain_exists() {
    let _domain = PropertyDomain::Structural;
}

// === SkyWeatherState Tests ===

#[test]
fn test_sky_weather_state_new_default() {
    let sky = SkyWeatherState::new_default();
    drop(sky);
}

// === FluidContainer Tests ===

#[test]
fn test_fluid_container_new() {
    let container = FluidContainer::new([0.0, 0.0, 0.0], 50.0, 2.0, 1.0);
    drop(container);
}

// === HydrologyState Tests ===

#[test]
fn test_hydrology_state_new() {
    let container = FluidContainer::new([0.0, 0.0, 0.0], 100.0, 1.0, 1.0);
    let hydro = HydrologyState::new(container);
    drop(hydro);
}

// === Rainfall Tests ===

#[test]
fn test_rainfall_new() {
    let rain = Rainfall::new(10.0, 100.0);
    drop(rain);
}

// === Evaporation Tests ===

#[test]
fn test_evaporation_new() {
    let evap = Evaporation::new(25.0, 60.0, 10.0);
    drop(evap);
}

// === SmokeSystem Tests ===

#[test]
fn test_smoke_system_new() {
    let smoke = SmokeSystem::new();
    drop(smoke);
}

// === WetnessState Tests ===

#[test]
fn test_wetness_state_new() {
    let wetness = WetnessState::new();
    drop(wetness);
}

// === FogState Tests ===

#[test]
fn test_fog_state_new() {
    let fog = FogState::new(0.5);
    drop(fog);
}
