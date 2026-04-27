# STRATUMX_SDK_L5_PACKET_AND_ENVELOPE_LAW

## Scope
This law distinguishes raw bridge payloads from the envelopes that move them.

## Binding laws
- packets carry subject facts or control facts;
- envelopes carry routing, ordering, or transport metadata around packets;
- packet semantics may not be hidden in envelope-only fields, and transport metadata may not masquerade as semantic payload.

## Audit checks
- packet docs name semantic payload fields;
- transport/order metadata is explicit and bounded;
- downstream consumers can separate content from carriage without guessing.
