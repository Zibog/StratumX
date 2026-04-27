# Dependency Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Allowed dependency direction
- `L6 -> L5`
- `L6A -> L6`
- `L6A -> bounded L7A planning services`
- `L7 -> L6` through compiled campaigns, task bundles, governance bundles, automation bundles, and reporting requests only
- `L7A -> L6A`
- `L7A -> bounded L7 context services`

## Forbidden dependency direction
- `L6 -> L6A` for hidden assistant ownership
- `L6 -> L7` for hidden orchestration ownership
- `L6 -> L7A` for hidden planning ownership
- `L6 -> editor` back-dependency for product state ownership
- `L6A -> engine` direct access
- `L7 -> engine` direct access
- `L7A -> L6` direct transaction ownership
- `L7A -> engine` direct access

## Dependency law
Upper layers may request, plan, validate, optimize, or orchestrate, but only `L6` may own editor mutation authority.
The only always-hot dependency path is `engine/L5 -> L6`.
`L7` and `L7A` are cold compiled-control layers and must stay off ordinary frame-level editor work.

## Editor-facing interpretation
- `editor/` reads projections from `L6 snapshot/index/derived/stream/artifact` surfaces;
- `editor/` lowers mutations into `L6 command_envelopes` and `transaction_ledger`;
- validation and preview remain `L6` runtimes;
- import/reimport/bake/build/release remain background runtimes owned below the product shell;
- `workspace_runtime` exposes session-safe refs and activation scopes only, never product-owned layout/panel/selection state;
- assistant UI may never smuggle direct writes around `L6`.
