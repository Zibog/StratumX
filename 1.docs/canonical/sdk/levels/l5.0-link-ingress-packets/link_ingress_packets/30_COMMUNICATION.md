# Link Ingress Packets Local Communication

## Sends or publishes
- receives opaque ingress packets from public L4 transport receivers
- publishes accepted packet envelopes to split/route stages
- emits bounded reject records when policy or legality fails

## Receives or resolves
- L4 ingress packet publish surface
- packet rejection surface
- decode-to-control split handoff
- decode-to-observation split handoff

## Local law
The communication contour above is exhaustive for `link_ingress_packets`.
