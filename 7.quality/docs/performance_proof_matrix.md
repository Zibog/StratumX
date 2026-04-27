# Performance Proof Matrix

| Path | Trigger | Frequency | Owner truth touched | Cache touched | GPU touched | Expected cheap/expensive | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| editor startup | app boot | once | yes | yes | yes | expensive-once | pending |
| sky update | UI event / world change | event-driven | yes | maybe | yes | cheap-event | pending |
| terrain import | explicit import | rare | yes | yes | yes | expensive-event | pending |
| viewport frame | every frame | high | no truth mutation | read-only | yes | cheap-frame | pending |
| command flush | frame or queue event | bounded | maybe | no | no | bounded | pending |

---

## Performance Verification Checklist

### 1. Startup Path
- [ ] Startup world opens exactly once
- [ ] Terrain bind occurs once
- [ ] Environment bind occurs once
- [ ] Viewport warmup does not duplicate
- **Evidence:** Logs from `startup.*` markers

### 2. Sky Update Path
- [ ] `update_sky()` not called every frame without dirty flag
- [ ] UI slider does not cause 5-10 identical world updates
- [ ] Cloud update only on change
- **Evidence:** Logs from `sky.update.*` markers

### 3. Terrain GPU Sync
- [ ] `update_terrain_from_world(...)` not called every frame without changes
- [ ] Sync only on dirty terrain state
- **Evidence:** Logs from `terrain.gpu_sync.*` markers

### 4. Viewport Frame Path
- [ ] Frame path only reads state and renders
- [ ] No world mutation during frame
- [ ] No heavy derived view rebuilds every frame
- **Evidence:** Frame profiling data

### 5. Command Flush Path
- [ ] Batch is bounded
- [ ] No unnecessary serialization/cloning
- [ ] No full queue walk without necessity
- **Evidence:** Logs from `command.flush.*` markers

---

## Diagnostic Log Markers

### Startup Path
- `startup.begin`
- `startup.world_open`
- `startup.terrain_bound`
- `startup.environment_bound`
- `startup.viewport_ready`

### Sky Update Path
- `sky.update.requested`
- `sky.update.skipped_not_dirty`
- `sky.update.applied`

### Terrain Path
- `terrain.gpu_sync.requested`
- `terrain.gpu_sync.skipped_not_dirty`
- `terrain.gpu_sync.applied`

### Command Flush
- `command.flush.begin size=N`
- `command.flush.end processed=N`

---

## Key Metrics to Measure

1. **Startup duration** - Total time from app launch to ready
2. **Terrain GPU syncs on idle** - Should be 0
3. **Sky updates on idle** - Should be 0
4. **Command flushes per second in idle** - Should be minimal
5. **Frame-time stability in idle viewport** - Should be consistent

### Idle Verification
If idle viewport shows:
- ❌ Constant terrain sync
- ❌ Constant sky reapply
- ❌ Constant flush storm

Then optimization discipline is **not yet proven**.

---

## Where to Look for Waste

### Editor Host
- `state_queries.rs` - Check query layer, cache invalidation, repeated rebuilds
- `viewport_service.rs` - Check for repeated projections
- `terrain_service.rs` - Check for unnecessary syncs
- `environment_service.rs` - Check for redundant binds
- `startup.rs` - Check for duplicate initialization

### Desktop App
- `command_flush.rs` - Check batch size and frequency
- `viewport_panel.rs` - Check frame path purity
- `app_state.rs` - Check state mutation patterns

### State Containers
- Query layer efficiency
- Cache invalidation patterns
- Repeated rebuilds
- Repeated projections
