# Technology Transaction State And Retry Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This registry freezes canonical transaction state ids and retry semantics.

| State id | Meaning |
|---|---|
| `requested` | intent accepted |
| `validated` | legal scope and ownership checked |
| `normalized` | ids and payload fields normalized |
| `prepared` | route and artifact plan ready |
| `executing` | route active |
| `publishing` | results, focus, and artifacts being emitted |
| `retryable_failure` | bounded retry available |
| `rolled_back` | rollback anchor applied |
| `completed` | terminal success |
| `terminal_failure` | terminal failure |

## Retry law
A retry-bearing transaction must publish:
- retry counter;
- failed-run ref;
- focus target after retry;
- recovery action id;
- invalidated cache ids.

## Rollback law
A rollback-bearing transaction must name:
- rollback anchor id;
- prior terminal failure or retryable failure code;
- retained artifact refs that survive rollback.

## Material-first authoring note
Terrain rebuild, world validation, material preview, and chunk save are retry-bearing only when the failed-run ref, unresolved truth owner, and next legal focus are all published.
A silent “try again” button is forbidden.

## Negative law
- retry without failed-run id is forbidden;
- rollback without anchor id is forbidden;
- terminal publication without state id is forbidden.

## Material-centric transaction additions
Transactions that mutate material-owned truth must publish retry/focus families for:
- response family bind
- light response bind
- visual response bind
- acoustic profile bind
- cheap-runtime rung change
- route-closure validation retry


## Dream-stack heavy-domain transaction note
Destruction, terrain deformation, hydrology, climate, society/tactics, wound/species, and groom/fallback edits are retry-bearing only when the route publishes:
- the exact mutated scope;
- the first blocker code;
- the recovery anchor ref;
- the downstream invalidation set;
- the retained artifact posture when certification is affected.

## Additional state note
Heavy-domain certification runs may insert one documented publication substate, `evidence_sealed`, between `publishing` and `completed` when retained artifacts are mandatory.
