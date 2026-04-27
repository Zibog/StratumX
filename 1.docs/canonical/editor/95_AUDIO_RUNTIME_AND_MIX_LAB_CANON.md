# Audio Runtime And Mix Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for audio emitters, zones, listeners, mixes, ducking, occlusion, streaming readiness, and runtime variation authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author emitter class | `btn.audio.author_emitter_profile` | `route.audio.author_emitter_profile.v1` | `packet.audio.author_emitter_profile.v1` | engine `93` | `mutate.profile` | emitter profile revision | `deny.audio.profile_invalid` |
| author zone policy | `btn.audio.author_zone_profile` | `route.audio.author_zone_profile.v1` | `packet.audio.author_zone_profile.v1` | engine `57` | `mutate.profile` | zone profile revision | `deny.audio.zone_profile_invalid` |
| bind zone mix | `btn.audio.bind_zone_mix` | `route.audio.bind_zone_mix.v1` | `packet.audio.bind_zone_mix.v1` | engine `93` | `mutate.binding` | zone mix binding | `deny.audio.zone_missing` |
| inspect listener profile | `btn.audio.inspect_listener_profile` | `route.audio.inspect_listener_profile.v1` | `packet.audio.inspect_listener_profile.v1` | engine `57` | `read.inspect` | listener profile drilldown | `deny.audio.listener_profile_missing` |
| inspect occlusion chain | `btn.audio.inspect_occlusion_chain` | `route.audio.inspect_occlusion_chain.v1` | `packet.audio.inspect_occlusion_chain.v1` | engine `93` | `read.inspect` | occlusion drilldown | `deny.audio.occlusion_missing` |
| inspect bus and ducking | `btn.audio.inspect_bus_ducking` | `route.audio.inspect_bus_ducking.v1` | `packet.audio.inspect_bus_ducking.v1` | engine `58` | `read.inspect` | bus and ducking drilldown | `deny.audio.bus_ducking_missing` |
| inspect variation state | `btn.audio.inspect_variation_state` | `route.audio.inspect_variation_state.v1` | `packet.audio.inspect_variation_state.v1` | engine `93` | `read.inspect` | variation drilldown | `deny.audio.variation_state_missing` |
| simulate mix preview | `btn.audio.simulate_mix_preview` | `route.audio.simulate_mix_preview.v1` | `packet.audio.simulate_mix_preview.v1` | engine `93` | `simulate.preview` | mix preview run | `deny.audio.preview_scope_invalid` |
| compare regression triplet | `btn.audio.compare_mix_triplet` | `route.audio.compare_mix_triplet.v1` | `packet.audio.compare_mix_triplet.v1` | engine `93` | `analyze.compare` | mix compare digest | `deny.audio.compare_baseline_missing` |
| capture mix evidence | `btn.audio.capture_mix_evidence` | `route.audio.capture_mix_evidence.v1` | `packet.audio.capture_mix_evidence.v1` | engine `93` | `capture.artifact` | mix evidence bundle | `deny.audio.capture_target_missing` |
| recover mix baseline | `btn.audio.recover_mix_baseline` | `route.audio.recover_mix_baseline.v1` | `packet.audio.recover_mix_baseline.v1` | engine `93` | `recover.baseline` | recovered mix baseline | `deny.audio.recovery_anchor_missing` |
| certify audio pack | `btn.audio.certify_audio_pack` | `route.audio.certify_audio_pack.v1` | `packet.audio.certify_audio_pack.v1` | engine `93` | `release.certify` | audio pack verdict | `deny.audio.certification_gap` |

## Exact compare modes
- mix triplet compare;
- occlusion compare;
- variation compare;
- ducking and priority compare;
- recovery compare;

## Required overlays and drilldowns
- emitter map;
- listener profile overlay;
- zone mix graph;
- occlusion rays;
- ducking and stolen-voice board;
- variation bands;
- baseline anchors;

## Exact inspector fields
- emitter profile id;
- listener profile id;
- zone profile id;
- zone mix id;
- occlusion chain id;
- ducking rule id;
- active audible set id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.audio.profile_missing`;
- `disable.audio.listener_profile_missing`;
- `disable.audio.zone_unbound`;
- `disable.audio.bus_policy_missing`;
- `disable.audio.baseline_missing`;
- `disable.audio.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `95` with dirty profile/binding and the most relevant drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `95` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.audio.mix_triplet` and pack `pack.audio_mix_occlusion_variation` pinned;
- capture success -> `103` or `105` with capture mode `capture.audio.mix_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `95` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.audio_mix_occlusion_variation`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.audio.profile`;
- `artifact.audio.binding`;
- `artifact.audio.compare_digest`;
- `artifact.audio.trace_ref`;
- `artifact.audio.baseline_ptr`;

## Freeze relevance
- certification for `pack.audio_mix_occlusion_variation` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;


## Advanced everyday audio authoring closure
Audio is not product-closed if it exists only as a review lab.
The following advanced-authoring controls are required to bridge world authoring into playable game behavior:
- `btn.audio.assign_emitter_class_world_source`
- `btn.audio.bind_zone_profile_world_surface`
- `btn.audio.bind_priority_ducking_policy`
- `btn.audio.preview_audibility_free_camera`
- `btn.audio.preview_obstruction_vs_occlusion`
- `btn.audio.preview_indoor_outdoor_transition`
- `btn.audio.preview_voice_subtitle_legality`

These are not base-shell mandatory, but they are required advanced-authoring controls for full-game closure.

## Audio implementation-tail law
For each listed button the audio canon must state:
- which world source, zone, bus, listener, or dialogue truth is mutated or inspected;
- which propagation/occlusion/mix caches are invalidated;
- which viewport or shell overlays may optionally update;
- which inspector and status-bar signals must publish;
- whether the action leaves a pending world delta, a retained artifact, or both.

## Material-audio branch relation
Acoustic authoring may inspect zones, mix, ducking, and runtime previews here, but material-owned acoustic family selection and state remap must remain anchored in `editor/113`.
