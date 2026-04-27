pub use engine_runtime::RuntimeProfile;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BridgeVersion {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl BridgeVersion {
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeHandle(pub u64);

impl RuntimeHandle {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub fn opaque_tag(&self) -> String {
        format!("runtime_{:016x}", self.0)
    }
}

impl std::fmt::Debug for RuntimeHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<opaque:{}>", self.opaque_tag())
    }
}

impl std::fmt::Display for RuntimeHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionHandle(pub u64);

impl SessionHandle {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub fn opaque_tag(&self) -> String {
        format!("session_{:016x}", self.0)
    }
}

impl std::fmt::Debug for SessionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<opaque:{}>", self.opaque_tag())
    }
}

impl std::fmt::Display for SessionHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Session({})", self.0)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectHandle(pub u64);

impl ObjectHandle {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub fn opaque_tag(&self) -> String {
        format!("object_{:016x}", self.0)
    }
}

impl std::fmt::Debug for ObjectHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<opaque:{}>", self.opaque_tag())
    }
}

impl std::fmt::Display for ObjectHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ObjectClass {
    World,
    Scene,
    Entity,
    Material,
    Terrain,
    Logic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Capability {
    Snapshots,
    Observations,
    Metrics,
    Controls,
    ArtifactRefs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityProfile {
    ToolRuntime,
    EditorSurface,
    Automation,
    Diagnostics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompatibilityVerdict {
    Compatible,
    MissingCapability(Capability),
    VersionTooOld,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegalityVerdict {
    Legal,
    Illegal(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportLane {
    OrderedControl,
    BoundedPreview,
    MetricsOnly,
    ArtifactOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketLane {
    Control,
    Diagnostics,
    Metrics,
    Artifact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportPolicy {
    pub lane: TransportLane,
    pub max_payload_bytes: usize,
    pub max_batch_size: usize,
    pub compression_enabled: bool,
}

pub fn default_transport_policy(lane: TransportLane) -> TransportPolicy {
    match lane {
        TransportLane::OrderedControl => TransportPolicy {
            lane,
            max_payload_bytes: 1024,
            max_batch_size: 1,
            compression_enabled: false,
        },
        TransportLane::BoundedPreview => TransportPolicy {
            lane,
            max_payload_bytes: 64,
            max_batch_size: 4,
            compression_enabled: true,
        },
        TransportLane::MetricsOnly => TransportPolicy {
            lane,
            max_payload_bytes: 128,
            max_batch_size: 64,
            compression_enabled: true,
        },
        TransportLane::ArtifactOnly => TransportPolicy {
            lane,
            max_payload_bytes: 512,
            max_batch_size: 16,
            compression_enabled: false,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BridgeControlKind {
    SetField { key: String, value: String },
    AddTag { tag: String },
    Pause,
    Resume,
    Reset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeControl {
    pub session: SessionHandle,
    pub sequence: u64,
    pub object: Option<ObjectHandle>,
    pub kind: BridgeControlKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgePacket {
    pub session: SessionHandle,
    pub sequence: u64,
    pub lane: PacketLane,
    pub topic: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateClass {
    Snapshot,
    Transient,
    Persistent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRef {
    pub tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateRef {
    pub tag: String,
    pub class: StateClass,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectView {
    pub handle: ObjectHandle,
    pub class: ObjectClass,
    pub label: String,
    pub session: SessionHandle,
    pub identity_ref: IdentityRef,
    pub state_ref: StateRef,
    pub fields: BTreeMap<String, String>,
    pub tags: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationRecord {
    pub timestamp: u64,
    pub session: SessionHandle,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationBatch {
    pub records: Vec<ObservationRecord>,
    pub next_cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricRecord {
    pub name: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricBatch {
    pub records: Vec<MetricRecord>,
    pub next_cursor: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSnapshotObject {
    pub handle: ObjectHandle,
    pub class: ObjectClass,
    pub label: String,
    pub fields: BTreeMap<String, String>,
    pub tags: BTreeSet<String>,
    pub state_ref: StateRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeSnapshot {
    pub epoch: u64,
    pub label: String,
    pub session_count: usize,
    pub object_count: usize,
    pub objects: Vec<BridgeSnapshotObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaunchPlan {
    pub runtime_pack_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeProjection {
    pub snapshot_epoch: u64,
    pub launch_plan: LaunchPlan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub runtime: RuntimeHandle,
    pub version: BridgeVersion,
    pub max_queue_depth: usize,
    pub max_objects_per_snapshot: usize,
}

impl Default for BridgeConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeHandle::new(1),
            version: BridgeVersion::new(1, 0, 0),
            max_queue_depth: 1024,
            max_objects_per_snapshot: 8192,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupConfig {
    pub runtime_profile: RuntimeProfile,
    pub enable_physics: bool,
    pub enable_audio: bool,
    pub enable_networking: bool,
}

#[derive(Debug, Clone)]
pub struct BridgeObject {
    pub handle: ObjectHandle,
    pub class: ObjectClass,
    pub label: String,
    pub session: SessionHandle,
    pub fields: BTreeMap<String, String>,
    pub tags: BTreeSet<String>,
}

#[derive(Debug, Clone)]
pub struct BridgeRuntime {
    pub config: BridgeConfig,
    pub next_session: u64,
    pub next_object: u64,
    pub next_epoch: u64,
    pub sessions: BTreeMap<SessionHandle, String>,
    pub objects: BTreeMap<ObjectHandle, BridgeObject>,
    pub observations: Vec<ObservationRecord>,
    pub metrics: Vec<MetricRecord>,
    pub snapshots: Vec<BridgeSnapshot>,
}
