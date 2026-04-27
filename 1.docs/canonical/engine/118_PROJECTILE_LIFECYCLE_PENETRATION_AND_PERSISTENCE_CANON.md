# Projectile Lifecycle Penetration And Persistence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full lifecycle of projectile objects from spawn to cleanup.

## Lifecycle
`spawned -> flying -> field_influenced -> contact_test -> penetration / ricochet / stop / deflect -> secondary_chain -> retained_result -> cleaned`

## Exact law
- field influences may include wind, obscuration-independent drag classes, anomaly forces, and water/media classes;
- penetration publishes every traversed layer, residual energy, and exit verdict;
- ricochet publishes angle class, lost energy, and new path seed;
- long-distance ballistics may degrade cadence, never object identity or final verdict lineage.

## Failure families
- `projectile.flight_phase_gap`
- `projectile.penetration_chain_missing`
- `projectile.ricochet_unpublished`
- `projectile.cleanup_without_digest`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
