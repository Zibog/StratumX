# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| interaction_route_id | InteractionRouteId | active routing decision identity | unique per routed interaction batch |
| input_event_batch_ref | InputEventBatchRef | input batch under routing | explicit and bounded |
| target_surface_ref | SurfaceRef | surface chosen to receive the interaction | must remain explicit |
| capture_state | InteractionCaptureState | who currently owns input capture | finite enum only |
| routing_result | RoutingResult | handled/passed/blocked result | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `interaction_routing_system` without stealing truth from neighboring levels or lower packages.
