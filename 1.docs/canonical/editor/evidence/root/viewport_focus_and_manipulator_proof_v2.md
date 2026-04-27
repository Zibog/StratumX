# Viewport Focus and Manipulator Proof v2

## Purpose

This document proves that viewport focus and manipulator systems are properly bounded and do not own lower-stack truth.

## Viewport Focus Verification

| Focus Type | Responsibility | Owns Truth | Consumes | Notes |
|------------|----------------|------------|----------|-------|
| Entity Focus | Entity selection focus | No | Tooling L6 snapshot refs | Display focus only |
| Camera Focus | Camera target focus | No | Tooling L6 snapshot refs | View focus only |
| Gizmo Focus | Manipulator focus | No | Tooling L6 snapshot refs | Interaction focus only |

## Manipulator Verification

| Manipulator | Responsibility | Owns Truth | Consumes | Notes |
|-------------|----------------|------------|----------|-------|
| Transform Gizmo | Transform manipulation | No | Tooling L6 commands | Command generation only |
| Selection Box | Selection manipulation | No | Tooling L6 commands | Command generation only |
| Measurement Tool | Measurement display | No | Tooling L6 snapshot refs | Display only |

## Authority Leakage Audit

- Viewport does not own entity truth ✓
- Viewport does not own camera truth ✓
- Manipulators generate commands, not direct mutations ✓
- Focus state is UI-only, not authoritative ✓

## Proof Basis

This proof is based on:
- Viewport component enumeration
- Manipulator command generation verification
- Focus state boundary checks
- Authority ownership verification

## Version

This is the v2 viewport focus and manipulator proof, active for editor gold closure.
