# Build and Freeze Conditions

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the conditions under which implementation-start, package-complete, and gold-freeze operations are permitted within the StratumX engine package.

## Implementation-Start Conditions
Implementation-start is allowed when:
- The engine's root documents (00_INDEX.md through 03_PACKAGE_ROLE_MAP.md) exist and are internally consistent
- The engine's dependency model does not violate the global dependency model
- No constitutional law prohibits implementation in the affected domain

## Package-Complete Conditions
Package-complete is allowed when:
- All acceptance matrix rows for the engine package are in `pass` status
- Every `pass` row references an active artifact in the engine's evidence registry
- The engine's evidence registry contains only active artifacts that exist
- No duplicate ordinals exist in the engine's root documents
- No wildcard/glob authority references exist where exact canonical paths are required
- All hot/core layers have full local documentation packages (not single-file contracts)

## Gold-Freeze Conditions
Engine gold-freeze is allowed ONLY when:
- The engine package has achieved package-complete status
- Every row in the Engine Acceptance Matrix (`14_ACCEPTANCE_MATRIX.md`) is in `pass` status
- Every `pass` row in the Engine Acceptance Matrix references an active artifact in the Engine Evidence Registry (`15_EVIDENCE_REGISTRY.md`)
- The Engine Evidence Registry contains only active artifacts that exist
- The Engine Audit Readiness Matrix (`16_AUDIT_READINESS_MATRIX.md`) shows full alignment with engine acceptance and evidence
- The engine's version marker (`STACK_VERSION`) is present and correctly formatted
- No engine-specific constitutional violations that affect package integrity

## Prohibited Conditions
Engine gold-freeze is PROHIBITED if:
- The engine package has not achieved package-complete status
- Any row in the Engine Acceptance Matrix is not in `pass` status
- Any `pass` row in the Engine Acceptance Matrix references a non-existent or inactive artifact
- The Engine Evidence Registry references any non-existent artifact
- The engine contains duplicate ordinals in its root documents
- The engine uses wildcard/glob authority references where exact canonical paths are required
- Any hot/core layer relies on a single-file contract instead of full local documentation
- The engine has violated the global dependency model
- The engine has violated the global authority order
- The engine exhibits forbidden direct ownership or shadow truth
- The engine's version marker is missing or incorrectly formatted

## Verification Process
Before declaring engine gold-freeze:
1. Verify the engine's internal completion using its own acceptance/evidence/readiness matrices
2. Verify engine acceptance matrix compliance
3. Verify engine evidence registry integrity
4. Verify engine audit readiness matrix alignment
5. Confirm engine version marker validity
6. Confirm no constitutional violations affect engine package integrity

## Emergency Unfreeze Conditions
If after gold-freeze any of the following are discovered:
- Evidence artifact referenced in acceptance matrix is deleted or becomes inactive
- Constitutional law is enacted that contradicts accepted engine truths
- Dependency model violation is discovered
- Authority order violation is discovered
Then the engine package must immediately revert to non-frozen state and enter remediation process.