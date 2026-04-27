# Prediction and Reconciliation

## Role

Latency compensation law.

## Data Model

Prediction is legal only over owned inputs and declared rewindable domains.
Reconciliation is correction-burst bounded and may not become a second hidden world owner.

## Canonical Rules

- prediction may not mutate authoritative world truth;
- reconciliation burst is capped at 2 consecutive frames above normal sync cost;
- correction payloads obey the same quantization classes as authoritative exports.
