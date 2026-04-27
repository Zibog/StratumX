# STRATUMX_STACK_VERSION_SOURCE

## 1. Purpose

This document defines the canonical source of truth for the canonical stack version marker.

## 2. Source of Truth

The canonical stack version marker is defined by the file:

- `docs/canonical/engine/STACK_VERSION`

## 3. Marker Format

Canonical format:

`SX-CANON/<major>.<minor>.<patch>/STACK-<revision>`

Example:

`SX-CANON/1.0.24/STACK-v30`

## 4. Owner

The owner of the canonical stack version marker is the canonical engine package rooted at `docs/canonical/engine/`.

## 5. Storage Rule

The marker is stored only in:
- `docs/canonical/engine/STACK_VERSION`

Constitutional and startup documents may reference it, but do not own its value.

## 6. Change Rule

The marker changes only when the canonical engine package changes in a way that affects:
- legal restoration;
- configuration legality;
- runtime law;
- family boundaries;
- world/state law;
- canonical shared shapes;
- canonical startup assembly legality.

Patch/minor/major meaning:
- patch: documentation correction with no legal engine-state change;
- minor: additive legal change preserving previous canonical legality where compatible;
- major: breaking legal change to restoration, execution, boundaries, or compatibility.

## 7. Historical Lineage Rule

Pre-v6 canonical revisions are mapped into the stack-version lineage as historical pre-marker revisions.
The first explicit marker-owning revision begins at v6 of the master archive lineage.
Historical mapping is:
- pre-v6 archives: historical lineage only, no authoritative in-package marker;
- v6 and later: authoritative marker-owned lineage.

## 8. Canonical Laws

- there is one canonical stack version source of truth;
- persistence and restoration flows must use that source;
- no alternate stack-version source is legal.
