# StratumX Game Engine

**Version:** 0.1.0
**Stack:** SX-CANON/1.0.28/STACK-v34

---

## Quick Start

### Clone the repository

```bash
git clone https://github.com/Zibog/StratumX.git
cd StratumX
```

### Launch Editor

```bash
tools/editor.sh  # Linux/macOS
tools/editor.ps1 # Windows
```

Or directly:

```bash
cargo run -p stratumx_editor_app --features desktop -- --gui
```

This is the production editor launch command.
The `desktop` feature and `--gui` flag are required.
Without `--gui`, the app runs validation/headless mode instead of the desktop editor.

### Launch Engine Runtimes

```bash
tools/engine-headless.sh  # Headless runtime
tools/engine-realtime.sh  # Realtime runtime
tools/stack-runtime.sh    # Stack runtime
```

### Verify Quality

All supported quality commands are available from workspace root:

```bash
tools/verify.sh
tools/smoke.sh
tools/full.sh
tools/bench.sh
tools/metrics.sh
tools/evidence.sh
tools/inventory.sh
tools/check-layer-boundaries.sh
tools/check-file-size-discipline.sh
tools/check-test-placement.ps1
tools/doctor.sh
```

## Workspace Validation Policy

`cargo build` may use engine-only `default-members`.
Full validation is always:

```bash
cargo check --workspace
cargo test --workspace
tools/full
```

---

## Architecture

StratumX is organized into layered packages:

- **1.docs/** - Canonical documentation and architecture
- **2.engine/** - Core engine runtime
- **3.sdk/** - SDK layer
- **4.tooling/** - Tooling runtime
- **5.editor/** - Editor systems
- **6.apps/** - Thin host/bootstrap only
- **7.quality/** - Quality assurance
- **tools/** - Convenience wrapper scripts

## Project Truth

- Current canon marker: `SX-CANON/1.0.28/STACK-v34`
- Workspace packages: 131 active Cargo packages, all included in `workspace.members`
- Layer dependency violations: must be 0
- Test policy: heavy/integration/property/matrix tests live under `7.quality`
- Current phase: gold-clean repository stabilization before new implementation sprint
- Next implementation sprint after cleanup: Native Graphics Port + asset/material pipeline seed

LOC counts are tracked by quality inventory tooling. Do not hand-maintain exact LOC numbers in README unless refreshed by `tools/doctor`.
Docs are ahead of code in graphics, assets, audio, netcode and dream-scene heavy domains. This is expected until implementation sprints catch up.

All active packages are part of the unified workspace. Use `--workspace` when you want full-repository validation instead of engine-only `default-members`.

---

## Documentation

Canonical documentation is in `1.docs/canonical/`:

- `00_INDEX.md` - Documentation index
- `01_SCOPE.md` - Project scope
- `02_STACK_MAP.md` - Stack architecture
- `03_PACKAGE_ROLE_MAP.md` - Package responsibilities
- `04_GLOBAL_DEPENDENCY_MODEL.md` - Dependency rules

Cleanup and status ledgers:

- [`CODE_CLEANUP_STATUS_LEDGER.md`](1.docs/developer_docs/CODE_CLEANUP_STATUS_LEDGER.md)
- [`CODE_VS_CANON_STATUS_LEDGER.md`](1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md)
- [`ACTIVE_EDITOR_PRODUCT_SPINE.md`](5.editor/ACTIVE_EDITOR_PRODUCT_SPINE.md)
- [`crate_status_ledger.md`](7.quality/inventory/crate_status_ledger.md)

---

## Canonical DTO Law

All editor-engine communication uses frozen DTOs from:

```text
3.sdk/l5.9-editor-dto-law
```

Single source of truth. No drift. No mirrors.

---

## Development Status

**Current Focus:** gold-clean repository stabilization before new implementation sprint

This pass is for cleanup only:

- workspace truth and root/meta cleanup
- test centralization into `7.quality`
- monolith splitting without behavior changes
- crate classification and honest ledgers

---

## Workspace Truth

**Workspace Model:** Full truth

All active packages are included in the unified workspace with no exclusions:

- Active Cargo packages: 131
- Workspace members: 131
- Packages outside workspace: 0
- Exclusions: None

The default workspace members are engine-focused for fast local iteration. Full validation uses `cargo check --workspace`, `cargo test --workspace`, and the `tools/*` command surface.

All packages are accessible via standard `cargo` commands from root, but root `cargo build` still follows engine-only `default-members`. Use `cargo check --workspace` and `cargo test --workspace` for full validation.

### Workspace Validation

To verify workspace integrity:

```bash
tools/validate-workspace.sh
tools/validate-workspace.ps1
```

---

## License

MIT OR Apache-2.0

---

## Contributing

StratumX follows strict canonical architecture:

1. Read relevant docs in `1.docs/canonical/`
2. Follow layer boundaries
3. Use frozen DTO law (`3.sdk/l5.9-editor-dto-law`)
4. Respect stabilization and spine discipline
