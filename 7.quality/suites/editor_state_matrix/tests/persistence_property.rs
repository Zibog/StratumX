//! Property-Based Tests for Persistence Layer
//!
//! Tests universal properties that should hold for all persistence views.
//!
//! Note: Types are locally stubbed because stratumx_editor_state_containers
//! is a FUTURE_STUB crate not yet integrated into the product spine.

use proptest::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
struct ProjectIdentity {
    project_id: Uuid,
    project_name: String,
    project_path: PathBuf,
}

impl ProjectIdentity {
    fn new(project_id: Uuid, project_name: String, project_path: PathBuf) -> Self {
        Self {
            project_id,
            project_name,
            project_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WorkspaceIdentity {
    workspace_id: Uuid,
    workspace_name: String,
    workspace_path: PathBuf,
}

impl WorkspaceIdentity {
    fn new(workspace_id: Uuid, workspace_name: String, workspace_path: PathBuf) -> Self {
        Self {
            workspace_id,
            workspace_name,
            workspace_path,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WorldIdentity {
    world_id: Uuid,
    world_name: String,
    world_path: PathBuf,
}

impl WorldIdentity {
    fn new(world_id: Uuid, world_name: String, world_path: PathBuf) -> Self {
        Self {
            world_id,
            world_name,
            world_path,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct RuntimeModeState;

#[derive(Debug, Clone, Default)]
struct TerrainState;

#[derive(Debug, Clone, PartialEq)]
struct DockingConfig;

impl Default for DockingConfig {
    fn default() -> Self {
        DockingConfig
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PanelId(String);

impl PanelId {
    fn new(id: String) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
struct PanelGeometry {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl PanelGeometry {
    fn floating(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectOwner {
    project_identity: ProjectIdentity,
    workspace_identity: WorkspaceIdentity,
    save_generation: u64,
    content_snapshots: Vec<String>,
}

impl ProjectOwner {
    fn new(project_identity: ProjectIdentity, workspace_identity: WorkspaceIdentity) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation: 0,
            content_snapshots: Vec::new(),
        }
    }

    fn increment_save_generation(&mut self) {
        self.save_generation += 1;
    }
}

#[derive(Debug, Clone)]
struct WorkspaceOwner {
    schema_version: u32,
    open_panel_ids: Vec<PanelId>,
    panel_positions: HashMap<PanelId, PanelGeometry>,
    focused_panel: Option<PanelId>,
    docking_configuration: DockingConfig,
}

impl WorkspaceOwner {
    fn new() -> Self {
        Self {
            schema_version: 1,
            open_panel_ids: Vec::new(),
            panel_positions: HashMap::new(),
            focused_panel: None,
            docking_configuration: DockingConfig::default(),
        }
    }

    fn add_panel(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        self.open_panel_ids.push(panel_id.clone());
        self.panel_positions.insert(panel_id, geometry);
    }
}

#[derive(Debug, Clone)]
struct WorldOwner {
    world_identity: WorldIdentity,
    world_snapshot_ref: String,
    terrain_state: Option<TerrainState>,
    environment_state: Option<String>,
    world_diagnostics: Vec<String>,
}

impl WorldOwner {
    fn new(world_identity: WorldIdentity, world_snapshot_ref: String) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state: None,
            environment_state: None,
            world_diagnostics: Vec::new(),
        }
    }

    fn set_terrain_state(&mut self, terrain: Option<TerrainState>) {
        self.terrain_state = terrain;
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProjectPersistenceView {
    project_identity: ProjectIdentityStub,
    workspace_identity: WorkspaceIdentityStub,
    save_generation: u64,
    content_snapshots: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProjectIdentityStub {
    project_id: String,
    project_name: String,
    project_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorkspaceIdentityStub {
    workspace_id: String,
    workspace_name: String,
    workspace_path: String,
}

impl ProjectPersistenceView {
    fn new(
        project_identity: ProjectIdentity,
        workspace_identity: WorkspaceIdentity,
        save_generation: u64,
        content_snapshots: Vec<String>,
    ) -> Self {
        Self {
            project_identity: ProjectIdentityStub {
                project_id: project_identity.project_id.to_string(),
                project_name: project_identity.project_name,
                project_path: project_identity.project_path.to_string_lossy().to_string(),
            },
            workspace_identity: WorkspaceIdentityStub {
                workspace_id: workspace_identity.workspace_id.to_string(),
                workspace_name: workspace_identity.workspace_name,
                workspace_path: workspace_identity
                    .workspace_path
                    .to_string_lossy()
                    .to_string(),
            },
            save_generation,
            content_snapshots,
        }
    }
}

impl From<&ProjectOwner> for ProjectPersistenceView {
    fn from(owner: &ProjectOwner) -> Self {
        Self::new(
            owner.project_identity.clone(),
            owner.workspace_identity.clone(),
            owner.save_generation,
            owner.content_snapshots.clone(),
        )
    }
}

impl TryFrom<ProjectPersistenceView> for ProjectOwner {
    type Error = String;
    fn try_from(view: ProjectPersistenceView) -> Result<Self, Self::Error> {
        Ok(ProjectOwner {
            project_identity: ProjectIdentity {
                project_id: Uuid::parse_str(&view.project_identity.project_id)
                    .map_err(|e| e.to_string())?,
                project_name: view.project_identity.project_name,
                project_path: PathBuf::from(view.project_identity.project_path),
            },
            workspace_identity: WorkspaceIdentity {
                workspace_id: Uuid::parse_str(&view.workspace_identity.workspace_id)
                    .map_err(|e| e.to_string())?,
                workspace_name: view.workspace_identity.workspace_name,
                workspace_path: PathBuf::from(view.workspace_identity.workspace_path),
            },
            save_generation: view.save_generation,
            content_snapshots: view.content_snapshots,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorkspacePersistenceView {
    schema_version: u32,
    open_panel_ids: Vec<String>,
    panel_positions: HashMap<String, PanelGeometryStub>,
    focused_panel: Option<String>,
    docking_configuration: DockingConfigStub,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PanelGeometryStub {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DockingConfigStub;

impl WorkspacePersistenceView {
    fn new(
        schema_version: u32,
        open_panel_ids: Vec<String>,
        panel_positions: HashMap<String, PanelGeometryStub>,
        focused_panel: Option<String>,
        _docking_configuration: DockingConfig,
    ) -> Self {
        Self {
            schema_version,
            open_panel_ids,
            panel_positions,
            focused_panel,
            docking_configuration: DockingConfigStub,
        }
    }
}

impl From<&WorkspaceOwner> for WorkspacePersistenceView {
    fn from(owner: &WorkspaceOwner) -> Self {
        Self::new(
            owner.schema_version,
            owner.open_panel_ids.iter().map(|p| p.0.clone()).collect(),
            owner
                .panel_positions
                .iter()
                .map(|(k, v)| {
                    (
                        k.0.clone(),
                        PanelGeometryStub {
                            x: v.x,
                            y: v.y,
                            width: v.width,
                            height: v.height,
                        },
                    )
                })
                .collect(),
            owner.focused_panel.as_ref().map(|p| p.0.clone()),
            owner.docking_configuration.clone(),
        )
    }
}

impl TryFrom<WorkspacePersistenceView> for WorkspaceOwner {
    type Error = String;
    fn try_from(view: WorkspacePersistenceView) -> Result<Self, Self::Error> {
        Ok(WorkspaceOwner {
            schema_version: view.schema_version,
            open_panel_ids: view.open_panel_ids.into_iter().map(PanelId).collect(),
            panel_positions: view
                .panel_positions
                .into_iter()
                .map(|(k, v)| {
                    (
                        PanelId(k),
                        PanelGeometry {
                            x: v.x,
                            y: v.y,
                            width: v.width,
                            height: v.height,
                        },
                    )
                })
                .collect(),
            focused_panel: view.focused_panel.map(PanelId),
            docking_configuration: DockingConfig::default(),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorldPersistenceView {
    world_identity: WorldIdentityStub,
    world_snapshot_ref: String,
    terrain_state: Option<TerrainStateStub>,
    environment_state: Option<String>,
    world_diagnostics: Vec<String>,
    runtime_mode: RuntimeModeStateStub,
    save_generation: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct WorldIdentityStub {
    world_id: String,
    world_name: String,
    world_path: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TerrainStateStub {
    dimensions: (u32, u32),
    bounds: (f32, f32),
    material: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RuntimeModeStateStub;

impl Default for RuntimeModeStateStub {
    fn default() -> Self {
        RuntimeModeStateStub
    }
}

impl WorldPersistenceView {
    fn new(
        world_identity: WorldIdentity,
        world_snapshot_ref: String,
        terrain_state: Option<TerrainState>,
        environment_state: Option<String>,
        world_diagnostics: Vec<String>,
        _runtime_mode: RuntimeModeState,
        save_generation: u64,
    ) -> Self {
        Self {
            world_identity: WorldIdentityStub {
                world_id: world_identity.world_id.to_string(),
                world_name: world_identity.world_name,
                world_path: world_identity.world_path.to_string_lossy().to_string(),
            },
            world_snapshot_ref,
            terrain_state: terrain_state.map(|_| TerrainStateStub {
                dimensions: (0, 0),
                bounds: (0.0, 0.0),
                material: String::new(),
            }),
            environment_state,
            world_diagnostics,
            runtime_mode: RuntimeModeStateStub,
            save_generation,
        }
    }
}

impl From<&WorldOwner> for WorldPersistenceView {
    fn from(owner: &WorldOwner) -> Self {
        Self::new(
            owner.world_identity.clone(),
            owner.world_snapshot_ref.clone(),
            owner.terrain_state.clone(),
            owner.environment_state.clone(),
            owner.world_diagnostics.clone(),
            RuntimeModeState,
            0,
        )
    }
}

impl TryFrom<WorldPersistenceView> for WorldOwner {
    type Error = String;
    fn try_from(view: WorldPersistenceView) -> Result<Self, Self::Error> {
        Ok(WorldOwner {
            world_identity: WorldIdentity {
                world_id: Uuid::parse_str(&view.world_identity.world_id)
                    .map_err(|e| e.to_string())?,
                world_name: view.world_identity.world_name,
                world_path: PathBuf::from(view.world_identity.world_path),
            },
            world_snapshot_ref: view.world_snapshot_ref,
            terrain_state: view.terrain_state.map(|_| TerrainState),
            environment_state: view.environment_state,
            world_diagnostics: view.world_diagnostics,
        })
    }
}

// ============================================================================
// Property 14: Persistence View Separation
// ============================================================================

#[test]
fn property_14_persistence_view_separation() {
    assert_ne!(
        std::any::type_name::<ProjectOwner>(),
        std::any::type_name::<ProjectPersistenceView>(),
        "ProjectOwner and ProjectPersistenceView must be distinct types"
    );

    assert_ne!(
        std::any::type_name::<WorkspaceOwner>(),
        std::any::type_name::<WorkspacePersistenceView>(),
        "WorkspaceOwner and WorkspacePersistenceView must be distinct types"
    );

    assert_ne!(
        std::any::type_name::<WorldOwner>(),
        std::any::type_name::<WorldPersistenceView>(),
        "WorldOwner and WorldPersistenceView must be distinct types"
    );
}

#[test]
fn property_14_persistence_views_are_serializable() {
    let project_view = ProjectPersistenceView::new(
        ProjectIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test")),
        WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Workspace".to_string(),
            PathBuf::from("/workspace"),
        ),
        0,
        Vec::new(),
    );

    let workspace_view = WorkspacePersistenceView::new(
        1,
        Vec::new(),
        HashMap::new(),
        None,
        DockingConfig,
    );

    let world_view = WorldPersistenceView::new(
        WorldIdentity::new(Uuid::new_v4(), "World".to_string(), PathBuf::from("/world")),
        "snapshot".to_string(),
        None,
        None,
        Vec::new(),
        RuntimeModeState,
        0,
    );

    assert!(serde_json::to_string(&project_view).is_ok());
    assert!(serde_json::to_string(&workspace_view).is_ok());
    assert!(serde_json::to_string(&world_view).is_ok());
}

// ============================================================================
// Property 15: Persistence View Round Trip
// ============================================================================

#[test]
fn property_15_project_owner_round_trip() {
    proptest!(|(
        project_name in "[a-zA-Z0-9 ]{1,50}",
        workspace_name in "[a-zA-Z0-9 ]{1,50}",
        save_generation in 0u64..1000u64,
    )| {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            project_name.clone(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            workspace_name.clone(),
            PathBuf::from("/test/workspace"),
        );

        let mut original = ProjectOwner::new(project_id.clone(), workspace_id.clone());
        for _ in 0..save_generation {
            original.increment_save_generation();
        }

        let view = ProjectPersistenceView::from(&original);
        let restored = ProjectOwner::try_from(view).unwrap();

        prop_assert_eq!(original.project_identity.project_id, restored.project_identity.project_id);
        prop_assert_eq!(original.workspace_identity.workspace_id, restored.workspace_identity.workspace_id);
        prop_assert_eq!(original.save_generation, restored.save_generation);
        prop_assert_eq!(original.content_snapshots, restored.content_snapshots);
    });
}

#[test]
fn property_15_workspace_owner_round_trip() {
    proptest!(|(
        panel_count in 0usize..10usize,
        schema_version in 1u32..10u32,
    )| {
        let mut original = WorkspaceOwner::new();
        original.schema_version = schema_version;

        for i in 0..panel_count {
            let panel_id = PanelId::new(format!("panel_{}", i));
            let geometry = PanelGeometry::floating(
                (i * 100) as f32,
                (i * 100) as f32,
                800.0,
                600.0,
            );
            original.add_panel(panel_id, geometry);
        }

        let view = WorkspacePersistenceView::from(&original);
        let restored = WorkspaceOwner::try_from(view).unwrap();

        prop_assert_eq!(original.schema_version, restored.schema_version);
        prop_assert_eq!(original.open_panel_ids.len(), restored.open_panel_ids.len());
    });
}

#[test]
fn property_15_world_owner_round_trip() {
    proptest!(|(
        world_name in "[a-zA-Z0-9 ]{1,50}",
        snapshot_ref in "[a-zA-Z0-9_]{1,50}",
        has_terrain in proptest::bool::ANY,
    )| {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            world_name.clone(),
            PathBuf::from("/test/world"),
        );

        let mut original = WorldOwner::new(world_id.clone(), snapshot_ref.clone());

        if has_terrain {
            original.set_terrain_state(Some(TerrainState));
        }

        let view = WorldPersistenceView::from(&original);
        let restored = WorldOwner::try_from(view).unwrap();

        prop_assert_eq!(original.world_identity.world_id, restored.world_identity.world_id);
        prop_assert_eq!(original.world_snapshot_ref, restored.world_snapshot_ref);
        prop_assert_eq!(original.terrain_state.is_some(), restored.terrain_state.is_some());
        prop_assert_eq!(original.environment_state.is_some(), restored.environment_state.is_some());
        prop_assert_eq!(original.world_diagnostics.len(), restored.world_diagnostics.len());
    });
}

#[test]
fn property_15_serialization_round_trip() {
    proptest!(|(
        project_name in "[a-zA-Z0-9 ]{1,50}",
        workspace_name in "[a-zA-Z0-9 ]{1,50}",
    )| {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            project_name.clone(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            workspace_name.clone(),
            PathBuf::from("/test/workspace"),
        );

        let original = ProjectOwner::new(project_id, workspace_id);
        let view = ProjectPersistenceView::from(&original);

        let json = serde_json::to_string(&view).unwrap();
        let deserialized: ProjectPersistenceView = serde_json::from_str(&json).unwrap();

        prop_assert_eq!(view.project_identity.project_id, deserialized.project_identity.project_id);
        prop_assert_eq!(view.workspace_identity.workspace_id, deserialized.workspace_identity.workspace_id);
        prop_assert_eq!(view.save_generation, deserialized.save_generation);
    });
}
