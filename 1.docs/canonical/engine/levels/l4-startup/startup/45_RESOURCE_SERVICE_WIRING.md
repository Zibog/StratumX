# Resource Service Wiring

## Role

Startup wiring for `L1.5` services.

## Wiring Matrix

| Service class | Legal startup input | Required guarantee | Illegal posture |
|---|---|---|---|
| streaming | startup-mediated runtime-pack products | no upward crate dependency inversion | direct `engine_content` dependency |
| residency | validated manifest state + runtime-pack descriptors | profile ceilings frozen before launch | hidden post-launch governor |
| memory | selected profile allocator classes only | allocator bind frozen before launch | startup-created allocator class |
| transfer | validated products + bounded warmup artifacts | upload/readback posture stays inside law | hidden runtime decode widening |

## Rule

Startup wires resource services once, then relinquishes ownership after legal bootstrap completion.

## Failure Posture

Invalid runtime-pack products, manifest mismatches, or allocator-class mismatches fail closed before runtime ownership begins.

## Illegal Patterns

- hidden extra resource-service class;
- startup-owned persistent resource governor;
- direct upward dependency on `engine_content`.

## Local Operating Law

Startup wires resource services once and exits.
Any persistent startup resource governor is illegal by construction.
