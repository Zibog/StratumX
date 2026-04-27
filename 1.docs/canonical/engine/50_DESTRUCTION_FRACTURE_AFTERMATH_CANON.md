# Destruction Fracture Aftermath Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define exact material response, terrain blast morphology, support failure, tree fracture, debris aftermath, and publication law for phase-2 material-world proof.

## Exact truth objects
| Truth object | Role | Authoritative fields | Publication scope |
|---|---|---|---|
| `material_response_profile` | material taxonomy row | `fracture_class`, `debris_class`, `crater_response`, `wetness_response`, `ignition_class`, `penetration_class`, `sound_class`, `persistence_class` | simulation + editor inspect |
| `damage_topology_state` | compact local damage truth | `field_family`, `breach_digest`, `release_groups`, `support_groups`, `inner_face_digest` | destruction + render + physics + nav |
| `crater_morphology_state` | terrain displacement result | `surface_family`, `rim_profile`, `void_depth`, `ejecta_family`, `settle_window` | destruction + terrain + audio |
| `support_failure_graph` | structure/tree support map | `bearing_nodes`, `load_paths`, `critical_supports`, `cascade_window` | destruction + traversal + causality |
| `debris_persistence_state` | aftermath persistence | `debris_class`, `lifetime_tier`, `collision_policy`, `cleanup_policy` | streaming + persistence + diagnostics |
| `secondary_collision_state` | branch/chunk secondary consequence | `impact_chain`, `rest_state`, `aftermath_delta` | physics + destruction + audio |
| `fragment_absorption_state` | merged fragment residue | `host_family`, `mass_bucket`, `residue_digest`, `cover_delta` | persistence + streaming + diagnostics |

## Exact phase order
| Phase | Input | Output | Illegal shortcut |
|---|---|---|---|
| `classify_material` | material request | canonical material response row | ad-hoc per-scene material override |
| `apply_impulse_or_blast` | impact / blast carrier | crater or fracture candidate set | visual-only crater without truth row |
| `mutate_damage_topology` | candidate set + support hints | breach, crack, release, and support digests | shader-only crack with no topology state |
| `solve_fracture_and_cascade` | topology state + support graph | fracture result + support updates | collapse without support graph update |
| `publish_aftermath` | fracture result | debris / residue / trace / compare payload | cleanup before persistence anchor captured |

## Exact compare / capture / certification law
| Family | Allowed ids | Pack |
|---|---|---|
| material response | `compare.material.response_triplet`, `capture.material.response_bundle` | `pack.material_surface_response` |
| terrain blast | `compare.terrain.blast_triplet`, `capture.terrain.blast_bundle` | `pack.terrain_crater_truth`, `pack.terrain_blast_degrade` |
| structure aftermath | `compare.struct.aftermath_triplet`, `capture.struct.aftermath_bundle` | `pack.structure_cascade_aftermath`, `pack.secondary_collision_aftermath` |

## Phase-2 brutal proof slices
- grenade in mud forms soft crater, dirt ejecta, and wetness-aware aftermath;
- grenade on asphalt forms chipped crater, shard ejecta, and different sound/penetration truth;
- tree trunk fracture publishes branch break and secondary collision aftermath;
- wall or slab collapse updates support graph and debris persistence before compare/capture;
- detached fragments settle, merge into a lawful host, and publish residue summary instead of remaining immortal rigid bodies.

## Failure and denial families
| Family | Meaning | Retryable | Required artifact / trace |
|---|---|---|---|
| `fail.material_profile_missing` | surface family lacks canonical row | no | `artifact.material.registry_gap`, `trace.material.lookup` |
| `fail.crater_response_unbound` | blast response lacks morphology rule | no | `artifact.terrain.profile`, `trace.terrain.blast` |
| `fail.support_graph_incomplete` | progressive collapse cannot be solved legally | yes | `artifact.support.graph`, `trace.support.failure` |
| `fail.damage_topology_missing` | reveal exists but no lawful topology digest was published | no | `artifact.damage.topology`, `trace.damage.reveal` |
| `fail.aftermath_persistence_denied` | debris or residue row cannot be retained | yes | `artifact.persistence.anchor`, `trace.aftermath.persist` |

## Resource envelope and degrade law
| Axis | Nominal law | Degrade rung | May never be faked |
|---|---|---|---|
| CPU | green <= 2.8 ms combined solve | reduce distant aftermath settle cadence and fragment richness | structural state publication |
| RAM | green <= 448 MiB live destruction state | compact cold debris histories and summarize old damage fields | support graph identity |
| GPU overlays | green <= 0.8 ms | sample crater / debris overlays more sparsely | first-fail visibility |
| IO | blocking hot-path write forbidden | defer non-critical artifact flush | persistence anchor sealing |

## Current posture
`document_gold / doc_closed_impl_open`
