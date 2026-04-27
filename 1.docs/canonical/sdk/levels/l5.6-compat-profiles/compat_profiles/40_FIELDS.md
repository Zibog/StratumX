# Compatibility Profiles Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| profile_id | CompatProfileId | required | stable profile identity | unique in registry |
| profile_name | ProfileName | required | stable compatibility profile name | must remain API-visible and stable |
| allowed_version_set | CompatVersionSet | required | versions admitted by this profile | must reference declared versions only |
| capability_set | CompatCapabilitySet | required | capabilities admitted by this profile | must reference declared capabilities only |
| fallback_profile_id | CompatProfileId | optional | fallback profile used for downgrade | must not create cycles |

## Local invariant rule
Each field above exists because `compat_profiles` must publish compatibility profiles without absorbing adjacent semantic truth.
