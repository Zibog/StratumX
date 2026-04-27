# Observation Scope Rate And Retention Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze observation cadence, retention, throttling, and scope rules for heavy domains.

## Observation classes

| Observation class | Scope | Default cadence | Retention | Delivery posture |
|---|---|---|---|---|
| live-per-frame | view/window scoped | every frame or declared degraded cadence | transient unless captured | subscribed stream |
| event-driven | domain/object scoped | on exact event | retained event ledger | retained event emit |
| snapshot-only | world/domain scoped | operator or checkpoint triggered | retained by explicit artifact policy | explicit query or checkpoint |
| debug-only | lab scoped | on demand | non-freeze unless promoted | demand-driven |
| certification-only | pack/scenario scoped | explicit run step | immutable retained artifact | explicit artifact emission |

## Consumer throttling matrix

| Consumer class | Allowed classes | Notes |
|---|---|---|
| live viewport | live-per-frame, event-driven | must declare target scope and degrade posture |
| focused inspector | live-per-frame, event-driven, snapshot-only | may request heavy slices only while focused |
| why/debug surface | event-driven, debug-only, snapshot-only | heavy traces may not stay permanently subscribed |
| certification runner | certification-only, snapshot-only, event-driven | immutable artifact posture required |
| background shell chrome | event-driven only | shell may not subscribe to heavy firehose streams |

## Observation law
- no heavy-domain packet may default to firehose mode in all surfaces;
- every editor surface must declare what it subscribes to and at what rate;
- degraded cadence must publish itself explicitly;
- retention downgrades are forbidden once a run enters certification posture.

## Failure families
- `OBS_SCOPE_*`: illegal scope or missing view/window context;
- `OBS_RATE_*`: cadence above the legal class;
- `OBS_RETAIN_*`: illegal retention downgrade or missing artifact class;
- `OBS_SUB_*`: undeclared subscriber or consumer class.
