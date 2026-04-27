# World Causality Trace And Explanation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own world-scale cause chains, operator-facing explanation digests, and lawful stitching of heavy-domain reasons without stealing domain truth.

## Exact truth objects
- `cause_chain_state`
- `reason_fragment_registry`
- `event_provenance_state`
- `compare_explanation_digest`
- `operator_question_state`
- `trace_restore_anchor`

## Exact state machine
`fragment_accept -> provenance_link -> chain_assemble -> explanation_publish -> reviewed -> retained`

## Exact phase order
1. accept bounded reason fragments from domain owners.
2. validate provenance and ownership.
3. assemble legal cause chains.
4. publish explanation and compare digests.
5. retain artifacts and restore anchors.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | 49/61/64/65/66/67/68/71/73/75 reason fragments | never owns source-domain truth or overrides failure families |
| publishes | 56 sdk causality packets; 58 tooling routes; 80 editor why-happened lab | may not invent missing causes to close a chain |

## Resource envelope
- CPU: green <= 0.6 ms, yellow <= 1.0 ms, orange <= 1.4 ms, red > 1.4 ms.
- GPU: none except overlays.
- RAM: green <= 96 MiB retained reason chains, orange > 144 MiB, red > 208 MiB.
- disk / IO: artifact retention only; blocking write forbidden.

## Ordered degrade ladder
- compact historical explanation mirrors;
- reduce non-critical visual trace detail;
- defer secondary exports after baseline retention;
- sample overlay-only views outside active review;

## Replay and compare windows
- `baseline.120`;
- `cert.300`;
- `restore.300`;
- `incident.900`;

## Failure and denial code families
- `trace.provenance.missing`
- `trace.chain.gap`
- `trace.compare.digest_missing`
- `trace.restore.anchor_missing`

## Certification duties
- retain one chain artifact, one compare digest, and one next legal recovery action for every certification review
- emit denial when a requested explanation lacks authoritative fragments
- retain restore anchor when trace artifacts participate in freeze review



## Phase-3 operator questions that must be answerable
- why did a civilian become criminalized?
- why did the squad abandon cover and replan the flank?
- why did the migration corridor reroute?
- why is this path blocked now?
- why did the trader change price or stock?
- why did this quest branch apply or deny?
- why did dialogue emit a guarded denial instead of a consequence?

## Required fragment exporters
- engine `64` society reason fragments;
- engine `65` tactics and cover-break fragments;
- engine `66` ecology and hazard reroute fragments;
- engine `68` semantic guard and consequence fragments;
- engine `72` navigation blocker fragments;
- engine `74` scarcity and equipment legality fragments;
- engine `75` delayed-consequence fragments.

## Required publications
- `event.reason.chain_published.v1` with `source_domain`, `trace_ref`, `blocker_code?`, `next_legal_recovery_action`, `artifact_ref`;
- `artifact.reason.triplet` with baseline/failed/recovery and operator question scope.


## Current posture
`document_gold / doc_closed_impl_open`
