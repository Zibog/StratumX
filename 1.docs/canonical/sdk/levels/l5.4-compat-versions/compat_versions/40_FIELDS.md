# Compatibility Versions Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| compat_version_id | CompatVersionId | required | stable identity of one protocol version entry | globally unique within the package |
| wire_version | WireVersion | required | version string or numeric wire code | must be immutable once published |
| schema_generation | SchemaGeneration | required | schema epoch for envelope interpretation | must advance only by declared migration rules |
| supersedes_version_id | CompatVersionId | optional | previous version replaced by this entry | must point only backward |
| support_state | SupportState | required | supported/deprecated/denied state | must use declared enum only |

## Local invariant rule
Each field above exists because `compat_versions` must publish compatibility versions without absorbing adjacent semantic truth.
