# Product Relay Progress State And Status Bar Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Status surfaces must reflect `sdk/77` lifecycle exactly:
- ack accepted or ack denied;
- progress bound;
- progress running;
- partial result;
- terminal success;
- retryable failure;
- terminal failure.

The status bar must also show:
- current route id;
- current packet family id;
- current proof-region recipe ref;
- current focus target id;
- next legal recovery action;
- freeze relevance;
- whether baseline / failed / recovery runs are already retained;
- whether first-result verification is still pending.
