# Evidence Capture Compare And Append Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for triplet compare, evidence append discipline, and retained-artifact closure.
This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| inspect | `btn.evidence.inspect_artifact_tuple` | `route.evidence.inspect_artifact_tuple.v1` | `packet.evidence.inspect_artifact_tuple.v1` | engine `84` + engine `97` | `read.inspect` | artifact-tuple drilldown | `deny.evidence.tuple_missing` |
| simulate | `btn.evidence.simulate_append_preview` | `route.evidence.simulate_append_preview.v1` | `packet.evidence.simulate_append_preview.v1` | engine `84` + engine `97` | `simulate.preview` | append-preview run | `deny.evidence.preview_scope_invalid` |
| compare | `btn.evidence.compare_triplet_bundle` | `route.evidence.compare_triplet_bundle.v1` | `packet.evidence.compare_triplet_bundle.v1` | engine `84` + engine `97` | `analyze.compare` | triplet compare digest | `deny.evidence.compare_baseline_missing` |
| capture | `btn.evidence.capture_evidence_delta` | `route.evidence.capture_evidence_delta.v1` | `packet.evidence.capture_evidence_delta.v1` | engine `84` + engine `97` | `capture.artifact` | evidence delta bundle | `deny.evidence.capture_target_missing` |
| recover | `btn.evidence.recover_append_context` | `route.evidence.recover_append_context.v1` | `packet.evidence.recover_append_context.v1` | engine `84` + engine `97` | `recover.baseline` | recovered append context | `deny.evidence.recovery_anchor_missing` |
| append | `btn.evidence.append_evidence_bundle` | `route.evidence.append_evidence_bundle.v1` | `packet.evidence.append_evidence_bundle.v1` | engine `84` + engine `97` | `evidence.append` | appended evidence bundle | `deny.evidence.append_forbidden` |
| certify | `btn.evidence.certify_evidence_pack` | `route.evidence.certify_evidence_pack.v1` | `packet.evidence.certify_evidence_pack.v1` | engine `84` + engine `97` | `release.certify` | evidence pack verdict | `deny.evidence.certification_gap` |
| review | `btn.evidence.review_focus_chain` | `route.evidence.review_focus_chain.v1` | `packet.evidence.review_focus_chain.v1` | engine `84` + engine `97` | `review.focus` | focus-chain review | `deny.evidence.focus_chain_missing` |

## Exact compare modes
- triplet compare;
- route-schema compare;
- button-chain conformance compare;
- recovery compare;
- proof-region relay compare;

## Required overlays and drilldowns
- artifact tuple;
- triplet closure ladder;
- focus chain;
- append lineage;
- baseline anchors;
- proof-region relay lineage;

## Exact inspector fields
- baseline artifact ref;
- failed-run artifact ref;
- recovery-run artifact ref;
- first failure code;
- next legal action;
- focus target id;
- packet family id;
- proof-region recipe ref;
- launch verification ref when present;

## Freeze relevance
- certification for `pack.evidence_triplet_append` and `pack.brutal_proof_region_relay` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Phase-5 evidence append obligations
- evidence append must preserve proof-region recipe, retained baseline, failed-run, recovery-run, compare digest, freeze review, executable ref, and first-result verification as one linked chain;
- retained artifact tuples must keep baseline/current/recovered links for mixed packs and restore-safe proofs;
- append is forbidden when the captured bundle hides the first denial, degrade rung, or launch-verification mismatch.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
