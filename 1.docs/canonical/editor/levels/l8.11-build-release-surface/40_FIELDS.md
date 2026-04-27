# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| build_release_surface_id | BuildReleaseSurfaceId | active build/release surface identity | unique per host |
| build_status_view_ref | BuildStatusViewRef | current build status view | must resolve through tooling build/release streams |
| release_manifest_ref | ReleaseManifestRef | currently inspected release manifest | must resolve through release_runtime/artifact_plane |
| action_availability_set | BuildReleaseActionSet | legal build/release actions | must remain command-visible |
| history_filter_state | BuildHistoryFilterState | filters for past build/release entries | editor-local and explicit |

## Field law
The records above are the minimum editor-owned state needed to drive `build_release_surface` without stealing truth from neighboring levels or lower packages.
