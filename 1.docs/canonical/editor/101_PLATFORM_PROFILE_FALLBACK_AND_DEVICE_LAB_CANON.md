# Platform Profile, Fallback, and Device Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose this technology family to the operator without stealing runtime truth from engine.

## Command inventory
- `inspect.platform.profile`
- `inspect.platform.device_target`
- `compare.platform.fallback`
- `certify.platform.floor`

## Overlay families
- `overlay.platform.profile`
- `overlay.platform.fallback_state`

## Inspector fields
- `platform_profile`
- `device_target`
- `fallback_profile`
- `platform_result_code`

## Disabled reasons
- `inspect.disabled.no_device`
- `certify.disabled.profile_missing`

## Compare and capture law
This lab may use only the compare/capture modes declared in root `54` and the packet families declared by sdk `64–74`.
Any capture or compare action must lower through tooling and return a retained artifact or a denial code.

## Focus rules
- success -> 102
- failure -> 100
- pressure -> 103

## Evidence duties
If this lab participates in certification, it must be able to point to one retained artifact, one terminal result code, and one next legal action when a failure occurs.

## Current posture
`document_gold / doc_closed_impl_open`
