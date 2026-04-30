use engine_core::{EngineCoreError, EngineCoreResult, Tick};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct WorldId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldRole {
    PrimaryRuntime,
    SecondaryRuntime,
    ValidationSandbox,
    ContentAssembly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChunkResidencyPolicy {
    AlwaysPinned,
    WarmStreamable,
    ColdStreamable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorldLineageKind {
    Created,
    Imported,
    Migrated,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldNamespaceRoots {
    pub terrain: String,
    pub materials: String,
    pub environment: String,
    pub placements: String,
    pub diagnostics: String,
    pub save_state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldStreamingProfile {
    pub residency_policy: ChunkResidencyPolicy,
    pub hot_radius_regions: u8,
    pub warm_radius_regions: u8,
    pub old_floor_degrade_posture: u8,
    pub streamed_out_chunks_restore_safe: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldLineageMetadata {
    pub kind: WorldLineageKind,
    pub revision: u32,
    pub last_lawful_mutation_tick: Tick,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldManifest {
    pub world_id: WorldId,
    pub world_label: String,
    pub world_role: WorldRole,
    pub namespace_roots: WorldNamespaceRoots,
    pub lineage: WorldLineageMetadata,
    pub streaming_profile: Option<WorldStreamingProfile>,
    pub requires_streaming_profile: bool,
}

impl WorldManifest {
    pub fn canonical_test_manifest() -> Self {
        Self {
            world_id: WorldId(1),
            world_label: "engine.test.world".to_string(),
            world_role: WorldRole::ValidationSandbox,
            namespace_roots: WorldNamespaceRoots {
                terrain: "terrain.root".to_string(),
                materials: "materials.root".to_string(),
                environment: "environment.root".to_string(),
                placements: "placements.root".to_string(),
                diagnostics: "diagnostics.root".to_string(),
                save_state: "save.root".to_string(),
            },
            lineage: WorldLineageMetadata {
                kind: WorldLineageKind::Created,
                revision: 1,
                last_lawful_mutation_tick: Tick(0),
            },
            streaming_profile: Some(WorldStreamingProfile {
                residency_policy: ChunkResidencyPolicy::WarmStreamable,
                hot_radius_regions: 1,
                warm_radius_regions: 2,
                old_floor_degrade_posture: 1,
                streamed_out_chunks_restore_safe: true,
            }),
            requires_streaming_profile: true,
        }
    }

    pub fn validate(&self) -> EngineCoreResult<()> {
        if self.world_id.0 == 0 {
            return Err(EngineCoreError::InvalidDescriptor(
                "world manifest requires non-zero world_id",
            ));
        }
        if self.world_label.trim().is_empty() {
            return Err(EngineCoreError::InvalidDescriptor(
                "world manifest requires non-empty world_label",
            ));
        }

        let roots = [
            self.namespace_roots.terrain.as_str(),
            self.namespace_roots.materials.as_str(),
            self.namespace_roots.environment.as_str(),
            self.namespace_roots.placements.as_str(),
            self.namespace_roots.diagnostics.as_str(),
            self.namespace_roots.save_state.as_str(),
        ];
        if roots.iter().any(|root| root.trim().is_empty()) {
            return Err(EngineCoreError::InvalidDescriptor(
                "world manifest namespace roots must be non-empty",
            ));
        }
        let unique_roots: BTreeSet<_> = roots.into_iter().collect();
        if unique_roots.len() != 6 {
            return Err(EngineCoreError::InvalidDescriptor(
                "world manifest namespace roots must be unique",
            ));
        }
        if self.requires_streaming_profile && self.streaming_profile.is_none() {
            return Err(EngineCoreError::InvalidDescriptor(
                "world manifest requires explicit streaming profile",
            ));
        }
        if let Some(profile) = &self.streaming_profile {
            if profile.hot_radius_regions == 0 {
                return Err(EngineCoreError::InvalidDescriptor(
                    "world streaming profile requires non-zero hot radius",
                ));
            }
            if profile.warm_radius_regions < profile.hot_radius_regions {
                return Err(EngineCoreError::InvalidDescriptor(
                    "world streaming warm radius must be greater than or equal to hot radius",
                ));
            }
            if profile.old_floor_degrade_posture == 0 {
                return Err(EngineCoreError::InvalidDescriptor(
                    "world streaming profile requires non-zero old-floor posture code",
                ));
            }
        }
        Ok(())
    }
}
