# 21 GENERATION TRIANGLE SEPARATION

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law separates three concerns that often get conflated during content-generation workflows.

## Triangle vertices
- **fact vertex**: deterministic bridge facts from `L4/L5` such as refs, cursors, snapshots, verdicts, and artifact refs;
- **proposal vertex**: assistant or tool suggestions about what might be generated or changed;
- **application vertex**: actual accepted mutations applied by legal tooling/editor layers.

## Separation rules
- fact publications may inform proposals but do not become proposals;
- proposals may mention prospective artifacts or edits but do not become applied truth;
- applied mutations must pass through legal upper-stack request/result paths and may not be back-written into `L5` as if they were facts of origin.

## Why this matters
The dream editor needs assistant help, procedural generation, and reproducible authoring. Those remain tractable only if raw facts, proposed changes, and accepted changes stay in separate semantic buckets.

## Audit checks
- `L5` contains fact-vertex rows only;
- any generation/proposal wording appears above `L5`;
- applied changes are traceable to legal upper-layer authority rather than hidden bridge mutation.
