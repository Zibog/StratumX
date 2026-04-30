# Global Package Role Map

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Authority and Truth-Ownership Table

| Package | Authority role | Truth ownership | Allowed consumers | Forbidden consumer pattern |
|---|---|---|---|---|
| `engine` | Runtime truth authority | World state, physics state, entity identity, resource state, performance baselines | `sdk` (via L5 bridge only) | tooling, editor — may not read engine internals directly |
| `sdk` | Data-bridge authority | Public bridge types: link packets, controls, observations, metrics, compat verdicts, transport policies, legality gates, handles, refs | `tooling` (via L6 planes only) | editor — may not read sdk types directly |
| `tooling` | Derived-truth and orchestration authority | Transaction records, snapshots, index derivations, artifact manifests, stream/cache/budget plane state, assistant proposals | `editor` (via L6 planes and L7 orchestration refs only) | editor — may not read tooling internals or bypass planes |
| `editor` | Authoring surface authority | UI state, selection state, panel/view layout, active suite context, interaction routing, overlay/gizmo state | None downstream (terminal consumer) | May not own lower-stack truth or write to engine/sdk/tooling directly |

## Constitutional Authority Sets

| Package | Constitutional authority set | Role in umbrella law |
|---|---|---|
| `engine` | `engine/constitutions/*` | Highest package-local constitutional authority; may override umbrella root docs when a governed runtime domain is explicitly covered |
| `sdk` | `sdk/constitutions/*` | Package-local bridge law for L5 structure, type classes, opacity, non-interference, and upper-stack legality |
| `tooling` | `tooling/constitutions/*` | Package-local tooling law for L6/L6A/L7/L7A authority, family composition, orchestration, and memory/GPU/disk discipline |
| `editor` | `editor/constitutions/*` | Package-local editor law for product surfaces, suites, services, resource discipline, collaboration, and interaction legality |

## Role Boundaries

- `engine` owns **all** canonical runtime truth. No other package may declare a competing truth for engine-state.
- `sdk` owns **only** the public bridge type definitions. It does not own engine internals.
- `tooling` owns **only** derived, indexed, and orchestration state. It does not own engine-original truth.
- `editor` owns **only** authoring surface state. It does not own lower-stack truth of any kind.

## Type-Ownership Summary

| Type class | Owner | Next legal holder |
|---|---|---|
| Entity IDs, world region handles | engine | sdk (as engine-identity-refs) |
| Physics / simulation state | engine | sdk (as engine-state-refs) |
| Runtime artifact refs | engine | sdk (as engine-artifact-refs) |
| Link packets, controls | sdk (L5.0, L5.1) | tooling L6 authority core |
| Observations, metrics | sdk (L5.2, L5.3) | tooling L6 snapshot/stream planes |
| Transport policies, legality gates | sdk (L5.8, L5.9) | tooling L6 validation runtime |
| Handle opaques | sdk (L5.10–L5.12) | tooling as opaque refs only |
| Transaction records | tooling L6.2 | editor as read-only task results |
| Snapshot keys | tooling L6.3 | editor as read-only view refs |
| Index cursors | tooling L6.4 | editor as read-only cursor handles |
| Preview results | tooling L6.x | editor surfaces (non-owning) |
| Diagnostics events | tooling L6.x | editor diagnostics surface (non-owning) |
| Suite-level edit commands | editor L9 | tooling via legal command channels |
| UI state | editor L8 | None |
