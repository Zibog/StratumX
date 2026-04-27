# Published release-intent fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| release_intent_id | ReleaseIntentId | required | published identity for a release intent | stable across build/release handling |
| intent_kind | ReleaseIntentKind | required | class of release action requested | must use declared enum |
| target_channel | ReleaseChannel | required | target channel for the release intent | must use declared enum |
| artifact_scope | ScopeRef | required | artifact scope targeted by the release | must resolve through declared refs |
| issuer_session_id | ToolSessionId | required | session that issued the release intent | must resolve through tool-session publications |
| package_class | ReleasePackageClass | required | package profile requested by the issuer | must use declared enum |
| publish_mode | PublishMode | required | dry-run, stage, or publish mode | must use declared enum |

## Publication law
This file freezes the externally visible publication contract for release intents. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
