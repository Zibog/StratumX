# Heavy Domain Authoring Transaction And Failure Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define how heavy-domain mutations are requested, validated, denied, applied, and rolled back through sdk surfaces.

## Mandatory transaction stages
| Stage | Required output |
|---|---|
| request | stable tx id, initiator, target family, target ids |
| validation | blocker family, exact invalid field list, deny/retry posture |
| apply | authoritative result or explicit deny |
| invalidation | downstream dirty scopes and consumer impact |
| undo record | reversible scope or explicit irreversibility verdict |
| artifact relation | retained artifact refs when capture or certification is affected |

## Failure families
- `TX_SCOPE_*` invalid target scope or ownership
- `TX_SCHEMA_*` malformed packet or missing required field
- `TX_STATE_*` illegal current state for requested mutation
- `TX_RATE_*` forbidden cadence or concurrency violation
- `TX_COMPAT_*` incompatible version or consumer break risk

## Undo law
Undo is legal only when:
- the mutation family explicitly names a reversible scope;
- authoritative invalidation data is retained; and
- reversal does not fabricate missing runtime evidence.

## Consumer duty
Editor and tooling consumers must never infer success from missing failure.
Every mutation path must emit either success or explicit denial.
