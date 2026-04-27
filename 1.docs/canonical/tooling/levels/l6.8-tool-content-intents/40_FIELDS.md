# Published content-intent fields

## Published canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| content_intent_id | ContentIntentId | required | published identity for a content intent | stable across request/result flow |
| intent_kind | ContentIntentKind | required | kind of content authoring action requested | must use declared enum |
| target_asset_ref | AssetRef | optional | target asset or content subject | required when the intent addresses an existing asset |
| issuer_session_id | ToolSessionId | required | session that issued the content intent | must resolve through tool-session publications |
| intent_scope | ScopeRef | required | scope the content intent applies to | must resolve through declared refs |
| intent_priority | IntentPriorityBand | optional | priority band for the content action | must use declared enum when present |

## Publication law
This file freezes the externally visible publication contract for content intents. Adjacent tooling or editor consumers may rely on these rows directly.

## Non-goals
- no hidden private bookkeeping fields are implied by this publication inventory;
- any richer runtime coordination belongs in the nested sidecar contract, not in this published projection.
