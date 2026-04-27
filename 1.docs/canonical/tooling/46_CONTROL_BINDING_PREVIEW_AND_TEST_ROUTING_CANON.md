# Control Binding, Preview, and Test Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document maps control-binding and input-test asks into tooling-owned flows.

## Canonical route
`editor binding or control-test ask -> tooling control intent -> conflict/legality scan -> preview or transaction classification -> sdk control surfaces -> engine/runtime consequence when required -> diagnostics and editor projections`

## Control task classes
| Task class | Tooling result class |
|---|---|
| add or change a binding | authoring transaction or denial |
| switch an input context | preview or runtime consequence request |
| test an action map without full gameplay possession | preview-only result |
| inspect focus or conflict diagnostics | diagnostics-only result |
| apply binding pack to active product profile | authoring transaction or artifact result |

## Legality law
A control change must be able to explain:
- whether the change is syntactically valid;
- whether it conflicts with active bindings;
- which context it affects;
- whether it is preview-only or durable.

## Current posture
This document closes routing for control asks even where the uploaded code does not yet prove the full runtime breadth.
