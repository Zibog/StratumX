# Photoreal Render Feature Tier And Fallback Packet Catalog

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define SDK packets for photoreal rendering tiers, old-hardware fallback, visual feature posture, and certification proof.

## Packet families
| Packet | Purpose |
|---|---|
| `packet.render.feature_tier_report.v1` | active/max render tier |
| `packet.render.shadow_tier_report.v1` | shadow path and fallback |
| `packet.render.texture_residency_report.v1` | resident mips, missing resources, memory pressure |
| `packet.render.material_visual_truth_report.v1` | material visual channels and missing slots |
| `packet.render.photoreal_proof_result.v1` | terrain/sky/tunnel/muzzle proof outcome |
| `packet.render.old_floor_fallback_report.v1` | degraded features and performance posture |

## Required consumers
- editor photoreal lab;
- viewport cockpit;
- tooling capture/compare routes;
- quality golden corpus;
- release readiness matrix.

## Current posture
`document_gold / photoreal_packet_catalog_defined / implementation_open`


---
# V32 SDK Closure: Photoreal/Fallback Packets

## Required packets
| Packet | Purpose |
|---|---|
| `packet.graphics.feature_tier.v1` | active/requested render tier |
| `packet.graphics.visual_fallback.v1` | feature degradation report |
| `packet.graphics.material_visual_truth.v1` | material channels and fallbacks |
| `packet.graphics.output_color_chain.v1` | exposure/tonemap/output metadata |
| `packet.graphics.shadow_tier.v1` | shadow mode/tier/disabled verdict |
| `packet.graphics.old_floor_profile.v1` | old-hardware profile and active rungs |
| `packet.graphics.golden_frame_expectation.v1` | expected proof frame metadata |

## Fallback fields
Feature id, requested tier, active tier, backend verdict, device verdict, policy verdict, visual consequence, editor badge severity, and evidence requirement.
