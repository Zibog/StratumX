# Truth to Frame Operator Proof Route Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the single legal operator proof route from runtime truth publication to frame, speaker, diagnostics, artifact bundle, and freeze signal.

## Legal proof route
1. identify the truth owner in engine;
2. reveal the published sdk packet family;
3. follow the tooling route that consumes, normalizes, and records it;
4. inspect the editor lab that renders the packet and diagnostics state;
5. perform capture and compare using the declared mode;
6. append evidence only through the declared artifact family;
7. certify or deny through the declared certification pack;
8. freeze only when acceptance rows remain green.

## Mandatory proof checkpoints
| Checkpoint | Required owner |
|---|---|
| truth object exists | engine |
| packet family is exact | sdk |
| route state and diagnostics are exact | tooling |
| overlay, fields, and focus are operator-visible | editor |
| capture and compare mode is declared | root/editor |
| artifact bundle is retained | sdk/tooling |
| pressure and degrade ladder are visible | root/engine/sdk/tooling/editor |
| freeze verdict is explicit | root/tooling/editor |

## Honesty rule
A proof route may prove document closure of one active contour.
It may not be used to overclaim broad runtime implementation breadth.
