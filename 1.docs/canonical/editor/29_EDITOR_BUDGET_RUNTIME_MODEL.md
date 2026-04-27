# Editor Budget Runtime Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Budget categories
- shell and UI frame cost
- primary viewport frame cost
- diagnostics and overlay cost
- background import/index/build cost
- preview/runtime bridge cost
- terrain presentation cost
- sky/environment presentation cost
- runtime attach/detach cost

## Core law
The editor must remain usable under load by degrading non-authoritative presentation first.
It must never recover performance by inventing fake authoritative state.

## First product-result budget priorities
The first closure protects these before everything else:
- open world identity remains visible
- primary viewport remains responsive
- terrain remains visible and navigable
- sky/environment remains plausibly present through legal bindings
- play/simulate controls remain honest
- diagnostics remain visible

## Degradation ladder
### First to degrade
- secondary viewports
- expensive overlays
- dense diagnostics histories
- heavy non-focused suites
- cinematic polish overlays

### Second to degrade
- environment fidelity tiers
- terrain detail overlays
- preview cadence for non-focused panels
- non-critical chrome refresh

### Must never silently degrade
- world identity
- world-open success/failure posture
- runtime-entry accept/deny posture
- terrain binding truth
- sky/environment binding truth
- viewport posture class

## Mandatory budget posture classes
- `Normal`
- `Reduced`
- `Degraded`
- `Recovery`

## Product rule
The operator must know when the editor is degraded.
A beautiful lie is worse than an ugly truth.
