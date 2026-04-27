# Budget, Degradation, Pressure, and Recovery Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose this technology family to the operator without stealing runtime truth from engine.

## Command inventory
- `inspect.resource.vector`
- `reveal.resource.threshold`
- `compare.resource.recovery`
- `certify.resource.floor`

## Overlay families
- `overlay.pressure.cpu`
- `overlay.pressure.gpu`
- `overlay.pressure.ram`
- `overlay.pressure.io`

## Inspector fields
- `resource_vector`
- `threshold_code`
- `degrade_step`
- `recovery_trigger`

## Disabled reasons
- `compare.disabled.no_spike`
- `certify.disabled.bundle_missing`

## Compare and capture law
This lab may use only the compare/capture modes declared in root `54` and the packet families declared by sdk `64–74`.
Any capture or compare action must lower through tooling and return a retained artifact or a denial code.

## Focus rules
- success -> 103/109
- failure -> 100
- pressure -> self

## Evidence duties
If this lab participates in certification, it must be able to point to one retained artifact, one terminal result code, and one next legal action when a failure occurs.

## Current posture
`document_gold / doc_closed_impl_open`
