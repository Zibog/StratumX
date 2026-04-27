# Stream Activation

## Role

Runtime streaming activation model.

## Data Model

Stream activation classifies requests into `immediate`, `predicted`, and `maintenance` work.
Activation is locality-aware, chunk-aware, byte-weighted, and budget-visible.
All numeric request, byte, and aggregate ceilings are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Request Law

| Class | Ownership | Legal trigger |
|---|---|---|
| `immediate` | current-frame/current-tick need | current visibility, current authority, or current replay legality |
| `predicted` | next-window preparation | explicit locality evidence only |
| `maintenance` | hygiene and verification | trim, verification, or low-priority integrity work only |

## Legal Scheduling Rule

- request count ceilings and byte-weighted ceilings are both mandatory;
- aggregate in-flight request ceiling is global and profile-visible;
- activation may not bypass locality evidence by downgrading request class names.

## Illegal Patterns

- treating every request as `immediate`;
- count-legal but byte-illegal request floods;
- hidden activation outside stream-control ownership.
