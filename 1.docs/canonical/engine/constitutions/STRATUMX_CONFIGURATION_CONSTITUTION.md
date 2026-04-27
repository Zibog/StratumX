# STRATUMX_CONFIGURATION_CONSTITUTION

## 1. Purpose

This document defines the canonical configuration law of StratumX.

## 2. Configuration Principles

- configuration selects legal profiles and legal units; it does not redefine architecture;
- configuration tunes runtime behavior within execution law; it does not replace execution law;
- startup owns final assembly decisions;
- configuration may disable only units declared legally optional in the enablement legality matrix.

## 3. Configuration Classes

### 3.1. Engine configuration
Canonical engine-level configuration shared across execution and system layers.

### 3.2. Runtime profile configuration
Configuration that selects `engine_runtime_headless` or `engine_runtime_realtime` under the runtime family.

### 3.3. Network role configuration
Configuration that selects offline, client, listen-host, or dedicated-server role variants where legal.

### 3.4. Enablement configuration
Configuration that selects a legal enablement set from the enablement legality matrix.

### 3.5. Diagnostics configuration
Configuration that controls tracing and observability detail.

### 3.6. Persistence configuration
Configuration that controls snapshot and persistence policy.

## 4. Canonical Enablement Terms

Canonical shapes are defined by:
- `STRATUMX_CANONICAL_SHAPES.md`

### 4.1. Legal units
The canonical enablement scope is the full set of legal units defined by `STRATUMX_ENABLEMENT_LEGALITY_MATRIX.md`.

### 4.2. Enablement configuration
The `EnablementConfiguration` object selects enabled optional legal units while preserving all always-on legal units.

### 4.3. Legal enablement set
The `LegalEnablementSet` object is the concrete validated enablement result produced from configuration after legality validation.

## 5. Startup-Ready Assembly Decisions

### 5.1. Definition
`StartupReadyAssemblyDecisionSet` is the final canonical configuration product produced before runtime launch.

### 5.2. Owner
`engine_startup` owns startup-ready assembly decision sets.

### 5.3. Canonical data contract
A startup-ready assembly decision set contains:
- selected runtime profile;
- selected network role where enabled;
- legal enablement set;
- diagnostics mode;
- persistence mode;
- required restoration selectors when restoring;
- canonical stack version marker;
- legal assembly decision reference.

## 6. Always-On Rule

Always-on units are listed in:
- `STRATUMX_ENABLEMENT_LEGALITY_MATRIX.md`

They are mandatory engine units, not only families. `engine_startup` is assembly-time mandatory and may not be disabled.

## 7. Canonical Laws

- no illegal unit enablement;
- no illegal unit disablement;
- no illegal network role/profile combination;
- no configuration path may create a second runtime authority for one world instance;
- no configuration may bypass canonical stack version compatibility checks.
