# Invalidation And Refresh

This contract belongs specifically to the import export pipeline service editor level and is expected to become direct implementation work.


## Refresh triggers for `import_export_pipeline_service`
- dependency change in content browser that affects `pipeline_request_id`
- dependency change in build-release surface that affects `source_path_ref`
- dependency change in tooling build/artifact/validation families that affects `import_profile_ref`
- explicit user action changing `pipeline_request_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `import_export_pipeline_service`

## Invalidation law
Refresh must name stale records such as `pipeline_request_id`, `source_path_ref`, `import_profile_ref`, `generated_artifact_set` rather than silently rebuilding hidden mirrors.
