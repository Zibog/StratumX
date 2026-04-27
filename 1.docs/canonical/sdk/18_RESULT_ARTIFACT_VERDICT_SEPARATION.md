# 18 RESULT ARTIFACT VERDICT SEPARATION

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law prevents three different bridge products from collapsing into one ambiguous publication class.

## Classes
- **result**: direct outcome row from an engine-side or bridge-side operation, usually consumed immediately;
- **artifact**: durable produced output or immutable generated asset reference, suitable for caching, packaging, or replay;
- **verdict**: legality/compatibility decision row explaining whether something is allowed, blocked, degraded, or unsupported.

## Separation rules
- a result row may point at an artifact ref, but it is not itself the artifact;
- a verdict may explain whether a result or artifact is legal, but it is not either of them;
- artifacts never carry approval semantics by implication; approval must come from verdict rows;
- results may be transient, artifacts are stable enough to reference, verdicts are auditable classification outputs.

## Required fields by class
- result rows must expose origin, cursor/epoch, and subject/ref scope;
- artifact rows must expose stable artifact identity, production cursor, and reproducibility metadata;
- verdict rows must expose subject, rule source, decision class, and downgrade/block reason when applicable.

## Failure conditions
The canon is broken if any document treats artifact production as equivalent to verdict publication, or if runtime/build tooling must guess whether a row is a result, an artifact, or a verdict.
