# Compatibility Capabilities Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| capability_id | CompatCapabilityId | required | stable identity of one capability | unique in registry |
| capability_name | CapabilityName | required | human-readable and API-stable capability name | must remain stable across patch versions |
| availability_profile_id | CompatProfileId | required | profile that grants or denies the capability | must resolve through `compat_profiles` |
| default_state | CapabilityDefaultState | required | default allowed/denied state | must use declared enum |
| deprecation_note | CapabilityDeprecationNote | optional | bounded migration note for deprecated capability | must not redefine behavior itself |

## Local invariant rule
Each field above exists because `compat_capabilities` must publish compatibility capabilities without absorbing adjacent semantic truth.
