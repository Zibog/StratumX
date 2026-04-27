# Boundary Preservation

## Canonical Rule

`engine_material` provides shared world property substrate. Material truth is cross-domain and read-heavy, consumed by simulation and synthesis families, not a regular world-progression family.

## Upward Boundary

**Exports to layers above:**
- Material descriptors (stable material identity, classes, property attachment)
- Property tables (physical, thermal, fluid, structural, acoustic, appearance properties)
- Response profiles (reaction rows for kinetics, field, acoustics, imaging)
- Lookup keys (parallel-safe material lookup)

**Canonical consumers:**
- `engine_kinetics` — Kinetics simulation
- `engine_field` — Field simulation
- `engine_imaging` — Imaging systems
- `engine_acoustics` — Acoustics systems

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable references for material instances
- `engine_world` — World-bound material context

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Response profiles mutated at runtime except through authoritative apply or legal content reload
- Duplicated ownership of response law by material consumers
- Descriptor chasing through long ownership chains in critical execution lanes
- Dynamic owner-chain walk for descriptor miss (one deterministic fallback step only)
- Nested descriptor chase for reaction miss (deterministic authored default only)
- Persistent hot-path miss ledger (diagnostics-only, optional)
- Consumer families rewriting lookup law
- Response-profile lookup not O(1)-like and cache-friendly
- Treating material as regular simulation family
