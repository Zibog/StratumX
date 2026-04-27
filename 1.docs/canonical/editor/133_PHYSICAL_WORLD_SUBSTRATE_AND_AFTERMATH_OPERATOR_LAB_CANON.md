# Physical World Substrate And Aftermath Operator Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Unify field substrate, deformation, topology, hydrology, fire/smoke, and fragment aftermath in one operator surface.

## Mandatory tabs
- field stack
- material pairs
- terrain deformation
- structural topology
- hydrology
- fire / smoke
- fragment aftermath

## Mandatory overlays
- wetness / heat / smoke overlay
- deformation mask overlay
- support graph overlay
- leak path overlay
- fragment settle overlay

## Required inspector fields
`field_scope_id`, `material_pair_row_ref`, `deformation_class`, `topology_ref`, `hydrology_state_ref`, `ignition_state_ref`, `fragment_aftermath_ref`

## Disabled reasons
`FLD_DISABLED_NO_WORLD`, `TRN_DISABLED_PATCH_LOCKED`, `DST_DISABLED_NO_TOPOLOGY_BIND`, `HYD_DISABLED_NO_CONTAINER`, `FIR_DISABLED_NO_IGNITABLE_TARGET`

## Capture hooks
The lab must be able to hand off capture and compare directly to editor `123`, `124` and tooling `90–92`.
