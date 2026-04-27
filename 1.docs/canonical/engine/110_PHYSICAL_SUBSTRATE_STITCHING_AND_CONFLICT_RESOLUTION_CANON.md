# Physical Substrate Stitching And Conflict Resolution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Join material pairs, deformation, topology, hydrology, fire/smoke, and fragment aftermath into one execution order.

## Precedence ladder
1. owner material pair legality
2. topology legality
3. terrain deformation legality
4. fluid inventory legality
5. fire and smoke thresholds
6. fragment spawn/merge legality
7. derived visual/audio consequence publication

## Required consequence rows
| Trigger | Required downstream publication |
|---|---|
| crater or spall | nav / cover / traversal invalidation |
| topology breach | traversal class and debris host |
| leak start or stop | fluid inventory delta and sound-class trigger |
| ignition or extinguish | smoke/char state and visibility delta |
| fragment settle | traversal friction and debris-bed merge verdict |
