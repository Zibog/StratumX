# Communication

## Publication classes
- receives control requests from L4 public control APIs
- publishes accepted control envelopes for downstream authority handling
- emits deterministic rejection reasons for unsupported or illegal control classes

## Synchronization surfaces
- L4 control submit surface
- ingress control publication surface
- control rejection stream
- control acknowledgement mirror

## Communication law
Communication in `link_ingress_controls` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.
