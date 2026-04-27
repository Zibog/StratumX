# Boundary Preservation

## Canonical Rule

`engine_core` provides minimal engine foundation. This crate stays at the bottom because every upper crate needs a shared language for types, contracts, errors, and math.

## Upward Boundary

**Exports to layers above:**
- Base types (core identifiers, value wrappers, primitive descriptors)
- Math primitives (scalar, vector, matrix, transform, geometry)
- Error types (canonical error classes, result patterns, failure typing)
- Marker traits (foundational contract surfaces)
- Feature flags (compile-time capability and profile flags)

**Canonical consumers:**
- All engine crates (identity, handle, storage, ECS, world, runtime, simulation, systems, startup)

## Downward Boundary

**Imports from layers below:**
- None (foundation layer)

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Convenience wrappers in core contracts
- Runtime-owned branching policy
- Hidden allocation policy
- Service ownership
- Feature flags smuggling allocator policy, runtime ownership, or scene-policy shortcuts
- Feature flags affecting execution law, memory law, replay law, or startup legality without explicit canonical documentation
- Any dependency on upper layers
