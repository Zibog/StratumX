# Play Simulate Debug Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- play in editor launch and stop
- simulate and step controls
- runtime attach and detach
- live property watch and runtime inspector
- component state diff between authoring and runtime
- validation-aware break and stop posture
- playtest capture hooks and replay handoff
- grounded walk loop for the first product result

## Runtime bridge rules
- runtime inspection is projection-only unless a legal edit path is declared;
- authoring and runtime state must never be conflated silently;
- attach and detach state must be explicit and visible;
- stale runtime bindings must be torn down on authority change or session end;
- play and simulate must attach to the same opened world identity unless an explicit forked-runtime mode is declared.

## First product-result runtime law
The first closure requires one legal loop:
`open world -> see terrain + sky/environment -> enter play/simulate -> move through the same world -> return to authoring -> preserve world identity and diagnostics context`.

That loop is not optional polish.
It is the first proof that the editor and runtime are working on one stack rather than two disconnected demos.

## Mandatory user-visible actions
- Play
- Simulate
- Pause
- Step Simulation
- Runtime Inspector
- Possess Walk Pawn
- Release Possession
- Return to Authoring

## Required product states
- `RuntimeEntryAvailable`
- `RuntimeEntryDenied`
- `PlayAttached`
- `SimulateAttached`
- `Paused`
- `GroundedWalkActive`
- `ReturnPending`
- `AuthoringRestored`

## Mandatory runtime-entry checks
Before play or simulate is accepted, the product must know:
- world-open posture;
- startup/bind posture;
- terrain binding posture;
- environment binding posture;
- camera attach posture;
- diagnostics attach posture;
- capability-tier posture.

## Runtime-entry law
Play and simulate entry must lower through explicit result-bearing contracts.
The editor may not flip into a runtime posture based on local optimism.

## Return law
`Return to Authoring` must:
- preserve world identity;
- preserve selection where legal;
- preserve camera by explicit policy;
- preserve diagnostics continuity;
- explicitly state whether any runtime-only state was discarded.
