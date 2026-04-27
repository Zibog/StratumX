# Runtime Wiring

## Role

Startup-mediated runtime wiring.

## Data Model

Startup binds the runtime kernel, the selected runtime profile, the selected runtime role, registered `L1.5` resource services, startup-mediated runtime-pack products, validated manifest state, optional bounded warmup artifacts, and the narrow public bridge export surfaces consumed by external `L5`.
Wiring ends at legal launch; ownership of runtime and services transfers after bootstrap completion.
Public bridge export surfaces remain public contract surfaces only and do not transfer engine execution ownership.

## Wiring Matrix

| Bound element | Legal input | Required guarantee | Illegal posture |
|---|---|---|---|
| runtime kernel | selected runtime profile + bootstrap state | runtime/profile bind frozen before launch | runtime selected by service-owned policy |
| runtime role | selected canonical runtime role | role bind frozen before launch | undeclared role class |
| resource services | registered `L1.5` crates + startup-mediated runtime-pack products | no upward crate dependency inversion | direct upward crate dependency on `engine_content` |
| validated manifest state | manifest verified under startup law | no hidden bypass of verification posture | implicit manifest trust |
| optional warmup artifacts | bounded pipeline-cache blobs or bounded verification outputs | warmup remains optional and budget-bounded | persistent bootstrap governor after launch |
| public bridge export surfaces | startup-owned public sinks, snapshots, and batch sources | narrow external bind only; no deep internal leakage | direct external reach into engine internals |

## Canonical Wiring Law

- startup wires content products only as startup-mediated runtime-pack products;
- startup may not define undeclared role classes, hidden service classes, or persistent bootstrap governors;
- runtime ownership begins only after legal bootstrap completion;
- wiring must remain consistent with selected profile, selected role, hardware-floor legality, and startup verification ceilings;
- public bridge export surfaces may expose only declared public contract classes, never deep engine internals.

## Failure Posture

- illegal profile/role/product combinations must be refused before runtime ownership begins;
- optional warmup artifacts may be dropped to preserve startup legality;
- failed verification may not be bypassed by hidden fallback wiring;
- illegal bridge export widening must be refused before external bind becomes visible.

## Illegal Patterns

- startup wiring that persists as a second runtime owner;
- undeclared product classes;
- fallback wiring that bypasses manifest validation or hardware-floor law;
- undefined canonical terms in runtime wiring state;
- public bridge export surfaces that leak mutable deep engine internals.

## Bind Matrix

| Bound class | Must be frozen at bootstrap | Illegal persistence |
|---|---|---|
| runtime profile | yes | runtime-side reselection |
| runtime role | yes | shadow post-launch role bind |
| resource-service set | yes | startup as persistent governor |
| warmup artifacts | optional and bounded | permanent bootstrap assist |
| public bridge export surfaces | yes | ad-hoc post-launch widening |

## Verification Posture

Failed profile/role/product verification aborts bootstrap.
Startup may drop optional warm artifacts but may not bypass manifest, hardware-floor, role legality, or public bridge contract law.

## Local Operating Law

Runtime wiring is startup-mediated and one-way.
After legal launch, startup relinquishes ownership rather than lingering as a second orchestrator.
