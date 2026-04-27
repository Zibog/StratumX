# STRATUMX_CANONICAL_SHAPES

## 1. Purpose

This document defines canonical shape classes used by configuration, persistence, startup, runtime, testing, performance, API contract law, and implementation-facing public contract derivation.

It defines shape classes, not crate boundary forms.

## 2. Generic Shape Classes

### 2.1. Config
Canonical configuration shape supplied to one owner, profile, family, service, or pipeline boundary.

### 2.2. Descriptor
Canonical immutable descriptive shape for identity, material, layout, membership, region, or surface metadata.

### 2.3. Snapshot
Canonical immutable authoritative-state capture shape at one execution boundary.

### 2.4. ReadView
Canonical localized immutable read shape derived from one legal snapshot or authoritative state owner.

### 2.5. Context
Canonical execution-facing input bundle for one family, service, or pipeline step.

### 2.6. Delta
Canonical staged change shape emitted before authoritative apply.

### 2.7. Result
Canonical bounded output shape emitted after one legal step, request, validation, or apply operation.

### 2.8. Metrics
Canonical bounded telemetry shape emitted by one legal execution, service, synthesis, or pipeline step.

### 2.9. ContractSet
Canonical grouped public contract surface for one backbone or law-oriented crate.


### 2.10. TypeSet
Canonical grouped foundational or law-oriented type surface that exposes a bounded base-type vocabulary.

### 2.11. ErrorSet
Canonical grouped error-model surface that exposes bounded typed failures.

### 2.12. Token
Canonical stable identity or handle token used as a compact public reference type.

### 2.13. Query
Canonical query-definition or traversal-definition shape.

### 2.14. Window
Canonical bounded legal access or mutation window shape.

### 2.15. Batch
Canonical grouped staged-output, mutation, or apply payload shape.

### 2.16. Request
Canonical bounded service or pipeline input shape.

### 2.17. Plan
Canonical bounded launch or orchestration plan shape.

## 3. Primitive Field Classes

### 3.1. ProofMarker
Opaque proof token emitted only by a legal validation path.

### 3.2. DecisionReference
Stable reference to a canonical decision object.

### 3.3. ResultClass
Canonical bounded result enum describing legal, rejected, degraded, or blocked outcome.

### 3.4. CompatibilityClass
Canonical bounded enum describing same-canon, additive-compatible, migration-required, or illegal compatibility state.

### 3.5. VersionMarker
Canonical stack version value owned only by `STACK_VERSION`.

## 4. Canonical Enablement Shapes

### 4.1. EnablementConfiguration
Canonical `Config` shape that selects optional legal units while preserving all always-on units.

Canonical fields:
- runtime profile target;
- optional service-layer enablement flags;
- optional content pipeline enablement flag;
- diagnostics mode;
- persistence mode.

### 4.2. LegalEnablementSet
Canonical validated `Result` shape of enablement configuration after legality checks.

Canonical fields:
- always-on units set;
- enabled optional units set;
- disabled optional units set;
- rejected optional units set;
- legality proof marker: `ProofMarker`.

## 5. Startup Shapes

### 5.1. StartupReadyAssemblyDecisionSet
Canonical `Result` shape produced before runtime launch.

Canonical fields:
- selected runtime profile;
- legal enablement set;
- diagnostics mode;
- persistence mode;
- restoration selectors when restoring;
- canonical stack version marker: `VersionMarker`;
- legal assembly decision reference: `DecisionReference`.

### 5.2. LegalAssemblyDecision
Canonical `Result` shape proving that one assembly is legal under execution, configuration, enablement, and compatibility law.

Canonical fields:
- world instance target;
- runtime profile;
- enablement proof: `ProofMarker`;
- compatibility proof: `ProofMarker`;
- startup legality result: `ResultClass`.

### 5.3. LegalRestorationDecision
Canonical `Result` shape proving that one persistence payload may restore into one legal assembly.

Canonical fields:
- payload identity;
- compatibility class: `CompatibilityClass`;
- restoration selectors;
- runtime profile target;
- restoration legality result: `ResultClass`.

## 6. Restoration Shapes

### 6.1. ProfileSafeRestorationSelectors
Canonical `Descriptor` shape required to restore state under a legal runtime profile without restoring transient runtime internals.

Canonical fields:
- runtime profile selector;
- world snapshot selector;
- required family markers selector;
- compatibility flags selector;
- restoration scope selector.

### 6.2. RuntimeProfileSafeRestorationState
Canonical `Snapshot` shape legal under a selected runtime profile.

Canonical fields:
- authoritative world state payload;
- material substrate payload when persisted;
- legal enablement set;
- profile-safe restoration selectors;
- canonical stack version marker: `VersionMarker`;
- compatibility metadata.

## 7. Persistence Metadata Shapes

### 7.1. RequiredFamilyMarkers
Canonical `Descriptor` shape carried by persistence payloads.

Canonical fields:
- runtime family marker: `VersionMarker`-compatible family id;
- critical simulation family markers set;
- service-layer markers set when payload depends on them.

### 7.2. CompatibilityFlags
Canonical `Descriptor` shape that describes migration, strictness, and restoration legality conditions.

Canonical fields:
- strict same-canon required;
- migration allowed;
- service-layer outputs ignored on restore;
- profile lock required;
- version forward-compatibility rejected.

## 8. Canonical Rule

Shape classes defined here are referenced by:
- API contract law;
- configuration law;
- persistence law;
- startup law;
- testing law.

They are not redefined elsewhere.
