# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| scalar_types | Numeric primitives | Required | Foundation for all numeric operations | engine_core |
| vector_types | Vector primitives | Required | 2D, 3D, 4D vector operations | engine_core |
| matrix_types | Matrix primitives | Required | Transform and projection matrices | engine_core |
| transform_types | Transform primitives | Required | Spatial transformations | engine_core |
| geometry_types | Geometry primitives | Required | Basic geometric structures | engine_core |
| identifier_types | Core identifiers | Required | POD-like identifier wrappers | engine_core |
| value_wrappers | Value containers | Required | Primitive descriptors | engine_core |
| error_classes | Error taxonomy | Required | Canonical error hierarchy | engine_core |
| result_patterns | Result types | Required | Failure typing for contracts | engine_core |
| contract_traits | Foundational traits | Required | Minimal cross-stack contract surface | engine_core |
| feature_flags | Compile-time flags | Required | Legal capability sets only | engine_core |

## Invariant Rules

- Core contracts contain tiny contracts only: POD-like types, invariants, feature legality, error taxonomy hooks, and descriptor traits
- No convenience wrappers allowed
- No runtime-owned branching policy
- No hidden allocation policy
- No service ownership
- Feature flags select legal capability sets only
- Feature flags may not smuggle allocator policy, runtime ownership, or scene-policy shortcuts
- Flags affecting execution law, memory law, replay law, or startup legality require explicit canonical documentation
- Foundation layer has no dependencies on other engine crates
