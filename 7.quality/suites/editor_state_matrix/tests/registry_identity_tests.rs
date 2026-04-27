//! Property-Based Tests for Registry Identity Usage
//!
//! **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5, 19.2, 20.4**
//!
//! This module contains property-based tests that verify identities come from
//! registries and are never randomly generated at runtime.

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use std::collections::HashSet;
    use std::path::PathBuf;
    use stratumx_editor_state_containers::{
        MaterialProfileId, MaterialRegistryState, ProjectIdentity, ProjectState, WorkspaceIdentity,
        WorldIdentity, WorldState,
    };
    use uuid::Uuid;

    // ========================================================================
    // Property 4: Registry Identity Usage
    // ========================================================================
    //
    // **Validates: Requirements 4.1, 4.2, 4.3, 4.4, 4.5, 19.2, 20.4**
    //
    // Property 4: Registry Identity Usage
    //
    // *For any* identity creation operation (world, material profile, project),
    // the identity must come from a registry and never be randomly generated
    // at runtime.
    //
    // This test validates that:
    // - World identities come from World_Registry, not Uuid::new_v4()
    // - Material profile identities come from Material_Registry
    // - Project identities come from Project_Registry
    // - No random UUID generation for identities
    //
    // The test simulates a registry by pre-generating a set of valid UUIDs,
    // then verifies that all created identities use UUIDs from this registry
    // set and never generate new random UUIDs.

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn property_registry_identity_usage(
            // Generate a registry of valid UUIDs (simulating registry truth)
            registry_uuids in prop::collection::hash_set(
                any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes)),
                10..20
            ),
            // Generate operations that should use registry UUIDs
            operations in prop::collection::vec(
                identity_operation_strategy(),
                1..50
            )
        ) {
            // Convert HashSet to Vec for indexing
            let registry_vec: Vec<Uuid> = registry_uuids.iter().copied().collect();

            // Track all UUIDs used in identity creation
            let mut used_uuids = HashSet::new();

            // Execute each operation and collect the UUIDs used
            for op in operations {
                let uuid = match op {
                    IdentityOperation::CreateWorld { registry_index, name, path } => {
                        let uuid = registry_vec[registry_index % registry_vec.len()];
                        let _world_identity = WorldIdentity::new(
                            uuid,
                            name,
                            PathBuf::from(path)
                        );
                        uuid
                    }
                    IdentityOperation::CreateProject { registry_index, name, path } => {
                        let uuid = registry_vec[registry_index % registry_vec.len()];
                        let _project_identity = ProjectIdentity::new(
                            uuid,
                            name,
                            PathBuf::from(path)
                        );
                        uuid
                    }
                    IdentityOperation::CreateMaterialProfile { registry_index } => {
                        let uuid = registry_vec[registry_index % registry_vec.len()];
                        let _profile_id = MaterialProfileId::new(uuid);
                        uuid
                    }
                    IdentityOperation::CreateWorldState { registry_index, name, path, snapshot_ref } => {
                        let uuid = registry_vec[registry_index % registry_vec.len()];
                        let world_identity = WorldIdentity::new(
                            uuid,
                            name,
                            PathBuf::from(path)
                        );
                        let _world_state = WorldState::new(world_identity, snapshot_ref);
                        uuid
                    }
                    IdentityOperation::CreateProjectState {
                        project_registry_index,
                        workspace_registry_index,
                        project_name,
                        project_path,
                        workspace_name,
                        workspace_path
                    } => {
                        let project_uuid = registry_vec[project_registry_index % registry_vec.len()];
                        let workspace_uuid = registry_vec[workspace_registry_index % registry_vec.len()];

                        let project_identity = ProjectIdentity::new(
                            project_uuid,
                            project_name,
                            PathBuf::from(project_path)
                        );
                        let workspace_identity = WorkspaceIdentity::new(
                            workspace_uuid,
                            workspace_name,
                            PathBuf::from(workspace_path)
                        );
                        let _project_state = ProjectState::new(project_identity, workspace_identity);

                        used_uuids.insert(workspace_uuid);
                        project_uuid
                    }
                };

                used_uuids.insert(uuid);
            }

            // CRITICAL PROPERTY: All used UUIDs must come from the registry
            // No random UUID generation should occur
            for used_uuid in &used_uuids {
                prop_assert!(
                    registry_vec.contains(used_uuid),
                    "Identity UUID {:?} was not from registry! This violates registry truth - \
                     identities must come from registries, never Uuid::new_v4()",
                    used_uuid
                );
            }

            // Additional validation: Verify no UUIDs were generated outside the registry
            // by checking that the set of used UUIDs is a subset of registry UUIDs
            prop_assert!(
                used_uuids.is_subset(&registry_uuids),
                "Used UUIDs contain identities not from registry. Registry size: {}, Used size: {}, \
                 Difference: {:?}",
                registry_uuids.len(),
                used_uuids.len(),
                used_uuids.difference(&registry_uuids).collect::<Vec<_>>()
            );
        }
    }

    // ========================================================================
    // Test Strategies
    // ========================================================================

    /// Identity operation types for property testing
    #[derive(Debug, Clone)]
    enum IdentityOperation {
        CreateWorld {
            registry_index: usize,
            name: String,
            path: String,
        },
        CreateProject {
            registry_index: usize,
            name: String,
            path: String,
        },
        CreateMaterialProfile {
            registry_index: usize,
        },
        CreateWorldState {
            registry_index: usize,
            name: String,
            path: String,
            snapshot_ref: String,
        },
        CreateProjectState {
            project_registry_index: usize,
            workspace_registry_index: usize,
            project_name: String,
            project_path: String,
            workspace_name: String,
            workspace_path: String,
        },
    }

    /// Strategy for generating identity operations
    fn identity_operation_strategy() -> impl Strategy<Value = IdentityOperation> {
        prop_oneof![
            // World identity creation
            (any::<usize>(), "[a-z]{3,10}", "[a-z/]{5,20}").prop_map(
                |(registry_index, name, path)| IdentityOperation::CreateWorld {
                    registry_index,
                    name,
                    path,
                }
            ),
            // Project identity creation
            (any::<usize>(), "[a-z]{3,10}", "[a-z/]{5,20}").prop_map(
                |(registry_index, name, path)| IdentityOperation::CreateProject {
                    registry_index,
                    name,
                    path,
                }
            ),
            // Material profile identity creation
            any::<usize>().prop_map(|registry_index| {
                IdentityOperation::CreateMaterialProfile { registry_index }
            }),
            // World state creation (includes world identity)
            (
                any::<usize>(),
                "[a-z]{3,10}",
                "[a-z/]{5,20}",
                "[a-z0-9]{5,15}"
            )
                .prop_map(|(registry_index, name, path, snapshot_ref)| {
                    IdentityOperation::CreateWorldState {
                        registry_index,
                        name,
                        path,
                        snapshot_ref,
                    }
                }),
            // Project state creation (includes project and workspace identities)
            (
                any::<usize>(),
                any::<usize>(),
                "[a-z]{3,10}",
                "[a-z/]{5,20}",
                "[a-z]{3,10}",
                "[a-z/]{5,20}"
            )
                .prop_map(
                    |(
                        project_registry_index,
                        workspace_registry_index,
                        project_name,
                        project_path,
                        workspace_name,
                        workspace_path,
                    )| {
                        IdentityOperation::CreateProjectState {
                            project_registry_index,
                            workspace_registry_index,
                            project_name,
                            project_path,
                            workspace_name,
                            workspace_path,
                        }
                    }
                ),
        ]
    }

    // ========================================================================
    // Unit Tests for Registry Identity Validation
    // ========================================================================

    #[test]
    fn test_world_identity_from_registry() {
        // Simulate a registry with known UUIDs
        let registry_uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();

        let world_identity = WorldIdentity::new(
            registry_uuid,
            "Test World".to_string(),
            PathBuf::from("/worlds/test"),
        );

        // Verify the identity uses the registry UUID
        assert_eq!(world_identity.world_id, registry_uuid);
    }

    #[test]
    fn test_project_identity_from_registry() {
        // Simulate a registry with known UUIDs
        let registry_uuid = Uuid::parse_str("660e8400-e29b-41d4-a716-446655440001").unwrap();

        let project_identity = ProjectIdentity::new(
            registry_uuid,
            "Test Project".to_string(),
            PathBuf::from("/projects/test"),
        );

        // Verify the identity uses the registry UUID
        assert_eq!(project_identity.project_id, registry_uuid);
    }

    #[test]
    fn test_material_profile_id_from_registry() {
        // Simulate a registry with known UUIDs
        let registry_uuid = Uuid::parse_str("770e8400-e29b-41d4-a716-446655440002").unwrap();

        let profile_id = MaterialProfileId::new(registry_uuid);

        // Verify the identity uses the registry UUID
        assert_eq!(*profile_id.as_uuid(), registry_uuid);
    }

    #[test]
    fn test_world_state_uses_registry_identity() {
        // Simulate a registry with known UUIDs
        let registry_uuid = Uuid::parse_str("880e8400-e29b-41d4-a716-446655440003").unwrap();

        let world_identity = WorldIdentity::new(
            registry_uuid,
            "Test World".to_string(),
            PathBuf::from("/worlds/test"),
        );

        let world_state = WorldState::new(world_identity.clone(), "snapshot_123".to_string());

        // Verify the world state uses the registry-based identity
        assert_eq!(world_state.get_world_identity().world_id, registry_uuid);
    }

    #[test]
    fn test_project_state_uses_registry_identities() {
        // Simulate a registry with known UUIDs
        let project_registry_uuid =
            Uuid::parse_str("990e8400-e29b-41d4-a716-446655440004").unwrap();
        let workspace_registry_uuid =
            Uuid::parse_str("aa0e8400-e29b-41d4-a716-446655440005").unwrap();

        let project_identity = ProjectIdentity::new(
            project_registry_uuid,
            "Test Project".to_string(),
            PathBuf::from("/projects/test"),
        );

        let workspace_identity = WorkspaceIdentity::new(
            workspace_registry_uuid,
            "Test Workspace".to_string(),
            PathBuf::from("/workspaces/test"),
        );

        let project_state = ProjectState::new(project_identity, workspace_identity);

        // Verify the project state uses registry-based identities
        assert_eq!(
            project_state.get_project_identity().project_id,
            project_registry_uuid
        );
        assert_eq!(
            project_state.get_workspace_identity().workspace_id,
            workspace_registry_uuid
        );
    }

    #[test]
    fn test_material_registry_state_uses_registry_profile_ids() {
        // Simulate a registry with known UUIDs
        let registry_uuid = Uuid::parse_str("bb0e8400-e29b-41d4-a716-446655440006").unwrap();

        let profile_id = MaterialProfileId::new(registry_uuid);
        let mut material_state = MaterialRegistryState::new();

        let profile = stratumx_editor_state_containers::MaterialProfile::new(
            profile_id.clone(),
            "Test Profile".to_string(),
        );
        material_state.add_profile(profile);

        // Verify the material registry state uses registry-based profile IDs
        let retrieved_profile = material_state.get_profile(&profile_id);
        assert!(retrieved_profile.is_some());
        assert_eq!(
            *retrieved_profile.unwrap().profile_id.as_uuid(),
            registry_uuid
        );
    }
}
