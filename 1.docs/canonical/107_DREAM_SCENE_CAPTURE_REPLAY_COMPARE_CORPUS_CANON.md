# Dream Scene Capture Replay Compare Corpus Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the retained corpus required to debug, certify, and regress the dream-stack domains.
A heavy system is not proof-bearing until it can be replayed, compared, and recovered without ad-hoc investigator work.

## Mandatory replay families

| Replay family | Primary focus | Required retained bundles | Primary companions |
|---|---|---|---|
| destruction replay | topology mutations, fragment release, aftermath | topology delta stream, fragment merge ledger, compare snapshot | sdk `82/88`, tooling `85/91`, editor `120/124` |
| deformation replay | crater/gouge/churn evolution | deformation slices, terrain material aftermath, compare baseline | sdk `82/88`, tooling `85/90–91`, editor `119/122/124` |
| hydrology replay | inventory, leak, inflow/outflow, dryout, contamination | container state checkpoints, event ledger, compare snapshots | sdk `82/88`, tooling `85/90–91`, editor `121/124` |
| fire/weather replay | ignition, drying, smoke transport, rain suppression | field snapshots, event stream, compare digest | sdk `82/88`, tooling `85/90–91`, editor `121/136` |
| climate theater replay | fronts, arrival windows, cloud and precipitation state | front checkpoints, timeline snapshots, compare baselines | sdk `82/88`, tooling `84/90–91`, editor `136/124` |
| living/tactics replay | needs deltas, squad intent, cover invalidation, ecology movement | intent packets, society deltas, causality trace bundle | sdk `82/88`, tooling `85/90–92`, editor `137/134` |
| wound traversal replay | projectile-to-layer traversal and consequence | projectile trace, wound packet, compare corpse or survivor state | sdk `82/88`, tooling `85/90–91`, editor `138/124` |
| photoreal proof replay | tunnel, flashlight, shadow ladder, audio-space continuity | frame captures, fallback reports, golden scene diff | sdk `82/88`, tooling `84/90–91`, editor `126/135/124` |
| first-playable proof replay | whole narrow proof region lane | project/world package, retained proof captures, benchmark verdict | sdk `84/88`, tooling `87/90–92`, editor `123/127/124` |

## Capture classes

| Class | Meaning | Retention law |
|---|---|---|
| event stream | exact retained events with lineage | immutable event ledger or sealed capture bundle |
| checkpoint snapshot | coarse or full state cut used for replay or compare | retained according to corpus policy |
| compare baseline | sealed golden or expected retained artifact | immutable until explicit recertification |
| differential digest | compact compare-friendly summary | may accompany but not replace required checkpoints |
| certification bundle | signed retained artifact pack | mandatory for release or old-floor proof claims |

## Compare law
- compare may not operate on unnamed or ad-hoc fields;
- every compare family must name primary keys, tolerance classes, and mismatch severity classes;
- if a family is non-deterministic by design, the compare contract must switch to statistical windows rather than silent waiver;
- first-playable proof compare may be narrower than dream-stack compare but must remain explicitly narrow.

## Mismatch severity classes

| Class | Meaning |
|---|---|
| informational | difference is acceptable and non-blocking |
| regression_warning | difference is legal but outside preferred range |
| certification_blocker | difference breaks the family proof obligation |
| schema_blocker | capture cannot be compared lawfully because packet/corpus identity is broken |

## Corpus packaging law
- every replay family must have one named corpus package class;
- corpus packages must include capture metadata, source identity, package marker, and baseline lineage;
- release-seal review may not claim a family is proof-bearing if its corpus package is absent;
- retained proof bundles must name whether they are slice-local, scenario-local, or stack-wide.

## Companion execution surfaces
- sdk `82` and `88` define capture, replay, and compare consumers;
- tooling `90–91` define benchmark and golden diff routing;
- editor `103`, `105`, `124`, and heavy labs own operator-visible replay/review surfaces;
- root `109`, `114`, and `118` must reference the same retained corpora.

## Acceptance obligations
A replay family is not canon-closed until:
- one retained bundle class is named;
- one compare baseline class is named;
- one consumer table exists in sdk `82/88`;
- one operator review surface exists in editor `103/105/124` or the owning lab;
- one tooling compare route exists in `90–91`.

## Current posture
`document_gold / proof_corpus_closed / execution_open`
