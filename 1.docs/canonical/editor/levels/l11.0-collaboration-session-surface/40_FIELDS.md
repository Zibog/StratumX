# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| collab_surface_id | CollabSurfaceId | active collaboration surface identity | unique per host |
| session_presence_ref | SessionPresenceRef | current collaboration presence projection | typed and explicit |
| participant_set_ref | ParticipantSetRef | participants visible in the session | bounded and publishable |
| permission_view_ref | PermissionViewRef | current permission or role projection | typed and explicit |
| collab_action_set | CollabActionSet | legal join/leave/share actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `collaboration_session_surface` without stealing truth from neighboring levels or lower packages.
