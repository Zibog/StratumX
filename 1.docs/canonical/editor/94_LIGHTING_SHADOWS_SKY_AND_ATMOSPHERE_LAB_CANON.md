# Lighting Shadows Sky And Atmosphere Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for lighting, shadows, sky, atmosphere, and long-range visibility authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.sky.author_celestial_profile` | `route.sky.author_celestial_profile.v1` | `packet.sky.author_celestial_profile.v1` | engine `90` + engine `91` | `mutate.profile` | celestial profile revision | `deny.sky.profile_invalid` |
| bind | `btn.sky.bind_atmosphere_profile` | `route.sky.bind_atmosphere_profile.v1` | `packet.sky.bind_atmosphere_profile.v1` | engine `90` + engine `91` | `mutate.binding` | atmosphere binding graph | `deny.sky.binding_missing` |
| inspect | `btn.sky.inspect_horizon_visibility` | `route.sky.inspect_horizon_visibility.v1` | `packet.sky.inspect_horizon_visibility.v1` | engine `90` + engine `91` | `read.inspect` | horizon visibility drilldown | `deny.sky.visibility_missing` |
| simulate | `btn.sky.simulate_storm_front` | `route.sky.simulate_storm_front.v1` | `packet.sky.simulate_storm_front.v1` | engine `90` + engine `91` | `simulate.preview` | storm-front preview | `deny.sky.preview_scope_invalid` |
| compare | `btn.sky.compare_atmosphere_triplet` | `route.sky.compare_atmosphere_triplet.v1` | `packet.sky.compare_atmosphere_triplet.v1` | engine `90` + engine `91` | `analyze.compare` | atmosphere compare digest | `deny.sky.compare_baseline_missing` |
| capture | `btn.sky.capture_atmosphere_evidence` | `route.sky.capture_atmosphere_evidence.v1` | `packet.sky.capture_atmosphere_evidence.v1` | engine `90` + engine `91` | `capture.artifact` | atmosphere evidence bundle | `deny.sky.capture_target_missing` |
| recover | `btn.sky.recover_atmosphere_baseline` | `route.sky.recover_atmosphere_baseline.v1` | `packet.sky.recover_atmosphere_baseline.v1` | engine `90` + engine `91` | `recover.baseline` | recovered atmosphere baseline | `deny.sky.recovery_anchor_missing` |
| certify | `btn.sky.certify_visibility_pack` | `route.sky.certify_visibility_pack.v1` | `packet.sky.certify_visibility_pack.v1` | engine `90` + engine `91` | `release.certify` | visibility pack verdict | `deny.sky.certification_gap` |

## Exact compare modes
- atmosphere triplet compare;
- storm front compare;
- shadow rung compare;
- visibility compare;

## Required overlays and drilldowns
- sky bands;
- shadow cascade map;
- cloud front track;
- long-range horizon;
- baseline anchors;

## Exact inspector fields
- celestial profile id;
- atmosphere profile id;
- storm-front id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.sky.profile_missing`;
- `disable.sky.binding_unbound`;
- `disable.sky.baseline_missing`;
- `disable.sky.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `94` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `94` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.atmosphere.triplet` and pack `pack.storm_long_range_visibility` pinned;
- capture success -> `103` or `105` with capture mode `capture.sky.atmosphere_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `94` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.storm_long_range_visibility`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.sky.profile`;
- `artifact.sky.binding`;
- `artifact.sky.compare_digest`;
- `artifact.sky.trace_ref`;
- `artifact.sky.baseline_ptr`;

## Freeze relevance
- certification for `pack.storm_long_range_visibility` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- tunnel darkness, muzzle flash, dynamic shadows, and far-storm media stay causally aligned;
- exposure/post changes may not fake a lighting result that the engine truth owner did not publish.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`


## Base-shell everyday sky authoring closure
The sky surface is not base-shell closed merely because lab-native author/capture/certify routes exist.
The following everyday buttons are mandatory promoted controls and are part of the clean editor shell:
- `btn.sky.bind_sky_profile`
- `btn.sky.set_time_of_day`
- `btn.sky.set_weather_regime`
- `btn.sky.bind_cloud_profile`

These controls are base-shell truth, not optional lab aliases.
For each of them the owning surface must publish:
- exact route and packet family;
- environment truth owner;
- dependency gate;
- focus result;
- world-save posture;
- denial family;
- required viewport and inspector publication.

## Everyday sky implementation tails
- binding a sky profile mutates world environment truth and invalidates lighting, horizon, and environment preview caches;
- changing time of day mutates environment parameters and republishes shadow, exposure-hint, and atmosphere state;
- changing weather regime republishes wetness, cloud, ambience-hint, and degraded-posture context;
- binding cloud profile republishes cloud/shadow sweep state and horizon visibility posture.

## Material-light branch relation
Everyday light/transmission/shadow authoring for material-owned surfaces must remain callable from `editor/113`.
This lab provides deeper inspection and compare routes but may not become the only place where material light law is configurable.
