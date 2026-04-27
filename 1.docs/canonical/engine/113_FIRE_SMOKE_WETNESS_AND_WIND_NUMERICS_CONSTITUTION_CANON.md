# Fire Smoke Wetness And Wind Numerics Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Close the numeric and transition law beneath fire/wetness/wind/smoke coupling.

## Ignition state machine
`cold -> preheated -> drying -> ignition_ready -> flaming -> rain_suppressed / wet_suppressed -> smoldering -> charred -> ash -> cooled`

## Numeric classes
- ignition thresholds by material response family;
- drying-before-burning law;
- heat-transfer classes: contact, radiant, carried ember, hot-gas exposure;
- wind-biased spread classes;
- smoke generation classes: light, heavy, oily, toxic, ash-rich;
- rain suppression and extinguish thresholds.

## Smoke law
Smoke carries density, lift class, settling tendency, and persistence horizon.
It may affect visibility, health, lighting, and sound only through declared substrate publications.

## Failure families
- `fire.numeric_threshold_missing`
- `fire.drying_transition_illegal`
- `fire.smoke_settle_desync`
- `fire.rain_extinguish_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
