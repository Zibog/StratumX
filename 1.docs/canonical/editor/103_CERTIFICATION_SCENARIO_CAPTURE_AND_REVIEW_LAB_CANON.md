# Certification Scenario Capture And Review Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for scenario capture, blocker review, and certification triage.
This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| inspect | `btn.cert.inspect_blocker_board` | `route.cert.inspect_blocker_board.v1` | `packet.cert.inspect_blocker_board.v1` | engine `84` + engine `97` | `read.inspect` | blocker-board drilldown | `deny.cert.blocker_board_missing` |
| simulate | `btn.cert.simulate_repro_window` | `route.cert.simulate_repro_window.v1` | `packet.cert.simulate_repro_window.v1` | engine `84` + engine `97` | `simulate.preview` | repro-window run | `deny.cert.repro_scope_invalid` |
| compare | `btn.cert.compare_cert_triplet` | `route.cert.compare_cert_triplet.v1` | `packet.cert.compare_cert_triplet.v1` | engine `84` + engine `97` | `analyze.compare` | cert compare digest | `deny.cert.compare_baseline_missing` |
| capture | `btn.cert.capture_failed_run` | `route.cert.capture_failed_run.v1` | `packet.cert.capture_failed_run.v1` | engine `84` + engine `97` | `capture.artifact` | failed-run bundle | `deny.cert.failed_run_missing` |
| capture | `btn.cert.capture_recovery_run` | `route.cert.capture_recovery_run.v1` | `packet.cert.capture_recovery_run.v1` | engine `84` + engine `97` | `capture.artifact` | recovery-run bundle | `deny.cert.recovery_run_missing` |
| recover | `btn.cert.recover_bundle_focus` | `route.cert.recover_bundle_focus.v1` | `packet.cert.recover_bundle_focus.v1` | engine `84` + engine `97` | `recover.baseline` | focused cert baseline | `deny.cert.recovery_anchor_missing` |
| certify | `btn.cert.certify_scenario_pack` | `route.cert.certify_scenario_pack.v1` | `packet.cert.certify_scenario_pack.v1` | engine `84` + engine `97` | `release.certify` | scenario pack verdict | `deny.cert.certification_gap` |
| append | `btn.cert.append_review_verdict` | `route.cert.append_review_verdict.v1` | `packet.cert.append_review_verdict.v1` | engine `84` + engine `97` | `evidence.append` | review verdict append | `deny.cert.append_forbidden` |

## Exact compare modes
- cert triplet compare;
- scenario compare;
- repro-window compare;
- recovery compare;
- proof-region relay compare;

## Required overlays and drilldowns
- blocker board;
- scenario timeline;
- failed vs recovery runs;
- retained bundle lineage;
- baseline anchors;
- proof-region relay board;

## Exact inspector fields
- pack id;
- scenario id;
- baseline id;
- failed-run id;
- recovery-run id;
- first failure code;
- next legal recovery action;
- proof-region recipe ref;
- first-result verification ref when launch relevance is active;

## Freeze relevance
- certification for `pack.certification_scenario_review` and `pack.brutal_proof_region_relay` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Phase-5 certification obligations
- certification review must show proof-region recipe, retained baseline, failed run, recovery run, and first-result verification as one blocker board, not as unrelated scraps;
- mixed packs remain pinned inside the same review surface;
- a proof-region relay may not be marked ready if compare digest, recovery anchor, freeze board, executable ref, or launch verification is missing.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
