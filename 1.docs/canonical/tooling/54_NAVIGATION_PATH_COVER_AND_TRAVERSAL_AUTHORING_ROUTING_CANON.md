# Navigation Path Cover And Traversal Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the exact production route family for navigation authoring, cover legality authoring, traversal mutation publication, preview simulation, compare, and certification.

## Exact intent schema families
- `intent.nav.author_path_profile`
- `intent.nav.bind_cover_response`
- `intent.nav.inspect_traversal_slice`
- `intent.nav.simulate_local_route`
- `intent.nav.compare_route_baseline`
- `intent.nav.capture_route_bundle`
- `intent.nav.recover_last_good_route`
- `intent.nav.certify_route_pack`

## Route ids
- `route.nav.author_profile`
- `route.nav.bind_cover_rule`
- `route.nav.inspect_slice`
- `route.nav.simulate_local_route`
- `route.nav.compare_baseline`
- `route.nav.capture_bundle`
- `route.nav.restore_baseline`
- `route.nav.certify_pack`

## Transaction state machine
`requested -> validated -> normalized -> prepared -> executing -> publishing -> completed | retryable_failure | rolled_back | terminal_failure`

## Retry limits
- author and bind routes: one automatic retry on transient cache race;
- simulate and compare routes: zero automatic retries, one operator retry with preserved failed-run bundle;
- certify route: zero retries without a new compare digest.

## Rollback anchors
- `anchor.nav.last_good_author_profile`
- `anchor.nav.last_good_cover_rule`
- `anchor.nav.last_good_compare_triplet`

## Invalidation triggers
- topology graph digest mismatch;
- cover rule version mismatch;
- traversal profile revision change;
- route-local simulation window change;
- owner-truth migration.

## Cache ownership
- traversal graph cache: tooling navigation route;
- cover legality projection cache: tooling navigation route;
- local simulation preview cache: tooling navigation simulation route.

## Artifact ownership
- compare digest and simulation result bundle are retained route-local artifacts;
- capture bundle ownership transfers to evidence surfaces only after successful append.

## Diagnostics envelope
Every terminal route emission must include:
- `route_id`
- `packet_family`
- `truth_owner_id`
- `focus_target_id`
- `next_action_id`
- `artifact_ref`
- `first_failure_code` or terminal success code

## Recovery action mapping
| Failure family | Legal recovery |
|---|---|
| `nav.topology_digest_gap` | `action.nav.rebuild_route_graph` |
| `nav.cover_rule_mismatch` | `action.nav.rebind_cover_rule` |
| `nav.baseline_triplet_missing` | `action.nav.restore_compare_triplet` |
| `nav.preview_window_invalid` | `action.nav.reset_preview_window` |
| `nav.owner_scope_forbidden` | `action.nav.focus_scope_boundary` |

## Production law
This route family is not limited to audit and compare.
It must support author, bind, inspect, simulate, compare, capture, recover, and certify for the navigation/traversal family without editor-side shortcuts.
