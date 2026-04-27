use stratumx_editor_l9_3_material_lookdev_authoring_suite::{
    EntityId, MaterialAuthoringService, ObjectHandle, RuntimeCheapnessRung, TextureSlot,
};

#[test]
fn test_create_material() {
    let mut service = MaterialAuthoringService::new();

    let handle = service.create_material("TestMaterial".to_string());

    // Verify material was created
    let registry = service.get_registry();
    assert!(registry.get_profile(handle).is_some());

    // Verify coverage was initialized
    let coverage = service.get_coverage();
    assert!(coverage.contains_key(&handle));
}

#[test]
fn test_bind_material() {
    let mut service = MaterialAuthoringService::new();

    let material_handle = service.create_material("TestMaterial".to_string());
    let entity_id = EntityId::new(1);

    // Bind material to entity
    let result = service.bind_material(entity_id, material_handle);
    assert!(result.is_ok());

    // Verify binding exists
    let bindings = service.get_bindings();
    assert_eq!(bindings.get(&entity_id), Some(&material_handle));
}

#[test]
fn test_bind_nonexistent_material() {
    let mut service = MaterialAuthoringService::new();

    let entity_id = EntityId::new(1);
    let fake_handle = ObjectHandle::new(999);

    // Attempt to bind nonexistent material
    let result = service.bind_material(entity_id, fake_handle);
    assert!(result.is_err());
}

#[test]
fn test_get_profile_names() {
    let mut service = MaterialAuthoringService::new();

    service.create_material("Material1".to_string());
    service.create_material("Material2".to_string());
    service.create_material("Material3".to_string());

    let names = service.get_profile_names();
    assert_eq!(names.len(), 3);
    assert!(names.contains(&"Material1".to_string()));
    assert!(names.contains(&"Material2".to_string()));
    assert!(names.contains(&"Material3".to_string()));
}

#[test]
fn test_unbind_material() {
    let mut service = MaterialAuthoringService::new();

    let material_handle = service.create_material("TestMaterial".to_string());
    let entity_id = EntityId::new(1);

    // Bind and then unbind
    service.bind_material(entity_id, material_handle).unwrap();
    let unbound = service.unbind_material(entity_id);

    assert_eq!(unbound, Some(material_handle));
    assert_eq!(service.get_entity_material(entity_id), None);
}

#[test]
fn test_duplicate_material() {
    let mut service = MaterialAuthoringService::new();

    let original = service.create_material("Original".to_string());
    service
        .bind_visual_response(original, "VisualFamily".to_string())
        .unwrap();

    let duplicate = service
        .duplicate_material(original, "Duplicate".to_string())
        .unwrap();

    // Verify both exist
    let registry = service.get_registry();
    assert!(registry.get_profile(original).is_some());
    assert!(registry.get_profile(duplicate).is_some());

    // Verify duplicate has same visual response
    let dup_profile = registry.get_profile(duplicate).unwrap();
    assert_eq!(
        dup_profile.visual_response,
        Some("VisualFamily".to_string())
    );
    assert_eq!(dup_profile.name, "Duplicate");
}

#[test]
fn test_coverage_updates() {
    let mut service = MaterialAuthoringService::new();

    let handle = service.create_material("TestMaterial".to_string());

    // Initial coverage should show incomplete
    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(!coverage.visual_complete);
    assert!(!coverage.acoustic_complete);

    // Bind visual response
    service
        .bind_visual_response(handle, "VisualFamily".to_string())
        .unwrap();

    // Coverage should update
    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(coverage.visual_complete);
    assert!(!coverage.acoustic_complete);

    // Bind acoustic profile
    service
        .bind_acoustic_profile(handle, "AcousticProfile".to_string())
        .unwrap();

    // Coverage should update again
    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(coverage.visual_complete);
    assert!(coverage.acoustic_complete);
}

#[test]
fn test_physical_closure_is_not_visual_plus_acoustic() {
    let mut service = MaterialAuthoringService::new();

    let handle = service.create_material("Concrete".to_string());
    service
        .bind_visual_response(handle, "visual.concrete".to_string())
        .unwrap();
    service
        .bind_acoustic_profile(handle, "audio.concrete".to_string())
        .unwrap();

    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(!coverage.physical_complete);

    service
        .assign_material_archetype(handle, "mat.archetype.concrete".to_string())
        .unwrap();
    service
        .bind_surface_family(handle, "surface.wall.concrete".to_string())
        .unwrap();
    service
        .bind_response_profile(handle, "response.wall.concrete".to_string())
        .unwrap();
    service
        .bind_physical_response_family(handle, "matresp.physical.contact.concrete".to_string())
        .unwrap();

    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(coverage.physical_complete);
}

#[test]
fn test_runtime_rung_is_typed_and_updates_coverage() {
    let mut service = MaterialAuthoringService::new();
    let handle = service.create_material("Wood".to_string());

    service
        .set_cheap_runtime_rung(handle, RuntimeCheapnessRung::CachedLocal)
        .unwrap();

    let coverage = service.get_coverage().get(&handle).unwrap();
    assert!(coverage.runtime_complete);
}

#[test]
fn test_bind_material_family_and_response_profile() {
    let mut service = MaterialAuthoringService::new();
    let handle = service.create_material("Asphalt".to_string());

    service
        .bind_material_family(handle, "surface.road.asphalt".to_string())
        .unwrap();
    service
        .set_response_profile(handle, "response.road.asphalt".to_string())
        .unwrap();

    let profile = service.get_registry().get_profile(handle).unwrap();
    assert_eq!(
        profile.surface_family_ref.as_deref(),
        Some("surface.road.asphalt")
    );
    assert_eq!(
        profile.response_profile_ref.as_deref(),
        Some("response.road.asphalt")
    );
}

#[test]
fn test_assign_texture_and_collect_diagnostics() {
    let mut service = MaterialAuthoringService::new();
    let handle = service.create_material("Mud".to_string());
    service
        .set_cheap_runtime_rung(handle, RuntimeCheapnessRung::CachedLocal)
        .unwrap();
    service
        .assign_texture(handle, TextureSlot::Albedo, "mud_albedo.png".to_string())
        .unwrap();
    service
        .assign_texture(handle, TextureSlot::Normal, "mud_normal.dds".to_string())
        .unwrap();

    let diagnostics = service.get_diagnostics(handle).unwrap();
    assert_eq!(diagnostics.textures.len(), 2);
    assert_eq!(
        diagnostics
            .textures
            .get(&TextureSlot::Albedo)
            .map(String::as_str),
        Some("mud_albedo.png")
    );
    assert_eq!(diagnostics.cheapness.texture_count, 2);
}

#[test]
fn test_validate_cheapness_requires_runtime_rung() {
    let mut service = MaterialAuthoringService::new();
    let handle = service.create_material("Concrete".to_string());
    service
        .assign_texture(
            handle,
            TextureSlot::Albedo,
            "concrete_albedo.png".to_string(),
        )
        .unwrap();

    let report = service.validate_cheapness(handle).unwrap();
    assert!(!report.passes);
    assert!(report
        .issues
        .iter()
        .any(|issue| issue.contains("cheap runtime rung")));
}
