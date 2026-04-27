# Texture Residency Streaming And Memory Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the exact bridge for texture residency, streaming, and memory.

## Required packet rows
- stable slice id;
- scope and tier tags;
- diagnostics and denial codes;
- capture and artifact pointers;
- compare eligibility markers.

## Rules
- normalization is explicit;
- compatibility is versioned;
- packets are singular and authoritative for this ordinal.
