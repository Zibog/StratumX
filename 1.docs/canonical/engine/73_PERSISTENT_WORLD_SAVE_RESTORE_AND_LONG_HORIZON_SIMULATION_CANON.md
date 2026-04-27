# Persistent World Save Restore And Long Horizon Simulation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own save/restore identity, long-horizon simulation anchors, retained baselines, and restore-safe publication for heavy domains.

## Exact truth objects
- `savepoint_identity_state`
- `restore_anchor_registry`
- `long_horizon_digest`
- `domain_baseline_pointer`
- `timeline_window_state`
- `artifact_retention_state`

## Exact state machine
`snapshot_prepare -> identity_seal -> persist -> restore_validate -> replay_compare -> published`

## Exact phase order
1. prepare declared domain save slices;
2. seal stable identities and baseline pointers;
3. persist chunked snapshots and artifacts;
4. validate restore anchors and replay windows;
5. publish restore and long-horizon digests.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | engine `61`, `64`, `66`, `67`, `68` domain anchors; root `71` wide entry/exit gate; root `99` readiness conditions | never owns domain simulation semantics |
| publishes | sdk persistence packets; root `68` signoff chain; editor `77`, `103`, `105`, `109` | may not collapse baseline loss into silent recovery |

## Resource envelope
- CPU: green <= `0.9 ms` amortized snapshot overhead, yellow <= `1.4 ms`, orange <= `2.0 ms`, red > `2.0 ms`;
- GPU: none;
- RAM: green <= `512 MiB` retained baselines, orange > `768 MiB`, red > `1.0 GiB`;
- disk / IO: orange when write burst > `32 MiB` checkpoint, red when visible hitch > `12 ms`.

## Ordered degrade ladder
- reduce non-critical background verification;
- compact historical snapshot density;
- serialize secondary artifacts after critical reads;
- sample timeline overlays only;

## Replay and compare windows
- `baseline.official`;
- `cert.restore`;
- `restore.official`;
- `longhorizon.2400`;

## Failure and denial code families
- `save.identity.drift`
- `restore.anchor.missing`
- `replay.compare.gap`
- `artifact.baseline.lost`

## Certification duties
- retain last-good baseline, failed-run artifact, and recovery-run artifact for every restore-bearing certification pack;
- publish exact recovery action when restore validation fails;
- emit denial when a freeze request lacks required retained baselines;
- expose retained-bundle completeness to `editor/87` and `editor/89`.

## Current posture
`document_gold / doc_closed_impl_open`
