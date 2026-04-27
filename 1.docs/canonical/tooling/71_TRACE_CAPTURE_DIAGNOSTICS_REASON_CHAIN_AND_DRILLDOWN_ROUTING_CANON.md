# Trace Capture Diagnostics Reason Chain And Drilldown Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the routing law for diagnostics, blocker drilldown, assistant apply lineage, and evidence drillthrough.

## Intent families
- `intent.trace.reason_summary`
- `intent.trace.blocker_triplet`
- `intent.trace.assistant_apply_lineage`
- `intent.trace.revert_lineage`

## Focus law
Success focus targets:
- viewport rail when render/capture issues are visual;
- assistant dock when proposal lineage is the primary question;
- extension manager when capability or mount scope is blocking.

Failure focus targets:
- exact blocker subsection only;
- never generic diagnostics home.

## Retention law
Trace routes retain:
- trace ids
- blocker codes
- affected routes
- rollback anchors when applicable
- evidence artifact refs when a capture exists
