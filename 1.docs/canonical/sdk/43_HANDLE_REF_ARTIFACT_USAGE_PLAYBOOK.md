# Handle, Ref, and Artifact Usage Playbook

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document explains when to use handles, identity refs, state refs, and artifact refs while answering real tasks.
The goal is to keep identity, live attachment, state snapshots, and durable products separate.

## Usage table
| Type | Use when | Do not use when |
|---|---|---|
| `SessionHandle` | the answer needs to identify the upper-stack session/request context | persistent world or asset identity |
| `ObjectHandle` | the answer needs an opaque bridge-level object attachment | public stable identity across saves |
| `RuntimeHandle` | the answer needs an active runtime attachment or source runtime anchor | a durable artifact or static content identity |
| `IdentityRef` | the answer needs a stable opaque identity reference | mutable snapshot payload or artifact lineage |
| `StateRef` | the answer needs a specific state-bearing view or snapshot owner | a durable cooked/build artifact |
| `ArtifactRef` | the answer needs a durable built/imported/generated product | runtime-local transient state |

## Practical law
- use handles for active attachments;
- use refs for identity or state without exposing raw ownership internals;
- use artifact refs for build/import/release outputs;
- never substitute a runtime handle for a durable asset identity;
- never substitute an artifact ref for live runtime truth.

## Example mappings
- `inspect a live runtime consequence` -> `RuntimeHandle` + `StateRef`
- `explain which object was hit` -> `IdentityRef` or the domain-specific entity id carried inside an observation
- `open the produced build package` -> `ArtifactRef`
- `refresh a session snapshot` -> `SessionHandle` + ingress control
