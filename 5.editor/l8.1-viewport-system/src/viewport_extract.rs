use serde::{Deserialize, Serialize};

// ============================================================================
// VIEWPORT FRAME STATE
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportFrameState {
    pub renderables: ViewportRenderableSnapshot,
    pub camera: ViewportCamera,
    pub overlays: Vec<ViewportOverlay>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportCamera {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub fov_deg: f32,
}

// ============================================================================
// RENDERABLE SNAPSHOT
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportRenderableSnapshot {
    pub terrain_patches: Vec<RenderableTerrainPatch>,
    pub walls: Vec<RenderableWall>,
    pub damage_overlays: Vec<RenderableDamageOverlay>,
    pub projectile_traces: Vec<RenderableProjectileTrace>,
    pub debris_proxies: Vec<RenderableDebrisProxy>,
    pub impact_markers: Vec<ImpactMarker>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableTerrainPatch {
    pub entity_id: u32,
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableWall {
    pub entity_id: u32,
    pub position: [f32; 3],
    pub dimensions: [f32; 3],
    pub base_color: [f32; 3],
    pub released_segments: Vec<ReleasedSegment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleasedSegment {
    pub layer_index: u8,
    pub segment_id: u16,
    pub position_offset: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableDamageOverlay {
    pub entity_id: u32,
    pub layer_index: u8,
    pub damage_type: DamageVisualizationType,
    pub position: [f32; 3],
    pub extent: [f32; 3],
    pub intensity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageVisualizationType {
    Crack,
    Crumble,
    Chip,
    Crater,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableProjectileTrace {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderableDebrisProxy {
    pub position: [f32; 3],
    pub size: f32,
    pub debris_type: DebrisType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebrisType {
    TileShard,
    PlasterDust,
    ConcreteChunk,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactMarker {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub energy_j: f32,
}

// ============================================================================
// VIEWPORT OVERLAY
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewportOverlay {
    SelectionBox {
        entity_id: u32,
        bounds: [f32; 6],
    },
    SegmentGrid {
        entity_id: u32,
        rows: u8,
        cols: u8,
    },
    ImpactDebug {
        position: [f32; 3],
        angle_deg: f32,
        energy_j: f32,
    },
}

// ============================================================================
// EXTRACT BUILDER
// ============================================================================

pub struct ViewportExtractBuilder {
    renderables: ViewportRenderableSnapshot,
    overlays: Vec<ViewportOverlay>,
}

impl ViewportExtractBuilder {
    pub fn new() -> Self {
        Self {
            renderables: ViewportRenderableSnapshot {
                terrain_patches: Vec::new(),
                walls: Vec::new(),
                damage_overlays: Vec::new(),
                projectile_traces: Vec::new(),
                debris_proxies: Vec::new(),
                impact_markers: Vec::new(),
            },
            overlays: Vec::new(),
        }
    }

    pub fn add_terrain(&mut self, terrain: RenderableTerrainPatch) {
        self.renderables.terrain_patches.push(terrain);
    }

    pub fn add_wall(&mut self, wall: RenderableWall) {
        self.renderables.walls.push(wall);
    }

    pub fn add_damage_overlay(&mut self, overlay: RenderableDamageOverlay) {
        self.renderables.damage_overlays.push(overlay);
    }

    pub fn add_projectile_trace(&mut self, trace: RenderableProjectileTrace) {
        self.renderables.projectile_traces.push(trace);
    }

    pub fn add_debris_proxy(&mut self, debris: RenderableDebrisProxy) {
        self.renderables.debris_proxies.push(debris);
    }

    pub fn add_impact_marker(&mut self, marker: ImpactMarker) {
        self.renderables.impact_markers.push(marker);
    }

    pub fn add_overlay(&mut self, overlay: ViewportOverlay) {
        self.overlays.push(overlay);
    }

    pub fn build(self, camera: ViewportCamera) -> ViewportFrameState {
        ViewportFrameState {
            renderables: self.renderables,
            camera,
            overlays: self.overlays,
        }
    }
}

impl Default for ViewportExtractBuilder {
    fn default() -> Self {
        Self::new()
    }
}
