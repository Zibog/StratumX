# Network Role Selection

## Role

Startup bind for network role classes.

## Role Matrix

| Role class | Legal bind | Failure posture | Illegal posture |
|---|---|---|---|
| no-network local role | explicit local-only bind | refuse hidden network enablement | implicit session role |
| interactive host-aware role | explicit local presentation role only | refuse remote burden widening | hidden host takeover |
| listen-host role | explicit host bind under profile ceilings | refuse peer count/profile mismatch | post-launch host widening |
| headless host role | explicit headless host bind | fail closed on unsupported mix | presentation-bearing headless role |

## Rule

Network role selection is startup-owned, explicit, and complete before runtime and transport ownership begin. Network role bind may not be widened after bootstrap.

## Role-Bind Matrix

| Role bind concern | Requirement | Illegal posture |
|---|---|---|
| peer-count entitlement | frozen before launch | runtime-side expansion |
| transport/session class | frozen before launch | implicit network enablement |
| role widening | forbidden after bootstrap | hidden host takeover |

## Local Operating Law

Network role selection seals host burden before transport begins owning live session state.
That seal exists specifically to block silent role drift.
