# Service and Extension Legality Proof v2

## Purpose

This document proves that editor services (L10) and extensions are properly bounded and do not bypass authority.

## Service Legality Verification

| Service | Responsibility | Owns Truth | Consumes | Notes |
|---------|----------------|------------|----------|-------|
| Import/Export | Import/export orchestration | No | Tooling L6 commands | Orchestration only |
| Graph Authoring | Graph editing UI | No | Tooling L6 commands | UI surface only |
| Script/Hot-Reload | Script runtime integration | No | Tooling L6 commands | Integration only |
| Plugin/Extension Host | Plugin hosting | No | Tooling L6 commands | Hosting only |
| Package/Market | Package management | No | Tooling L6 commands | Management only |

## Extension Legality Verification

| Extension Type | Capability | Forbidden | Notes |
|----------------|------------|-----------|-------|
| UI Extension | UI customization | Direct truth mutation | UI only |
| Command Extension | Command registration | Authority bypass | Command generation only |
| Panel Extension | Panel registration | Shadow truth stores | Display only |
| Tool Extension | Tool registration | Direct engine access | Tool UI only |

## Authority Bypass Audit

- No service bypasses L6 authority ✓
- No extension bypasses L6 authority ✓
- All services route through L6 commands ✓
- All extensions route through L6 commands ✓

## Proof Basis

This proof is based on:
- Service enumeration
- Extension capability enumeration
- Authority bypass audit
- Command routing verification

## Version

This is the v2 service and extension legality proof, active for editor gold closure.
