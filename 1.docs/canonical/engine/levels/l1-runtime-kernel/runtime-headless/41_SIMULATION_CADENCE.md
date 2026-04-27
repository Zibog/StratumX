# Simulation Cadence

## Role

Headless profile cadence contract.

## Data Model

Numeric Authority: `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` §3, §5, §10

Headless cadence owns authoritative tick pacing, no presentation path, and legal coordination with network, replay, storage, and verification pressure on `headless-20`.

## Cadence Matrix

| Concern | Canonical law | Illegal posture |
|---|---|---|
| authoritative cadence | fixed 20 Hz | variable authoring cadence |
| tick budget | 50.00 ms hard budget | hidden wider tick entitlement |
| integrated proof | must pass `mixed-pressure-headless-a` | proof by isolated microbench only |
| replay interaction | replay may consume only frozen reserve | replay widens tick cadence |
| storage interaction | storage and verification may not steal authoritative cadence | hidden IO back-pressure on tick |
| network burden | remote burden stays inside headless profile ceilings | unbounded peer expansion |

## Operating Matrix

| Phase concern | Required behavior | Failure posture |
|---|---|---|
| tick start | runtime-owned cadence only | refuse service-owned tick pacing |
| publication | bounded headless export only | shed noncritical publication first |
| backlog response | obey pressure-response latency | degrade optional work before authority work |
| verification | defer noncritical verification before cadence breach | never widen tick window |

## Illegal Patterns

- cadence stretching after queue breach;
- borrowing realtime presentation rules;
- masking overload by skipping authoritative publication;
- hidden headless queue reservoir.

## Ownership Matrix

| Ownership concern | Canonical owner | Illegal owner |
|---|---|---|
| authoritative tick pace | runtime headless shell | service-owned cadence |
| tick overrun response | runtime degradation order | replay or storage loops |
| headless export windows | runtime-owned publication slots | transport-owned pacing |

## Pressure Priority Matrix

| Pressure source | Preserve first | Degrade first |
|---|---|---|
| storage pressure | authoritative cadence | noncritical verification |
| replay pressure | authoritative cadence | optional compare/report work |
| network pressure | authoritative cadence | noncritical publication |

## Local Operating Law

Headless cadence is a hard operating law, not a tuning hint.
No local contract may widen the 20 Hz authority step, borrow realtime presentation reserve, or hide tick debt inside deferred publication or deferred IO.
