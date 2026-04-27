# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| scaffold_request_id | ScaffoldRequestId | active scaffold request | unique per creation flow |
| template_ref | TemplateRef | template or preset being applied | typed and explicit |
| target_scope | ScaffoldTargetScope | scope where scaffold output will land | explicit |
| generated_item_set | GeneratedItemSet | items created by the scaffold | must remain bounded |
| scaffold_result_ref | ScaffoldResultRef | result summary for the scaffold flow | publishable and explicit |

## Field law
The records above are the minimum editor-owned state needed to drive `template_preset_and_scaffold_service` without stealing truth from neighboring levels or lower packages.
