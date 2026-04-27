# Link Ingress Controls Local Communication

## Sends or publishes
- receives control requests from L4 public control APIs
- publishes accepted control envelopes for downstream authority handling
- emits deterministic rejection reasons for unsupported or illegal control classes

## Receives or resolves
- L4 control submit surface
- ingress control publication surface
- control rejection stream
- control acknowledgement mirror

## Local law
The communication contour above is exhaustive for `link_ingress_controls`.
