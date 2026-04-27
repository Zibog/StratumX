# 33 HANDLE AND REF OPACITY LAW

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law defines what it means for `L5` handles and refs to remain opaque while still being useful to the editor/tooling stack.

## Opaque classes
- session handles;
- object handles;
- runtime handles;
- identity refs;
- state refs;
- artifact refs.

## Opacity rules
- consumers may compare, store, route, snapshot, and publish these values;
- consumers may not infer private engine memory layout, ownership, or mutation rights from the bit-pattern or string shape;
- any dereference must go through declared public surfaces, never by structural guessing;
- ref kind and subject class must remain explicit even when the payload itself is opaque.

## Bridge obligations
`L5` must still provide enough metadata for upper tooling to use opaque values safely:
- stable type class;
- resolution surface or publication domain;
- cursor/epoch association when freshness matters;
- legality/verdict association when consumption may be blocked.

## Failure conditions
The canon fails if a handle/ref can be used as a secret capability token, if its shape implies hidden layout, or if upper layers must crack it open to recover semantics.
