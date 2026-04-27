# STRATUMX_PERSISTENCE_COMPATIBILITY_LAW

## 1. Purpose

This document defines canonical compatibility law for snapshots and persistence payloads.

## 2. Compatibility Principles

- persistence payloads must declare version and compatibility metadata;
- restoration is legal only if compatibility law is satisfied;
- startup owns the final restoration decision;
- incompatible payloads may not silently restore.

## 3. Compatibility Classes

### 3.1. Same-canon compatibility
Same canonical stack, same canonical family identities, compatible payload schema.

### 3.2. Compatible evolution
Payload may be restored through explicit migration rules and preserved boundary mappings.

### 3.3. Incompatible payload
Payload may not restore into authoritative progression without an explicit migration path.

## 4. Canonical Compatibility Data

Canonical metadata shapes are defined by:
- `STRATUMX_CANONICAL_SHAPES.md`
- `STRATUMX_STACK_VERSION_SOURCE.md`

A persistence payload must carry:
- schema version;
- canonical stack version marker;
- required family markers;
- restoration profile selectors;
- compatibility flags.

## 5. Restoration Decision

`engine_startup` performs the final compatibility decision using:
- payload metadata;
- configuration;
- legal enablement set;
- runtime profile selection;
- legal restoration decision.

## 6. Canonical Laws

- no silent compatibility assumptions;
- no restoration without explicit compatibility check;
- no incompatible payload becomes authoritative state;
- required family markers and compatibility flags are mandatory canonical fields, not optional commentary.
