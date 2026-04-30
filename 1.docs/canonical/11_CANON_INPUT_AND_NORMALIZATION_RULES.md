# Canon Input and Normalization Rules

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This document freezes how the canonical archive must be consumed by humans, automation, and neural implementation systems.
The goal is to prevent split-brain ingestion caused by stale file names, duplicated families, package-boundary contamination, or stale evidence packs.

## Canon ingestion law
The canonical archive must be read through indexes and explicitly named family documents.
No consumer may treat every markdown file inside the archive as equally authoritative.
Authority flows through:
1. root `00_INDEX.md`;
2. package `00_INDEX.md`;
3. package authority-order documents;
4. package acceptance/evidence/readiness documents;
5. feature/family canons explicitly referenced by package indexes.

## Product-input allowlist for the first product-result loop
The authoritative minimum set for the first product-result loop is:
- `canonical/00_INDEX.md`
- `canonical/10_FIRST_PRODUCT_RESULT_CANON.md`
- `canonical/11_CANON_INPUT_AND_NORMALIZATION_RULES.md`
- `canonical/24_PROJECT_TO_PLAYABLE_PRODUCT_OPERATOR_PLAYBOOK_CANON.md`
- `canonical/34_PRODUCT_ARTIFACT_AND_EXECUTABLE_IDENTITY_CANON.md`
- `canonical/35_BUILD_EXPORT_AND_LAUNCH_DIAGNOSTIC_CANON.md`
- `canonical/39_FIRST_PRODUCT_RELAY_EVIDENCE_PROTOCOL_CANON.md`
- `canonical/62_FIRST_LAYER_TO_FRAME_WORKLOAD_ATLAS_CANON.md`
- `canonical/65_HEAVY_DOMAIN_RESOURCE_ENVELOPE_AND_CERTIFICATION_PACK_CANON.md`
- `canonical/editor/00_INDEX.md`
- `canonical/editor/39_ACCEPTANCE_MATRIX.md`
- `canonical/editor/40_EVIDENCE_REGISTRY.md`
- `canonical/editor/99_AUDIT_READINESS_MATRIX.md`
- `canonical/editor/65–81` heavy-domain labs
- `canonical/editor/89–109` technology-wave labs

## Explicit exclude set
The following classes are non-authoritative and intentionally excluded from this gold bundle:
- any historical or archival subtree;
- any patch manifest, patch delete list, or remediation-plan file;
- any retired engine-root editor/product spillover document;
- any world-named obsolete alias formerly used for demo-specific paths;
- any repo code evidence path that is not canonically promoted.

## Package-boundary law
`engine/` must not serve as a spillover directory for editor/product documents.
`editor/` must not pretend to own lower-stack truth.
`sdk/` must publish exact typed surfaces, not hidden runtime truth.
`tooling/` must publish exact execution routes, not alternate engine ownership.

## One-host law for product ingestion
Neural implementation systems must assume exactly one canonical editor host.
Any second host path, preview shell, deprecated shell, or parallel app path may only be used as migration evidence outside this gold bundle, never as equal product truth.

## Practical ingestion rule
When feeding the archive into a neural implementation system:
- include the root reading order above;
- trust current package indexes and readiness docs;
- exclude any reference to retired ordinals if a current active document replaces the role;
- treat implementation reality as evidence, but treat canonical files as target law.
