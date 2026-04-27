# Heavy Domain Numerics Replay And Compare Law Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This law pulls numerics, replay windows, quantization, and compare tolerances directly into heavy-domain closure.
No heavy-domain packet, compare mode, or certification row may bypass this file.

## Mandatory fields
Every heavy-domain publication row must carry:
- numeric clock source;
- integration mode;
- quantization mode;
- compare window id;
- deterministic replay eligibility flag;
- tolerated absolute error;
- tolerated relative error;
- failure code when outside tolerance.

## Domain tolerances
| Domain | Primary integrate mode | Compare window | Absolute tolerance | Relative tolerance | First failure family |
|---|---|---|---|---|---|
| hydrology | fixed-step semi-implicit | `300 / 900 / restore` | `0.001` world-volume units | `0.5%` | `hydro.numeric.*` |
| storm / weather | fixed-step field solve | `300 / 1200` | `0.002` field units | `1.0%` | `storm.numeric.*` |
| cloth / fur | fixed-step with bounded substep count | `120 / 300` | `0.003` contact-space units | `1.5%` | `soft.numeric.*` |
| population / tactics | fixed-step decision epoch | `60 / 300 / 900` | `1` state bucket step max | `1.0%` | `society.numeric.*`, `tactics.numeric.*` |
| ecology | fixed-step migration epoch | `300 / 900` | `1` route-rank step max | `1.0%` | `ecology.numeric.*` |
| wounds / ballistics | fixed-step consequence epoch | `1 / 60 / 300` | `0.001` damage-space units | `0.5%` | `wound.numeric.*`, `ballistics.numeric.*` |
| semantic runtime | event-step with deterministic guards | `event chain + restore` | `0` policy violations | `0` | `semantic.numeric.*` |
| persistence / replay | snapshot cadence with hash compare | `restore / long horizon` | `0` identity drift | `0` | `timeline.numeric.*` |
| brutal proof-region relay | mixed event-step plus fixed-step families | `baseline / failed / recovery / launch` | `0` identity drift across retained bundles | `0` hidden route swaps | `relay.numeric.*`, `timeline.numeric.*`, `cert.numeric.*` |

## Compare law
- compare must happen against retained baselines, never against memory of a previous run;
- a domain may expose fast compare, certification compare, replay compare, and launch verification compare, but all four must name their tolerance class explicitly;
- no editor surface may label a result as equivalent unless the tolerance class is visible.

## Failure law
A numerics failure is not a generic warning.
It must publish one of the following operator-visible outcomes:
- rerun allowed from current state;
- restore baseline required;
- degrade step required before rerun;
- freeze blocker until review.
