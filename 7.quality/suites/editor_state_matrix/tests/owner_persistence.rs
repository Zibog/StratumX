//! Tests for owner to_persistence_view methods
//!
//! Verifies that all owner containers can convert to persistence views correctly.
//!
//! Note: Types are locally stubbed because stratumx_editor_state_containers
//! is a FUTURE_STUB crate not yet integrated into the product spine.

use std::collections::HashMap;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PanelId(String);

impl PanelId {
    fn new(id: &str) -> Self {
        Self(id.to_string())
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

    fn get_bounds(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

#[derive(Debug, Clone, Default)]
struct DockingConfig;

impl DockingConfig {
    fn is_empty(&self) -> bool {
        true
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
    const CURRENT_SCHEMA_VERSION: u32 = 1;

    fn new() -> Self {
        Self {
            schema_version: Self::CURRENT_SCHEMA_VERSION,
            open_panel_ids: Vec::new(),
            panel_positions: HashMap::new(),
            focused_panel: None,
            docking_configuration: DockingConfig,
        }
    }

    fn add_panel(&mut self, panel_id: PanelId, geometry: PanelGeometry) {
        self.open_panel_ids.push(panel_id.clone());
        self.panel_positions.insert(panel_id, geometry);
    }

    fn set_focused_panel(&mut self, panel: Option<PanelId>) {
        self.focused_panel = panel;
    }

    fn has_docking_config(&self) -> bool {
        !self.docking_configuration.is_empty()
    }

    fn to_persistence_view(&self) -> WorkspacePersistenceView {
        WorkspacePersistenceView {
            schema_version: self.schema_version,
            open_panel_ids: self.open_panel_ids.iter().map(|p| p.0.clone()).collect(),
            focused_panel: self.focused_panel.as_ref().map(|p| p.0.clone()),
        }
    }
}

#[derive(Debug, Clone)]
struct WorkspacePersistenceView {
    schema_version: u32,
    open_panel_ids: Vec<String>,
    focused_panel: Option<String>,
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

#[derive(Debug, Clone)]
struct TerrainState {
    dimensions: (u32, u32),
    bounds: (f32, f32),
    material: String,
}

impl TerrainState {
    fn new(dimensions: (u32, u32), bounds: (f32, f32), material: String) -> Self {
        Self {
            dimensions,
            bounds,
            material,
        }
    }

    fn get_dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    fn get_bounds(&self) -> (f32, f32) {
        self.bounds
    }

    fn get_material(&self) -> &str {
        &self.material
    }
}

#[derive(Debug, Clone)]
struct WorldOwner {
    world_identity: WorldIdentity,
    world_snapshot_ref: String,
    terrain_state: Option<TerrainState>,
    environment_state: Option<String>,
}

impl WorldOwner {
    fn new(world_identity: WorldIdentity, world_snapshot_ref: String) -> Self {
        Self {
            world_identity,
            world_snapshot_ref,
            terrain_state: None,
            environment_state: None,
        }
    }

    fn set_terrain_state(&mut self, terrain: Option<TerrainState>) {
        self.terrain_state = terrain;
    }

    fn to_persistence_view(&self) -> WorldPersistenceView {
        WorldPersistenceView {
            world_identity: self.world_identity.clone(),
            world_snapshot_ref: self.world_snapshot_ref.clone(),
            terrain_state: self.terrain_state.is_some(),
            environment_state: self.environment_state.is_some(),
        }
    }
}

#[derive(Debug, Clone)]
struct WorldPersistenceView {
    world_identity: WorldIdentity,
    world_snapshot_ref: String,
    terrain_state: bool,
    environment_state: bool,
}

type EventCallback = Box<dyn Fn(&str)>;

struct ProjectOwner {
    project_identity: ProjectIdentity,
    workspace_identity: WorkspaceIdentity,
    save_generation: u64,
    content_snapshots: Vec<String>,
    #[allow(dead_code)]
    event_callback: Option<EventCallback>,
}

impl std::fmt::Debug for ProjectOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProjectOwner")
            .field("project_identity", &self.project_identity)
            .field("workspace_identity", &self.workspace_identity)
            .field("save_generation", &self.save_generation)
            .field("content_snapshots", &self.content_snapshots)
            .finish_non_exhaustive()
    }
}

impl Clone for ProjectOwner {
    fn clone(&self) -> Self {
        Self {
            project_identity: self.project_identity.clone(),
            workspace_identity: self.workspace_identity.clone(),
            save_generation: self.save_generation,
            content_snapshots: self.content_snapshots.clone(),
            event_callback: None,
        }
    }
}

impl ProjectOwner {
    fn new(project_identity: ProjectIdentity, workspace_identity: WorkspaceIdentity) -> Self {
        Self {
            project_identity,
            workspace_identity,
            save_generation: 0,
            content_snapshots: Vec::new(),
            event_callback: None,
        }
    }

    fn set_event_callback(&mut self, _cb: EventCallback) {
        self.event_callback = Some(_cb);
    }

    fn to_persistence_view(&self) -> ProjectPersistenceView {
        ProjectPersistenceView {
            project_identity: self.project_identity.clone(),
            workspace_identity: self.workspace_identity.clone(),
            save_generation: self.save_generation,
            content_snapshots: self.content_snapshots.clone(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ProjectPersistenceView {
    project_identity: ProjectIdentity,
    workspace_identity: WorkspaceIdentity,
    save_generation: u64,
    content_snapshots: Vec<String>,
}

#[test]
fn test_project_owner_to_persistence_view() {
    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let owner = ProjectOwner::new(project_id.clone(), workspace_id.clone());
    let view = owner.to_persistence_view();

    assert_eq!(view.project_identity.project_id, project_id.project_id);
    assert_eq!(
        view.workspace_identity.workspace_id,
        workspace_id.workspace_id
    );
    assert_eq!(view.save_generation, 0);
    assert!(view.content_snapshots.is_empty());
}

#[test]
fn test_workspace_owner_to_persistence_view() {
    let mut owner = WorkspaceOwner::new();
    let panel_id = PanelId::new("viewport");
    let geometry = PanelGeometry::floating(0.0, 0.0, 800.0, 600.0);

    owner.add_panel(panel_id.clone(), geometry.clone());
    owner.set_focused_panel(Some(panel_id.clone()));

    let view = owner.to_persistence_view();

    assert_eq!(view.schema_version, WorkspaceOwner::CURRENT_SCHEMA_VERSION);
    assert_eq!(view.open_panel_ids.len(), 1);
    assert_eq!(view.focused_panel, Some(panel_id.0));
    
    // Use geometry bounds
    let bounds = geometry.get_bounds();
    assert_eq!(bounds, (0.0, 0.0, 800.0, 600.0));
    
    // Use docking config
    assert!(!owner.has_docking_config());
}

#[test]
fn test_world_owner_to_persistence_view() {
    let world_id = WorldIdentity::new(
        Uuid::new_v4(),
        "Test World".to_string(),
        PathBuf::from("/test/world"),
    );

    let mut owner = WorldOwner::new(world_id.clone(), "snapshot_123".to_string());

    let terrain = TerrainState::new((1024, 1024), (1000.0, 1000.0), "default".to_string());
    
    // Use terrain getters
    assert_eq!(terrain.get_dimensions(), (1024, 1024));
    assert_eq!(terrain.get_bounds(), (1000.0, 1000.0));
    assert_eq!(terrain.get_material(), "default");
    
    owner.set_terrain_state(Some(terrain));

    let view = owner.to_persistence_view();

    assert_eq!(view.world_identity.world_id, world_id.world_id);
    assert_eq!(view.world_snapshot_ref, "snapshot_123");
    assert!(view.terrain_state);
    assert!(!view.environment_state);
}

#[test]
fn test_persistence_view_excludes_runtime_fields() {
    let project_id = ProjectIdentity::new(
        Uuid::new_v4(),
        "Test Project".to_string(),
        PathBuf::from("/test/project"),
    );
    let workspace_id = WorkspaceIdentity::new(
        Uuid::new_v4(),
        "Test Workspace".to_string(),
        PathBuf::from("/test/workspace"),
    );

    let mut owner = ProjectOwner::new(project_id, workspace_id);
    owner.set_event_callback(Box::new(|_| {}));

    let view = owner.to_persistence_view();

    let json = serde_json::to_string(&view).unwrap();
    assert!(!json.is_empty());

    let _deserialized: ProjectPersistenceView = serde_json::from_str(&json).unwrap();
}

#[test]
fn test_all_owners_have_to_persistence_view() {
    let project_id =
        ProjectIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));
    let workspace_id =
        WorkspaceIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));
    let world_id = WorldIdentity::new(Uuid::new_v4(), "Test".to_string(), PathBuf::from("/test"));

    let project_owner = ProjectOwner::new(project_id, workspace_id);
    let workspace_owner = WorkspaceOwner::new();
    let world_owner = WorldOwner::new(world_id, "snapshot".to_string());

    let _project_view = project_owner.to_persistence_view();
    let _workspace_view = workspace_owner.to_persistence_view();
    let _world_view = world_owner.to_persistence_view();
}
