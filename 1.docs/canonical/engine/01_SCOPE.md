# Scope

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This package defines the canonical engine-only stack of StratumX.

It describes:

- the fixed lower substrate;
- the world truth boundary;
- the shared world property substrate;
- the runtime family;
- the runtime resource service layer;
- the critical simulation families;
- the network runtime service layer;
- the model systems;
- the synthesis systems;
- the resource system;
- the startup layer.

It does not describe:

- game-specific rules;
- SDK-side product behavior;
- editor/UI tool behavior;
- external consumer stacks;
- marketplace, studio, or live-ops surfaces.

External consumers may exist above this engine stack, but they are not part of this package.

This package also records intentional boundary transitions from earlier split canonical packages through explicit transition and preservation documents.
