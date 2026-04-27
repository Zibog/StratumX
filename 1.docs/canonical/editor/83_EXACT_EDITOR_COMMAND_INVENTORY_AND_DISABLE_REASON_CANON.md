# Exact Editor Command Inventory and Disable-Reason Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document is the suite-wide law for exact editor command inventory publication.
It no longer applies only to heavy labs. It also governs the promoted base shell.
If a button is visible in the top-level menu, the day-zero conveyor, the project-to-game ladder, or any mandatory base surface, it must be formalized here and concretized in `editor/110`.

## Required inventory fields
- command id
- label
- owner suite
- visibility class
- owner surface
- activation class
- toolbar group
- palette entry
- shortcut
- context menu action
- inspector action class
- overlay toggle class when relevant
- dependency gate
- disabled reason family
- preview vs commit classification
- authoring / simulate / play availability
- focus restoration rule
- success focus
- retry focus
- terminal failure focus
- shell state change
- evidence relation if the action mutates project, world, terrain, material, sky, audio, or runtime truth

## Base-shell exact inventory closure
The following promoted families are mandatory inventory classes exactly as strictly as heavy lab routes:
- `project`
- `world`
- `terrain`
- `material`
- `sky`
- `view`

For each promoted base-shell command the canonical inventory must publish:
- one exact `btn.*` id;
- one exact label;
- one owner surface;
- one visibility class;
- one activation class;
- one dependency gate;
- one disabled-reason family;
- one focus restoration rule;
- one success focus;
- one retry focus;
- one terminal failure focus;
- one shell-state transition;
- one evidence relation when the action mutates truth.

A top-level button that lacks the above fields is not “implicitly obvious”.
It is formally incomplete and therefore not gold.

## Visibility classes
### A. Base-shell mandatory
Everyday controls required to create, inspect, save, and validate a game world without debug detours.

### B. Advanced authoring
Serious content and runtime authoring that is not always promoted in the initial shell but is still part of everyday production depth.

### C. Lab / review / certification
Compare, capture, recover, certify, regression, freeze, release, and review-heavy surfaces.

Visibility class is a mandatory field.
A button may not silently drift between these classes.

## Suite coverage
The requirement explicitly covers `58` through `82`, plus benchmark/certification controls in `84`.
Audio everyday authoring surfaces in `62` and runtime/mix lab surfaces in `95` are freeze-relevant for audio closure when they introduce active `btn.audio.*` rows.
View/shell controls promoted by `editor/111`, `editor/112`, and `editor/106` are now freeze-relevant base-shell controls even when they do not mutate runtime truth.

## Phase-2 production closure law
The following labs are now required to expose full production contour classes in addition to heavy-domain review flows:
- editor `75`
- editor `76`
- editor `78`
- editor `79`
- editor `80`
- editor `90`
- editor `91`
- editor `92`
- editor `93`
- editor `94`
- editor `95`
- editor `96`
- editor `97`
- editor `98`
- editor `103`
- editor `105`
- editor `109`

For each listed lab, the canonical inventory must include exactly these classes where the domain supports them:
- `author`
- `bind`
- `inspect`
- `simulate`
- `compare`
- `capture`
- `recover`
- `certify`

Audit/review-only declarations are insufficient.

## Base-shell implementation-tail rule
For every promoted `mutate`, `bind`, `save`, `import`, or `rebuild` action, the inventory law is incomplete unless the owning canon also states:
- what authoritative truth object is changed;
- which tooling route and SDK family carry the mutation;
- which validation gates are checked before commit;
- which derived caches or indices are invalidated;
- which viewport, inspector, outliner, and status-bar publications are required;
- what is saved immediately versus left pending;
- which recovery anchor is created;
- which denial family returns on failure.

## Acceptance rule
A patch fails if any button promoted by `editor/111`, `editor/112`, or `editor/106` is absent from `editor/110`, lacks a visibility class, or lacks an implementation tail where it mutates truth.

## Material-centric inventory requirement
For material-owned promoted commands the inventory is incomplete unless it classifies which branch the command belongs to:
- identity
- physical
- visual
- audio
- light
- runtime cheapness
- preview/validation

These branch tags are mandatory for `editor/113` and must align with `editor/110`.
