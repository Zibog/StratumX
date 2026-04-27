# StratumX Quality Contour

Single entry command:

```bash
cargo run -p stratumx_quality_tasks -- verify
```

Supported modes:

- `verify`
- `smoke`
- `full`
- `bench`
- `metrics`
- `evidence`

Phase 5 suite groups live under `7.quality/suites/` and shared helpers live under
`7.quality/support/`.

Primary Phase 5 suites:

- `editor_command_matrix`
- `editor_shell_matrix`
- `editor_state_matrix`
- `world_authoring_matrix`
- `terrain_authoring_matrix`
- `material_authoring_matrix`
- `environment_authoring_matrix`
- `audio_authoring_matrix`
- `build_release_matrix`
- `route_schema_golden`
- `forbidden_shortcuts`
- `focus_recovery_matrix`
- `proof_region_integration`
- `smoke`

---

## Phase 3: Quality Contour Normalization

This contour has undergone comprehensive normalization under Phase 3. All test files comply with established laws.

### Law Documents

All laws are documented in `docs/`:

| Document | Purpose |
|----------|---------|
| [SUITE_LAW.md](docs/SUITE_LAW.md) | Defines single role for each test suite |
| [FILE_SIZE_LAW.md](docs/FILE_SIZE_LAW.md) | File size limits (300 normal, 800+ mandatory split) |
| [NAMING_LAW.md](docs/NAMING_LAW.md) | Naming standards for files, functions, directories |
| [TASK_ROUTING_LAW.md](docs/TASK_ROUTING_LAW.md) | Test routing categories (verify/smoke/full) |
| [NEW_TEST_LAW.md](docs/NEW_TEST_LAW.md) | Process for adding new tests |

### Evidence Documents

| Document | Purpose |
|----------|---------|
| [INVENTORY.md](docs/INVENTORY.md) | Complete catalog of test files |
| [SPLIT_PLANS.md](docs/SPLIT_PLANS.md) | Plans for splitting giant files |
| [SUPPORT_CODE_ANALYSIS.md](docs/SUPPORT_CODE_ANALYSIS.md) | Embedded support code catalog |
| [SUPPORT_EXTRACTION_COMPLETION.md](docs/SUPPORT_EXTRACTION_COMPLETION.md) | Support extraction report |
| [DELETION_LOG.md](docs/DELETION_LOG.md) | Dead test deletion audit trail |
| [NORMALIZATION_EVIDENCE.md](docs/NORMALIZATION_EVIDENCE.md) | Complete normalization evidence |

### Compliance Status

- ✅ **File Size Law:** 0 mandatory violations (57 → 0)
- ✅ **Suite Law:** All 27 suites documented
- ✅ **Naming Law:** All files have semantic names
- ✅ **Routing Law:** All tests routed into verify/smoke/full
- ✅ **Dead Tests:** 0 ignored, 0 duplicates
- ✅ **Support Code:** Extracted and organized

### Automated Enforcement

File size check runs automatically:
```bash
cargo test -p repo_hygiene --test quality_file_size_check
```
