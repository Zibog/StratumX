# STRATUMX_SDK_L5_FIELD_SEMANTICS_CONSTITUTION

## Scope
This constitution freezes how `L5` field contracts must behave.

## Binding laws
- every field belongs to a declared semantic class such as packet, observation, metric, verdict, handle, ref, cursor, epoch, or artifact reference;
- field names must reveal subject, class, and freshness expectations when relevant;
- field rows must declare invariants strong enough for tooling/editor consumers to use them without reinterpretation.

## Audit checks
- `40_FIELDS.md` files stay typed and domain-specific;
- no field row is justified only by convenience or local implementation preference;
- invariants remain compatible with registry and readiness artifacts.
