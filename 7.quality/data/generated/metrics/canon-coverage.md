# Canon Coverage Metrics

## Summary

- **Canonical packages with code owner**: 100.0% (14/14)
- **Heavy domains with full coverage**: 33.3% (1/3)
- **Vertical slices closed end-to-end**: 12.5% (1/8)

## Missing Components

- **Heavy Domain**: Material System (missing: engine)
- **Heavy Domain**: Terrain System (missing: engine)
- **Vertical Slice**: Physical substrate (terrain, water, atmosphere) (missing: engine)
- **Vertical Slice**: Living runtime (entities, AI, animation) (missing: engine, sdk, tooling, editor)
- **Vertical Slice**: Photoreal proof (rendering, lighting, materials) (missing: engine)
- **Vertical Slice**: Audio reality (spatial audio, propagation) (missing: engine, sdk)
- **Vertical Slice**: Animation and procedural interaction (missing: engine, sdk, tooling)
- **Vertical Slice**: Fur/cloth/wetness simulation (missing: engine, sdk, tooling, editor)
- **Vertical Slice**: Release/capture/freeze workflows (missing: engine, sdk)


## Details

### Canonical Packages
✓ l-0.05-world-region (Engine)
✓ l-0.1-world-spatial (Engine)
✓ l-0.2-ecs-assembly (Engine)
✓ l-0.3-ecs-query (Engine)
✓ l-0.4-ecs-registry (Engine)
✓ l0.5-shared-world-property-substrate (Engine)
✓ l5.0-link-ingress-packets (Sdk)
✓ l5.2-link-egress-observations (Sdk)
✓ l5.9-legality-gates (Sdk)
✓ l6.0-authority-core (Tooling)
✓ l6.1-command-envelopes (Tooling)
✓ l7.0-editor-command-spine (Editor)
✓ l8.0-editor-shell (Editor)
✓ l9.0-world-authoring-suite (Editor)


### Heavy Domains
✓ World Scale (engine:✓ sdk:✓ tooling:✓ editor:✓)
✗ Material System (engine:✗ sdk:✓ tooling:✓ editor:✓)
✗ Terrain System (engine:✗ sdk:✓ tooling:✓ editor:✓)


### Vertical Slices
✓ World-scale geodesy and coordinates (engine:✓ sdk:✓ tooling:✓ editor:✓)
✗ Physical substrate (terrain, water, atmosphere) (engine:✗ sdk:✓ tooling:✓ editor:✓)
✗ Living runtime (entities, AI, animation) (engine:✗ sdk:✗ tooling:✗ editor:✗)
✗ Photoreal proof (rendering, lighting, materials) (engine:✗ sdk:✓ tooling:✓ editor:✓)
✗ Audio reality (spatial audio, propagation) (engine:✗ sdk:✗ tooling:✓ editor:✓)
✗ Animation and procedural interaction (engine:✗ sdk:✗ tooling:✗ editor:✓)
✗ Fur/cloth/wetness simulation (engine:✗ sdk:✗ tooling:✗ editor:✗)
✗ Release/capture/freeze workflows (engine:✗ sdk:✗ tooling:✓ editor:✓)

