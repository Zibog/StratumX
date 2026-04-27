# Material Shader And Texture Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for materials, shader variants, and texture truth authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.render.material.author_material_profile` | `route.render.material.author_material_profile.v1` | `packet.render.material.author_material_profile.v1` | engine `88` | `mutate.profile` | material profile revision | `deny.material.profile_invalid` |
| bind | `btn.render.material.bind_shader_variant` | `route.render.material.bind_shader_variant.v1` | `packet.render.material.bind_shader_variant.v1` | engine `88` | `mutate.binding` | shader variant binding | `deny.material.variant_missing` |
| inspect | `btn.render.material.inspect_material_truth` | `route.render.material.inspect_material_truth.v1` | `packet.render.material.inspect_material_truth.v1` | engine `88` | `read.inspect` | material truth drilldown | `deny.material.truth_missing` |
| simulate | `btn.render.material.simulate_material_preview` | `route.render.material.simulate_material_preview.v1` | `packet.render.material.simulate_material_preview.v1` | engine `88` | `simulate.preview` | material preview run | `deny.material.preview_scope_invalid` |
| compare | `btn.render.material.compare_material_triplet` | `route.render.material.compare_material_triplet.v1` | `packet.render.material.compare_material_triplet.v1` | engine `88` | `analyze.compare` | material compare digest | `deny.material.compare_baseline_missing` |
| capture | `btn.render.material.capture_material_evidence` | `route.render.material.capture_material_evidence.v1` | `packet.render.material.capture_material_evidence.v1` | engine `88` | `capture.artifact` | material evidence bundle | `deny.material.capture_target_missing` |
| recover | `btn.render.material.recover_material_baseline` | `route.render.material.recover_material_baseline.v1` | `packet.render.material.recover_material_baseline.v1` | engine `88` | `recover.baseline` | recovered material baseline | `deny.material.recovery_anchor_missing` |
| certify | `btn.render.material.certify_material_pack` | `route.render.material.certify_material_pack.v1` | `packet.render.material.certify_material_pack.v1` | engine `88` | `release.certify` | material pack verdict | `deny.material.certification_gap` |

## Exact compare modes
- material triplet compare;
- shader variant compare;
- texture truth compare;
- recovery compare;

## Required overlays and drilldowns
- material graph;
- shader variant map;
- texture bindings;
- fallback hazards;
- baseline anchors;

## Exact inspector fields
- material profile id;
- shader variant id;
- texture truth id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.material.profile_missing`;
- `disable.material.variant_unbound`;
- `disable.material.baseline_missing`;
- `disable.material.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `92` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `92` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.render.material_triplet` and pack `pack.material_shader_texture` pinned;
- capture success -> `103` or `105` with capture mode `capture.render.material_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `92` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.material_shader_texture`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.render.material.profile`;
- `artifact.render.material.binding`;
- `artifact.render.material.compare_digest`;
- `artifact.render.material.trace_ref`;
- `artifact.render.material.baseline_ptr`;

## Material-first stack obligations
- this lab must author and inspect material profiles through root `84` and root `85`;
- the operator may not treat texture binding as a substitute for archetype or response law;
- any preview for bullet hit, blast, burn, wetness, or aftermath must resolve through a response profile, not a shader-only effect;
- when a material instance stack is bound here, the owning route must preserve `material_archetype_ref`, `surface_family_ref`, `response_profile_ref`, and `material_instance_ref`.

## Required day-zero authoring buttons
- `btn.material.new_profile`
- `btn.material.assign_archetype`
- `btn.material.bind_response_profile`
- `btn.material.bind_texture_stack`
- `btn.material.preview_bullet_hit`
- `btn.material.preview_blast`
- `btn.material.preview_wetness`
- `btn.material.save_profile_as`

## Freeze relevance
- certification for `pack.material_shader_texture` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- material truth, shader-family binding, and fallback legality publish together;
- one failed material path must surface first denial code instead of hiding behind a placeholder surface.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`

## Secondary-view relation to material-centric surface
This lab may inspect texture stack, microdetail, reveal, and visual-response consequences, but the owning grammar for material law now lives in `editor/113`.
This lab is a secondary view, not the owner of material identity.
