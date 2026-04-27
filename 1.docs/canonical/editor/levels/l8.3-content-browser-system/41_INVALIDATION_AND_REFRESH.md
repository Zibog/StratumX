# Invalidation And Refresh

This contract belongs specifically to the content browser system editor level and is expected to become direct implementation work.


## Refresh triggers for `content_browser_system`
- dependency change in tooling index_plane that affects `content_browser_view_id`
- dependency change in artifact_plane that affects `asset_listing_ref`
- dependency change in validation_runtime that affects `filter_state`
- dependency change in import-export pipeline service that affects `selection_ref_set`
- explicit user action changing `content_browser_view_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `content_browser_system`

## Invalidation law
Refresh must name stale records such as `content_browser_view_id`, `asset_listing_ref`, `filter_state`, `selection_ref_set` rather than silently rebuilding hidden mirrors.
