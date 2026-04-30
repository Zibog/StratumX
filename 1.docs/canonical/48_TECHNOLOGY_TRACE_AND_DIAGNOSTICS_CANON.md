# Technology Trace and Diagnostics Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Guarantee one explainable drilldown path for truth publication, denial, degradation, certification outcome, and placeholder visibility.

## Trace node minimum fields
- `trace_id`
- `parent_trace_id`
- `technology_family`
- `owner_package`
- `phase`
- `resource_vector`
- `result_code`
- `artifact_ref?`
- `timestamp_bucket`

## Required drilldown classes
- boundary failure
- denial code explanation
- degrade-step explanation
- recovery explanation
- certification mismatch explanation
- placeholder or stub visibility explanation

## Placeholder and stub law
- placeholder = declared shape exists but behavior is intentionally inert;
- stub = one legal surface exists but inner implementation is absent;
- specified-not-proved = document route exists but code breadth is still open;
- silent fake completeness is forbidden.

## Law
Every failure that reaches editor must preserve the originating owner package, the denied phase, the next legal operator action, and any placeholder or stub class involved.
