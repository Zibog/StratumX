# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| profile_id | CompatProfileId | required | stable profile identity | unique in registry |
| profile_name | ProfileName | required | stable compatibility profile name | must remain API-visible and stable |
| allowed_version_set | CompatVersionSet | required | versions admitted by this profile | must reference declared versions only |
| capability_set | CompatCapabilitySet | required | capabilities admitted by this profile | must reference declared capabilities only |
| fallback_profile_id | CompatProfileId | optional | fallback profile used for downgrade | must not create cycles |

## No hidden store law
All semantic truth in `compat_profiles` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
