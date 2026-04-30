# Audio Capability Matrix

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Map audio capability families to the active production contour using one vocabulary: truth owner, owner surfaces, button namespace, canonical pack ids, and the root sound delivery chain frozen in `43`.

## Matrix
| Capability family | Root chain role | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| event classification and timing | event emission, category, subtitle/voice relation | `engine/93` | `editor/62 + editor/95` | audio command rows in `editor/110` | `pack.audio_mix_occlusion_variation` | `doc_closed_impl_open` |
| runtime emitters and listeners | emitter/listener resolution and context selection | `engine/57 + engine/93` | `editor/62 + editor/95` | audio command rows in `editor/110` | `pack.audio_mix_occlusion_variation` | `doc_closed_impl_open` |
| propagation, occlusion, and zones | spatial path, obstruction, zone/reverb context | `engine/57 + engine/93` | `editor/62 + editor/95` | audio command rows in `editor/110` | `pack.audio_mix_occlusion_variation` | `doc_closed_impl_open` |
| mix, priority, and ducking | audible set, bus ownership, ducking and voice steal | `engine/58 + engine/93` | `editor/62 + editor/95` | audio command rows in `editor/110` | `pack.audio_mix_occlusion_variation` | `doc_closed_impl_open` |
| streaming, decode, and output device | stream/decode readiness, fallback, output presentation | `engine/58 + engine/93` | `editor/95` | audio command rows in `editor/110` | `pack.audio_mix_occlusion_variation` | `doc_closed_impl_open` |

## Law
These matrices may not invent local namespaces, local pack aliases, or local route names outside root `43`, `71`, `72`, `74`, `76`, `81`, and `editor/110`.


## Dream-stack expansion rows
| Capability family | Root chain role | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| diffraction and layered occlusion | deep path legality and material-informed propagation | `engine/128 + engine/57 + engine/93` | `editor/62 + editor/123` | audio rows in `editor/110` | `pack.deep_audio_soundscape` | `document_gold` |
| large-world soundscape continuity | streaming continuity for weather, crowds, fires, and distant theater | `engine/128 + engine/60 + engine/114 + engine/115` | `editor/62 + editor/122 + editor/123 + editor/126` | audio/cert rows in `editor/110` | `pack.deep_audio_soundscape + pack.distant_world_representation` | `document_gold` |
| material-first surface-event variation | footsteps/impacts tied to material-pair rows and state modifiers | `engine/116 + engine/128` | `editor/119 + editor/123` | audio/material rows in `editor/110` | `pack.material_pair_contact + pack.deep_audio_soundscape` | `document_gold` |

---

# V33 audio authoring closure assimilation

Stack version: `SX-CANON/1.0.27/STACK-v33`

## Audio capability completion requirement

Audio is complete as a documented engine system only when authoring and runtime delivery are both covered:

| Capability | Minimum completion |
|---|---|
| Authoring graph | Event nodes, layers, variations, material routing, bus routing, and preview state are described. |
| Bank pipeline | Source files, bank manifests, cook, streaming chunks, memory tiers, and missing asset behavior are described. |
| Material sound matrix | Surface × event × state variants for steps, impacts, scrapes, breaks, rain, fire, water, and cloth are described. |
| Occlusion/portal | Indoor/outdoor, room/portal, obstruction, diffraction approximation, and recovery are described. |
| Audition/proof | Editor audition, compare capture, loudness/priority diagnostics, and certification packs are described. |
