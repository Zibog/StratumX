# engine_net_latency

## Stack Position

L2.5. Network Runtime Services

## Primary Role

net latency.

## Canonical Scope

Runtime network-service ownership local to this crate.

## Boundary Rationale

This crate exists so transport, sync, and latency-control ownership stay explicit and do not collapse into runtime, gameplay, or SDK layers.

## Canonical Consumers

- higher service layers and startup where justified by local contracts.

## Downward Dependencies

See `20_DEPENDENCIES.md`.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Input History | Owned-input history model. | `40_INPUT_HISTORY.md` |
| Prediction and Reconciliation | Prediction and ordered correction model. | `41_PREDICTION_AND_RECONCILIATION.md` |
| Rewind and Validation | Short-history rewind validation model. | `42_REWIND_AND_VALIDATION.md` |
