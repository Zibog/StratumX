# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| bootstrap_request_id | BootstrapRequestId | active bootstrap request | unique per invocation |
| project_manifest_ref | ProjectManifestRef | manifest being created or opened | typed and explicit |
| content_mount_plan_ref | ContentMountPlanRef | mount plan for project content | must remain explicit |
| workspace_seed_ref | WorkspaceSeedRef | seed layout or template refs | typed and explicit |
| bootstrap_result_ref | BootstrapResultRef | result of the bootstrap flow | bounded and publishable |

## Field law
The records above are the minimum editor-owned state needed to drive `project_bootstrap_service` without stealing truth from neighboring levels or lower packages.
