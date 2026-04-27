# tool_focus_refs communication

Ingress:
- focus-change publications from inspector or viewport
- pin/unpin focus actions
- focus clear or retarget updates

Egress:
- stable focus refs keyed by focus_ref_id
- focus owner surface descriptors
- focus epoch notifications

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.
