# STRATUMX_PERSISTENCE_AND_SNAPSHOT_CONSTITUTION

## 1. Purpose

This document defines the canonical persistence and snapshot law of StratumX.

## 2. Principles

### 2.1. Snapshot is not arbitrary dumping
Snapshots are canonical state forms, not uncontrolled memory captures.

### 2.2. Authoritative world state is the primary persistence target
Persistence is centered on world truth and required restoration context.

### 2.3. Transient execution state is not default persistence state
Runtime transient orchestration state is not automatically part of persistence.

## 3. Snapshot Classes

### 3.1. World snapshots
Canonical authoritative or read-oriented world captures.

### 3.2. Read snapshots
Snapshot forms used to support legal read windows.

### 3.3. Persistence payloads
Portable saved state forms meant for restoration.

## 4. Persistence Scope

The canonical persistence scope may include:

- authoritative world state;
- required material and descriptor references;
- required configuration needed for restoration legality;
- required version and compatibility metadata;
- runtime-profile-safe restoration state.

The canonical persistence scope does not automatically include:

- transient runtime scheduling state;
- temporary diagnostics buffers;
- non-required service-layer caches.

## 5. Restoration Law

Restoration must produce a legal world instance through startup-ready assembly decisions and compatibility checks.

See:

- `STRATUMX_PERSISTENCE_COMPATIBILITY_LAW.md`
- `STRATUMX_CONFIGURATION_CONSTITUTION.md`

## 6. Canonical Laws

- persistence targets authority, not arbitrary memory;
- restoration is legal only through startup and compatibility law;
- runtime transient state is excluded by default;
- snapshot and persistence law do not create new world authority.
