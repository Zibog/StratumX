# StratumX Game Engine

**Version:** 0.1.0
**Stack:** SX-CANON/1.0.24/STACK-v30

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

This is the **only** production editor launch command.
The `desktop` feature and `--gui` flag are required.
Without `--gui`, the app runs validation/headless mode instead of the desktop editor.

### Launch Engine Runtimes

```bash
tools/engine-headless.sh  # Headless runtime
tools/engine-realtime.sh  # Realtime runtime
tools/stack-runtime.sh    # Stack runtime
```

### Verify Quality

All supported quality commands are now accessible via workspace:

```bash
# Format check + quality verify
tools/verify.sh  # or cargo run -p stratumx_quality_tasks -- verify

# Run smoke tests
tools/smoke.sh   # or cargo run -p stratumx_quality_tasks -- smoke

# Run full test suite
tools/full.sh    # or cargo run -p stratumx_quality_tasks -- full

# Run benchmarks
tools/bench.sh   # or cargo run -p stratumx_quality_tasks -- bench

# Generate metrics
tools/metrics.sh # or cargo run -p stratumx_quality_tasks -- metrics

# Generate evidence
tools/evidence.sh # or cargo run -p stratumx_quality_tasks -- evidence

# Generate inventory (includes root cleanliness check)
tools/inventory.sh # or cargo run -p stratumx_quality_tasks -- inventory

# Check layer boundaries (architectural discipline)
tools/check-layer-boundaries.sh # or tools/check-layer-boundaries.ps1

# Run comprehensive health check
tools/doctor.sh  # or cargo run -p stratumx_quality_tasks -- doctor
```

**Doctor Health Check** includes:
1. Workspace truth validation
2. Root cleanliness check
3. Format check (cargo fmt)
4. Lint check (cargo clippy)
5. Test suite (cargo test)
6. Quality verify
7. Smoke tests

---

## Entrypoints

StratumX provides several application entrypoints:

### Editor
- **Script:** `tools/editor.sh` / `tools/editor.ps1`
- **Direct:** `cargo run -p stratumx_editor_app --features desktop -- --gui`
- **Purpose:** Production editor with desktop GUI
- **Requirements:** `desktop` feature and `--gui` flag required

### Engine Runtimes
- **Headless:** `tools/engine-headless.sh` / `tools/engine-headless.ps1`
  - Runs engine without graphics (for servers, CI/CD)
- **Realtime:** `tools/engine-realtime.sh` / `tools/engine-realtime.ps1`
  - Runs engine with realtime rendering
- **Stack Runtime:** `tools/stack-runtime.sh` / `tools/stack-runtime.ps1`
  - Full stack runtime with all systems

### Quality Commands
All quality commands are documented in the "Verify Quality" section above.

---

## Architecture

StratumX is organized into layered packages:

- **1.docs/** - Canonical documentation and architecture (120+ canon documents)
- **2.engine/** - Core engine runtime (33 packages: ECS, world, physics, rendering, ~13.3k LOC)
- **3.sdk/** - SDK layer (9 packages: DTOs, handles, compatibility, ~4.6k LOC)
- **4.tooling/** - Tooling runtime (7 packages: preview, validation, build, ~9.2k LOC)
- **5.editor/** - Editor systems (39 packages: shell, viewport, authoring suites, ~32.9k LOC)
- **6.apps/** - Thin host/bootstrap only (5 packages: editor app, engine apps, ~738 LOC)
- **7.quality/** - Quality assurance (35 packages: test suites, harnesses, ~98k LOC)
- **tools/** - Convenience wrapper scripts

**Workspace Members:** 128 active packages
**Total Active Code:** ~60k LOC (excluding quality)

All packages are now part of the unified workspace for consistent build and verification.

---

## Documentation

Canonical documentation is in `1.docs/canonical/`:

- `00_INDEX.md` - Documentation index
- `01_SCOPE.md` - Project scope
- `02_STACK_MAP.md` - Stack architecture
- `03_PACKAGE_ROLE_MAP.md` - Package responsibilities
- `04_GLOBAL_DEPENDENCY_MODEL.md` - Dependency rules
- And 90+ more canonical documents

---

## Canonical DTO Law

All editor-engine communication uses frozen DTOs from:

```
3.sdk/l5.9-editor-dto-law
```

**Single source of truth. No drift. No TypeScript mirrors.**

---

## Development Status

**Current Focus:** 100% Canon Code Coverage Execution

The project is executing a 10-phase stabilization plan:
- Phase A: Workspace truth (COMPLETE)
- Phase B: Root command surface and meta hygiene (IN PROGRESS)
- Phases C-J: Engine/SDK/Tooling/Editor massive code closure

See `STRATUMX_100_PERCENT_CANON_CODE_COVERAGE_EXECUTION_PLAN.md` for details.

---

## Workspace Truth

**Workspace Model:** Full Truth (Option 1)

All active packages are included in the unified workspace with no exclusions:
- **2.engine:** 33 packages (L-0.05 through L4)
- **3.sdk:** 9 packages (L5 contract wall)
- **4.tooling:** 7 packages (L6 orchestration)
- **5.editor:** 35 packages (L7-L11 authoring surface)
  - L7: Command spine (1 package)
  - L8: Editor systems (11 packages)
  - L9: Authoring suites (12 packages)
  - L10: Editor services (8 packages)
  - L11: Collaboration surfaces (6 packages)
- **6.apps:** 5 packages (thin hosts only)
- **7.quality:** 39 packages (test suites and support)

**Total:** 128 workspace members
**Exclusions:** None (honest topology)

All packages are accessible via standard `cargo` commands from root.
All root commands (verify, full, bench, metrics, evidence, inventory, doctor) work without `--manifest-path` workarounds.

### Workspace Validation

To verify workspace integrity:

```bash
tools/validate-workspace.sh  # Linux/macOS
tools/validate-workspace.ps1 # Windows
```

Or run as part of doctor checks:

```bash
tools/doctor.sh  # Includes workspace validation
```

---

## License

MIT OR Apache-2.0

---

## Contributing

StratumX follows strict canonical architecture:
1. Read relevant docs in `1.docs/canonical/`
2. Follow layer boundaries (no upward dependencies)
3. Use frozen DTO law (`3.sdk/l5.9-editor-dto-law`)
4. Respect spine freeze policy

