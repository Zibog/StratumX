// Terrain Authority Container - Type definitions

/// Lifecycle state of the terrain authority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainLifecycleState {
    Uninitialized,
    Initialized,
    Disposed,
}

/// Terrain registry state
#[derive(Default)]
pub struct TerrainRegistryState {
    /// Available terrain layers
    pub layers: Vec<TerrainLayer>,
    /// Terrain chunks
    pub chunks: Vec<TerrainChunk>,
    /// Heightmap data
    pub heightmap: Option<HeightmapData>,
}

/// Terrain layer definition
#[derive(Debug, Clone)]
pub struct TerrainLayer {
    pub id: u32,
    pub name: String,
    pub material_id: Option<String>,
    pub visible: bool,
}

/// Terrain chunk definition
#[derive(Debug, Clone)]
pub struct TerrainChunk {
    pub id: String,
    pub grid_pos: [i32; 2],
    pub resolution: u32,
    pub loaded: bool,
    pub dirty: bool,
}

/// Heightmap data
#[derive(Debug, Clone)]
pub struct HeightmapData {
    pub resolution: [u32; 2],
    pub data: Vec<f32>,
}

/// Brush state for terrain editing
#[derive(Debug, Clone, Default)]
pub struct BrushState {
    pub radius: f32,
    pub strength: f32,
    pub target_height: f32,
    pub target_layer: u32,
    pub continuous: bool,
}

/// Terrain Authority Container
///
/// Owns the terrain registry and manages its lifecycle.
/// Provides read-only query methods and routes all mutations through command spine.
pub struct TerrainAuthorityContainer {
    /// Terrain registry state
    pub registry: TerrainRegistryState,
    /// Current lifecycle state
    pub(super) lifecycle_state: TerrainLifecycleState,
    /// Brush state
    pub(super) brush_state: BrushState,
}

impl TerrainAuthorityContainer {
    /// Create a new TerrainAuthorityContainer
    pub fn new() -> Self {
        Self {
            registry: TerrainRegistryState::default(),
            lifecycle_state: TerrainLifecycleState::Uninitialized,
            brush_state: BrushState::default(),
        }
    }
}

impl Default for TerrainAuthorityContainer {
    fn default() -> Self {
        Self::new()
    }
}
