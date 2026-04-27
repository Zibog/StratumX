# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| package_service_session_id | PackageServiceSessionId | active package service session | unique per host |
| package_listing_ref | PackageListingRef | current package listing or search result | typed and explicit |
| dependency_graph_ref | DependencyGraphRef | current dependency graph projection | typed and explicit |
| mount_plan_ref | MountPlanRef | content/runtime/editor mount plan for packages | explicit and bounded |
| package_action_set | PackageActionSet | legal install/update/remove/enable actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `package_market_and_dependency_service` without stealing truth from neighboring levels or lower packages.


## Exact package model field labels
- package_id
- version
- owner_layer
- dependencies[]
- optional_dependencies[]
- editor_tools[]
- runtime_exports[]
- validation_rules[]
- test_suites[]
- content_mounts[]
