# Typology System

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

L5 is split by data class, not by feature fantasy.

## Mutation-oriented classes
- command packets
- binding controls
- execution signals

## Read-oriented classes
- observation records
- metric frames

## Decision classes
- compatibility facts
- compatibility verdicts
- legality verdicts
- transport policies

## Opaque classes
- handles
- refs
- artifact refs

## Cost rationale
Mutation-oriented classes stay narrow and ordered.
Read-oriented classes stay fanout-friendly.
Decision classes stay pure and replayable.
Opaque classes stay pointer-free and data-light.
