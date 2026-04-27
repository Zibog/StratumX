# Invalidation And Refresh

This contract belongs specifically to the review annotation surface editor level and is expected to become direct implementation work.


## Refresh triggers for `review_annotation_surface`
- dependency change in content browser that affects `review_surface_id`
- dependency change in viewport system that affects `annotation_set_ref`
- dependency change in production dashboard that affects `review_target_ref`
- explicit user action changing `review_surface_id` or related scope/mode
- build/validation/preview/runtime status change relevant to `review_annotation_surface`

## Invalidation law
Refresh must name stale records such as `review_surface_id`, `annotation_set_ref`, `review_target_ref`, `comment_thread_ref` rather than silently rebuilding hidden mirrors.
