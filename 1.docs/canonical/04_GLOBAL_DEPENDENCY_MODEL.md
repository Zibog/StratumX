# Global Dependency Model

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Direct Dependency Law

Only the following **direct package dependencies** are legal in the active canonical stack.
Everything else is forbidden as a direct dependency, even if the same lower-stack truth is visible through an indirect relay.

| Dependent package | Direct dependency package | Direct dependency status | Legal use |
|---|---|---|---|
| `sdk` | `engine` | legal | typed bridge intake only |
| `tooling` | `sdk` | legal | plane/runtime/orchestration intake from L5 only |
| `editor` | `tooling` | legal | authoring-surface intake from tooling only |

## Indirect Relay Visibility Law

The following lower-stack visibility is legal **only as relay visibility** and must never be modeled as a direct package dependency.

| Observer package | Lower package | Direct dependency status | Legal relay path |
|---|---|---|---|
| `tooling` | `engine` | forbidden direct / legal indirect | `tooling -> sdk -> engine` |
| `editor` | `sdk` | forbidden direct / legal indirect | `editor -> tooling -> sdk` |
| `editor` | `engine` | forbidden direct / legal indirect | `editor -> tooling -> sdk -> engine` |

## Illegal Direct Dependencies

The following direct dependencies are prohibited at umbrella level:

- `editor -> engine`
- `editor -> sdk`
- `tooling -> engine`
- `sdk -> tooling`
- `sdk -> editor`
- `engine -> sdk`
- `engine -> tooling`
- `engine -> editor`
- `tooling -> editor`

## Relay Direction Rules

1. Truth relay is upward only: `engine -> sdk -> tooling -> editor`.
2. Control relay is downward only through legal interfaces: `editor -> tooling -> sdk -> engine`.
3. Indirect relay visibility is not the same thing as a direct dependency.
4. No package may bypass the relay chain to read lower-stack truth.
5. No package may own truth that belongs to another package.
6. `sdk` is a typed bridge, not a second truth owner.

## Exact Direct Dependency Matrix

| From \ To | `engine` | `sdk` | `tooling` | `editor` |
|---|---|---|---|---|
| `engine` | - | X | X | X |
| `sdk` | L | - | X | X |
| `tooling` | X | L | - | X |
| `editor` | X | X | L | - |

Where:
- `L` = legal **direct** dependency
- `X` = illegal direct dependency
- `-` = self edge / not applicable

## Relay Visibility Matrix

| Consumer | May observe lower-stack truth? | Condition |
|---|---|---|
| `sdk` | yes | only from `engine` through declared bridge interfaces |
| `tooling` | yes | only from `engine` through `sdk` relay surfaces |
| `editor` | yes | only from `engine` / `sdk` truth through `tooling` relay surfaces |

## Closed Inter-Package Contract Anchors

Umbrella-level legal crossing anchors are limited to the following package-root authorities:

- `engine -> sdk`: `sdk/16_BOUNDARY_AUTHORITY.md`, `sdk/26_SHARED_TYPE_REGISTRY.md`, `sdk/31_ENGINE_L4_BINDING_MAP.md`, `sdk/33_HANDLE_AND_REF_OPACITY_LAW.md`
- `sdk -> tooling`: `tooling/19_CROSS_LAYER_EXCHANGE_MODEL.md`, `tooling/22_L5_SYNCHRONIZATION_MODEL.md`, `tooling/26_SHARED_TYPE_REGISTRY.md`
- `tooling -> editor`: `editor/22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`, `editor/31_SHARED_TYPE_REGISTRY.md`, `editor/33_BOUNDARY_PRESERVATION_MATRIX.md`

Any other direct dependency claim or undeclared crossing surface is illegal until explicitly added here and in the affected package roots.
