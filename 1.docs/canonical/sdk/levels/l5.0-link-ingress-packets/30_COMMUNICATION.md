# Communication

## Publication classes
- receives opaque ingress packets from public L4 transport receivers
- publishes accepted packet envelopes to split/route stages
- emits bounded reject records when policy or legality fails

## Synchronization surfaces
- L4 ingress packet publish surface
- packet rejection surface
- decode-to-control split handoff
- decode-to-observation split handoff

## Communication law
Communication in `link_ingress_packets` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.
