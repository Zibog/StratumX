# STRATUMX_API_CONTRACT_LAW

## 1. Purpose

This document defines the canonical law for public engine-facing contract surfaces.

## 2. Scope

This law governs:
- primary public boundary forms;
- primary public boundary types;
- minimal supporting public types;
- public trait boundaries;
- role-to-form legality;
- canonical shape-class usage across crate boundaries.

It does not govern implementation sequencing or local crate internals.

## 3. Taxonomy Separation Rule

Three classes are distinct and may not be conflated:

### 3.1. Boundary forms
Boundary forms describe the canonical public posture of a crate.

Allowed boundary forms are:
- backbone
- registry
- model
- layout
- window
- facade
- owner
- profile
- family
- service
- pipeline

### 3.2. Boundary types
A boundary type is the concrete public type that instantiates one legal boundary form for one crate.

### 3.3. Shape classes
Shape classes describe reusable traffic and support shapes used across crate boundaries.
Shape classes are defined in `STRATUMX_CANONICAL_SHAPES.md` and include both generic support classes and bounded primitive traffic classes.

## 4. Primary Boundary Form Law

Each canonical crate exposes one primary public boundary form.
Each canonical crate exposes one primary public boundary type that instantiates that form.

## 5. Role-to-Form Legality Rule

Canonical role classes map to legal primary boundary forms as follows:

| Canonical Role Class | Legal Primary Boundary Form |
|---|---|
| foundation substrate | backbone |
| identity substrate | registry |
| handle substrate | registry |
| physical or metadata registry substrate | registry |
| layout substrate | layout |
| access substrate | window |
| mutation substrate model | model |
| query substrate model | model |
| spatial substrate model | model |
| region substrate model | model |
| assembled substrate surface | facade |
| authoritative owner | owner |
| runtime profile | profile |
| runtime service | service |
| critical simulation family | family |
| model or synthesis service | service |
| resource processing layer | pipeline |

No crate may select a primary boundary form outside the legal mapping for its canonical role class.
Grouped substrate families are legal only through these exact role labels.

## 6. Minimal Public Surface Law

Each crate may expose:
- one primary public boundary type;
- only the minimum supporting public types required by the next canonical layer, constitutional law, testing law, or performance law.

## 7. Trait Surface Law

Public traits must be narrow, role-specific, and non-overlapping.
No public trait may combine read, stage, and apply authority in one boundary.

## 8. Shape Law

Public boundary traffic must use canonical shape classes defined in `STRATUMX_CANONICAL_SHAPES.md`.
Crate-local supporting types may derive from those classes, but may not redefine the classes themselves.

## 9. Authority Rule

Implementation documents may instantiate contract skeletons, but constitutional law remains the source of truth for:
- legal boundary forms;
- legal role-to-form mappings;
- legal shape classes;
- legal public boundary traffic.
