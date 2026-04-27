# Project Bootstrap and Template Selection Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how a legal project begins.

## Required command set
- `editor.project.create`
- `editor.project.template.select`
- `editor.project.profile.bind`
- `editor.project.workspace.provision`
- `editor.project.bootstrap.validate`
- `editor.project.bootstrap.evidence.append`

## Template contract
| Field | Meaning | Required |
|---|---|---|
| template_id | stable template identity | yes |
| template_family | world/gameplay/product intent class | yes |
| supported_profiles | platform/build profiles legal for the template | yes |
| required_capabilities | engine/sdk/tooling/editor prerequisites | yes |
| starter_content_manifest | initial content payloads or references | optional |
| schema_revision | template schema revision | yes |
| migration_floor | oldest compatible template revision | yes |
| deprecation_state | live, restricted, deprecated, blocked | yes |

## Bootstrap transaction
| Phase | Expectation | Failure codes |
|---|---|---|
| resolve template | template exists and is not blocked | TEMPLATE_NOT_FOUND, TEMPLATE_DEPRECATED_BLOCKED |
| bind project identity | project id and namespace become durable | PROJECT_ID_COLLISION, NAMESPACE_DENIED |
| provision workspace | workspace tree and initial save targets are writable | WORKSPACE_CREATE_DENIED, SAVE_TARGET_UNAVAILABLE |
| apply defaults | template defaults lower into project settings | DEFAULT_PROFILE_MISMATCH, CAPABILITY_GAP |
| validate bootstrap | first bootstrap verdict is emitted | BOOTSTRAP_VALIDATION_FAILED |

## Mandatory outputs
- one `project_id`
- one `workspace_root`
- one `template_id`
- one `default_profile_ref`
- one `bootstrap_verdict`
- one `bootstrap_evidence_ref`

## Disabled reasons
- `NoWritableLocation`
- `TemplateBlocked`
- `ProfileUnsupported`
- `CapabilityGap`
- `NamespaceCollision`
- `BootstrapValidationBusy`
