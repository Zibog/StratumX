# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| review_surface_id | ReviewSurfaceId | active review surface identity | unique per host |
| annotation_set_ref | AnnotationSetRef | current annotation collection | typed and explicit |
| review_target_ref | ReviewTargetRef | asset/scene/sequence under review | explicit and bounded |
| comment_thread_ref | CommentThreadRef | selected comment thread | publishable and explicit |
| review_action_set | ReviewActionSet | legal approve/reject/comment actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `review_annotation_surface` without stealing truth from neighboring levels or lower packages.
