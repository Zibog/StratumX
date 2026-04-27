use crate::bridge_types::*;
use std::collections::BTreeMap;

impl BridgeRuntime {
    pub fn new(config: BridgeConfig) -> Self {
        Self {
            config,
            next_session: 1,
            next_object: 1,
            next_epoch: 0,
            sessions: BTreeMap::new(),
            objects: BTreeMap::new(),
            observations: Vec::new(),
            metrics: Vec::new(),
            snapshots: vec![BridgeSnapshot {
                epoch: 0,
                label: "initial".to_string(),
                session_count: 0,
                object_count: 0,
                objects: Vec::new(),
            }],
        }
    }

    pub fn open_session(&mut self, label: impl Into<String>) -> SessionHandle {
        let handle = SessionHandle::new(self.next_session);
        self.next_session += 1;
        self.sessions.insert(handle, label.into());
        handle
    }

    pub fn register_object(
        &mut self,
        label: impl Into<String>,
        class: ObjectClass,
    ) -> Result<ObjectHandle, String> {
        let session = self
            .sessions
            .keys()
            .next_back()
            .copied()
            .unwrap_or_else(|| self.open_session("auto_session"));
        self.register_object_in_session(session, class, label)
    }

    pub fn register_object_in_session(
        &mut self,
        session: SessionHandle,
        class: ObjectClass,
        label: impl Into<String>,
    ) -> Result<ObjectHandle, String> {
        if !self.sessions.contains_key(&session) {
            return Err("Session not found".to_string());
        }
        let handle = ObjectHandle::new(self.next_object);
        self.next_object += 1;
        self.objects.insert(
            handle,
            BridgeObject {
                handle,
                class,
                label: label.into(),
                session,
                fields: BTreeMap::new(),
                tags: Default::default(),
            },
        );
        Ok(handle)
    }

