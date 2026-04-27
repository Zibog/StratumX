# Hydrology Container Leak Flow And Contamination Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own authoring and debug of container fill, leak geometry, rain intake, overflow, evaporation, contamination, and freeze.

## Mandatory tabs
- inventory and mass ledger
- leak geometry
- rain / dry interval
- contamination
- freeze / thaw
- restore replay

## Mandatory controls
- set fluid inventory;
- add or remove leak opening;
- inject rain or dry interval;
- set seep / overflow policy;
- toggle freeze conditions;
- inspect mass ledger;
- replay restore.

## Required overlays
- container fill overlay
- leak path overlay
- seep / overflow overlay
- contamination overlay
- freeze state overlay

## Required inspector fields
- `fluid_inventory_ref`
- `fill_ratio`
- `leak_geometry_ref`
- `inflow_rate`
- `outflow_rate`
- `contamination_class`
- `freeze_state`
- `mass_ledger_ref`

## Disabled reasons
`HYD_DISABLED_NO_CONTAINER`, `HYD_DISABLED_NO_WORLD_RAIN_BIND`, `HYD_DISABLED_LEAK_GEOMETRY_INVALID`, `HYD_DISABLED_RESTORE_SCOPE_MISSING`
