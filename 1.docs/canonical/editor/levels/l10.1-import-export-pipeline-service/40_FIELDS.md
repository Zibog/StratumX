# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| pipeline_request_id | PipelineRequestId | active import/export request | unique per invocation |
| source_path_ref | SourcePathRef | source file or folder path being processed | explicit and bounded |
| import_profile_ref | ImportProfileRef | profile applied during import | typed and explicit |
| generated_artifact_set | GeneratedArtifactSet | artifacts produced by the pipeline | must resolve through artifact plane |
| pipeline_status | PipelineStatus | queued/running/warn/error/ready state | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `import_export_pipeline_service` without stealing truth from neighboring levels or lower packages.


## Exact import pipeline field labels
- asset_id
- source_path
- importer_type
- import_profile
- last_import_hash
- generated_artifact_ids[]
- dependency_ids[]
- platform_targets[]
- compression
- cook_rule
- streaming_group
- status
- warnings_count
- errors_count