    pub fn apply_control(&mut self, control: BridgeControl) -> Result<(), String> {
        if !self.sessions.contains_key(&control.session) {
            return Err("Session not found".to_string());
        }

        match control.kind {
            BridgeControlKind::SetField { key, value } => {
                let handle = control.object.ok_or("Object required".to_string())?;
                let object = self
                    .objects
                    .get_mut(&handle)
                    .ok_or("Object not found".to_string())?;
                object.fields.insert(key.clone(), value);
                self.observations.push(ObservationRecord {
                    timestamp: self.next_epoch,
                    session: control.session,
                    message: format!("Field set on {}: {}", handle.opaque_tag(), key),
                });
            }
            BridgeControlKind::AddTag { tag } => {
                let handle = control.object.ok_or("Object required".to_string())?;
                let object = self
                    .objects
                    .get_mut(&handle)
                    .ok_or("Object not found".to_string())?;
                object.tags.insert(tag.clone());
                self.observations.push(ObservationRecord {
                    timestamp: self.next_epoch,
                    session: control.session,
                    message: format!("Tag added on {}: {}", handle.opaque_tag(), tag),
                });
            }
            BridgeControlKind::Pause => {
                self.observations.push(ObservationRecord {
                    timestamp: self.next_epoch,
                    session: control.session,
                    message: "Session paused".to_string(),
                });
            }
            BridgeControlKind::Resume => {
                self.observations.push(ObservationRecord {
                    timestamp: self.next_epoch,
                    session: control.session,
                    message: "Session resumed".to_string(),
                });
            }
            BridgeControlKind::Reset => {
                self.observations.push(ObservationRecord {
                    timestamp: self.next_epoch,
                    session: control.session,
                    message: "Session reset".to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn transport_legality(
        &self,
        policy: &TransportPolicy,
        payload_bytes: usize,
        metrics_ok: bool,
        artifacts_ok: bool,
    ) -> LegalityVerdict {
        if payload_bytes > policy.max_payload_bytes {
            return LegalityVerdict::Illegal("payload exceeds transport ceiling".to_string());
        }

        match policy.lane {
            TransportLane::OrderedControl => LegalityVerdict::Legal,
            TransportLane::BoundedPreview => {
                if artifacts_ok {
                    LegalityVerdict::Legal
                } else {
                    LegalityVerdict::Illegal("preview lane requires artifact refs".to_string())
                }
            }
            TransportLane::MetricsOnly => {
                if metrics_ok {
                    LegalityVerdict::Legal
                } else {
                    LegalityVerdict::Illegal("metrics lane requires metrics".to_string())
                }
            }
            TransportLane::ArtifactOnly => {
                if artifacts_ok {
                    LegalityVerdict::Legal
                } else {
                    LegalityVerdict::Illegal("artifact lane requires artifacts".to_string())
                }
            }
        }
    }

    pub fn estimated_hot_path_bytes(&self) -> usize {
        self.observations.len() * 64 + self.metrics.len() * 32
    }

    pub fn session_label(&self, handle: SessionHandle) -> Option<&str> {
        self.sessions.get(&handle).map(|s| s.as_str())
    }

    pub fn compatibility_verdict(
        &self,
        version: BridgeVersion,
        capabilities: &[Capability],
        profile: CompatibilityProfile,
    ) -> CompatibilityVerdict {
        if version.major == 0 {
            return CompatibilityVerdict::VersionTooOld;
        }

        let required = match profile {
            CompatibilityProfile::ToolRuntime => Some(Capability::Controls),
            CompatibilityProfile::EditorSurface => Some(Capability::Observations),
            CompatibilityProfile::Automation => Some(Capability::Metrics),
            CompatibilityProfile::Diagnostics => {
                if !capabilities.contains(&Capability::Observations) {
                    return CompatibilityVerdict::MissingCapability(Capability::Observations);
                }
                if !capabilities.contains(&Capability::Metrics) {
                    return CompatibilityVerdict::MissingCapability(Capability::Metrics);
                }
                None
            }
        };

        if let Some(capability) = required {
            if !capabilities.contains(&capability) {
                return CompatibilityVerdict::MissingCapability(capability);
            }
        }

        CompatibilityVerdict::Compatible
    }

    pub fn object_view(&self, handle: ObjectHandle) -> Option<ObjectView> {
        self.objects.get(&handle).map(|obj| ObjectView {
            handle: obj.handle,
            class: obj.class,
            label: obj.label.clone(),
            session: obj.session,
            identity_ref: IdentityRef {
                tag: obj.handle.opaque_tag(),
            },
            state_ref: StateRef {
                tag: format!("snapshot:{}", obj.handle.opaque_tag()),
                class: StateClass::Snapshot,
            },
            fields: obj.fields.clone(),
            tags: obj.tags.clone(),
        })
    }

    pub fn ingest_packet(&mut self, packet: BridgePacket) -> Result<(), String> {
        if !self.sessions.contains_key(&packet.session) {
            return Err("Session not found".to_string());
        }
        self.observations.push(ObservationRecord {
            timestamp: self.next_epoch,
            session: packet.session,
            message: format!("Packet ingested on {:?}: {}", packet.lane, packet.topic),
        });
        Ok(())
    }

    pub fn record_metric(&mut self, name: impl Into<String>, value: f64, unit: impl Into<String>) {
        self.metrics.push(MetricRecord {
            name: name.into(),
            value,
            unit: unit.into(),
        });
    }

    pub fn publish_snapshot(&mut self, label: impl Into<String>) -> Result<BridgeSnapshot, String> {
        self.next_epoch += 1;
        let objects = self
            .objects
            .values()
            .map(|object| BridgeSnapshotObject {
                handle: object.handle,
                class: object.class,
                label: object.label.clone(),
                fields: object.fields.clone(),
                tags: object.tags.clone(),
                state_ref: StateRef {
                    tag: format!(
                        "snapshot:{}:{}",
                        self.next_epoch,
                        object.handle.opaque_tag()
                    ),
                    class: StateClass::Snapshot,
                },
            })
            .collect::<Vec<_>>();
        let snapshot = BridgeSnapshot {
            epoch: self.next_epoch,
            label: label.into(),
            session_count: self.sessions.len(),
            object_count: self.objects.len(),
            objects,
        };
        self.snapshots.push(snapshot.clone());
        Ok(snapshot)
    }

    pub fn latest_snapshot(&self) -> BridgeSnapshot {
        self.snapshots
            .last()
            .cloned()
            .unwrap_or_else(|| BridgeSnapshot {
                epoch: 0,
                label: "empty".to_string(),
                session_count: 0,
                object_count: 0,
                objects: Vec::new(),
            })
    }

    pub fn read_observation_batch(&self, cursor: usize, limit: usize) -> ObservationBatch {
        let records = self
            .observations
            .iter()
            .skip(cursor)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let next_cursor = cursor + records.len();
        ObservationBatch {
            records,
            next_cursor,
        }
    }

    pub fn read_metric_batch(&self, cursor: usize, limit: usize) -> MetricBatch {
        let records = self
            .metrics
            .iter()
            .skip(cursor)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>();
        let next_cursor = cursor + records.len();
        MetricBatch {
            records,
            next_cursor,
        }
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

pub fn canonical_demo_startup_config(profile: RuntimeProfile) -> StartupConfig {
    StartupConfig {
        runtime_profile: profile,
        enable_physics: true,
        enable_audio: false,
        enable_networking: false,
    }
}

#[derive(Debug, Clone)]
pub struct EngineBridgeHarness {
    pub _startup: StartupConfig,
    pub bridge: BridgeRuntime,
    pub projection: BridgeProjection,
}

impl EngineBridgeHarness {
    pub fn new(startup: StartupConfig, config: BridgeConfig) -> Result<Self, String> {
        Ok(Self {
            _startup: startup,
            bridge: BridgeRuntime::new(config),
            projection: BridgeProjection {
                snapshot_epoch: 0,
                launch_plan: LaunchPlan {
                    runtime_pack_count: 1,
                },
            },
        })
    }

    pub fn bootstrap_demo_world(
        &mut self,
        segments: usize,
        passes: usize,
    ) -> Result<(ObjectHandle, BridgeSnapshot), String> {
        let session = self.bridge.open_session("demo_session");
        let world_handle =
            self.bridge
                .register_object_in_session(session, ObjectClass::World, "demo_world")?;
        self.bridge.observations.push(ObservationRecord {
            timestamp: self.bridge.next_epoch,
            session,
            message: format!(
                "Bootstrap demo world: segments={}, passes={}",
                segments, passes
            ),
        });
        let snapshot = self.bridge.publish_snapshot("world_bootstrapped")?;
        self.projection.snapshot_epoch = snapshot.epoch;
        Ok((world_handle, snapshot))
    }

    pub fn projection(&self) -> BridgeProjection {
        self.projection.clone()
    }

    pub fn bridge(&self) -> &BridgeRuntime {
        &self.bridge
    }

    pub fn bridge_mut(&mut self) -> &mut BridgeRuntime {
        &mut self.bridge
    }

    pub fn into_bridge(self) -> BridgeRuntime {
        self.bridge
    }
}
