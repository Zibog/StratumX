# Hydrology Inventory Leak Flow And Persistence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own hydrology as inventory-carrying world truth: fill, leak, seep, absorb, overflow, evaporate, freeze, contaminate, and restore.

## Exact truth objects
- `FluidInventoryLedger`
- `ContainerFillBand`
- `LeakGeometryLedger`
- `FlowRateLedger`
- `RainIntakeLedger`
- `EvaporationLedger`
- `ContaminationBand`
- `FreezeStateLedger`

## Exact state machine
`empty -> filling -> stable -> leaking / overflowing / seeping -> evaporating / freezing -> restored`

## Exact law
- leak legality depends on leak geometry relative to current fill height;
- rain fill and outflow are integrated in the same mass ledger;
- seep and absorption are distinct from open leak and overflow;
- persistence must restore mass, contamination, freeze state, and active leak openings.

## Failure families
- `hydrology.mass_balance_break`
- `hydrology.leak_geometry_missing`
- `hydrology.rain_fill_desync`
- `hydrology.restore_state_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
