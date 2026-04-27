# Engine Identity Refs Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| identity_ref | EngineIdentityRef | required | stable public identity ref | must remain stable across sessions when policy allows |
| identity_class | IdentityClass | required | closed identity class enum | must use declared enum |
| external_name | ExternalIdentityName | optional | publicly visible stable name | must not become mutable truth owner |
| visibility_scope | IdentityVisibilityScope | required | scope where the identity may be resolved | must be explicit and bounded |
| status | IdentityRefStatus | required | active/stale/revoked state | must use declared enum |

## Local invariant rule
Each field above exists because `engine_identity_refs` must publish engine identity refs without absorbing adjacent semantic truth.
