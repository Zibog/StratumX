# Terrain Deformation Crater Volume And Consequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own terrain as a physically mutable substrate rather than a static art surface.

## Deformation classes
- `crater`
- `gouge`
- `trench`
- `soft_depression`
- `asphalt_spall`
- `mud_churn`

## Exact truth objects
- `TerrainVolumeDelta`
- `TerrainRemovedMassLedger`
- `TerrainDepositedMassLedger`
- `DeformationClassDigest`
- `TerrainConsequenceDigest`

## Volume law
Terrain deformation may remove, redistribute, compact, or deposit mass, but every operation must publish a class and a persistence posture.

## Consequence law
Every committed deformation publishes consequence deltas to:
- navigation / cover;
- hydrology and pooling;
- material exposure and aftermath;
- projectile impact behavior.

## Failure families
- `terrain.deformation_class_missing`
- `terrain.mass_ledger_fraud`
- `terrain.consequence_publication_missing`
- `terrain.restore_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
