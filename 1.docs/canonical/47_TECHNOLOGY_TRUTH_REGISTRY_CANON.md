# Technology Truth Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

| Truth class | Owning package | Canonical object families | Non-owners may |
|---|---|---|---|
| frame extraction truth | engine | `ViewsetState`, `PassGraphState`, `PresentationState` | inspect, capture, compare |
| material archetype and registry truth | engine | `MaterialArchetypeState`, `SurfaceFamilyState`, `ResponseProfileState`, `BlendPolicyState` | inspect, bind, compare |
| material instance and overlay truth | engine | `MaterialInstanceStackState`, `TerrainLayerBlendState`, `BiomeOverlayState`, `AftermathOverlayState` | inspect, capture, recover |
| material consequence tier truth | engine | `ConsequenceTierState`, `SleepWakeState`, `LocalConsequenceState`, `DownstreamConsequenceState` | inspect, compare, capture |
| material and residency truth | engine | `MaterialBindingState`, `ResidencyState`, `SamplerPolicyState` | observe and diagnose |
| audio route truth | engine | `AudioRouteState`, `MixState`, `DevicePresentationState` | audition, inspect, compare |
| diagnostics truth | engine | `TraceNodeState`, `FailureState`, `ReasonChainState` | render drilldown surfaces |
| budget truth | engine | `PressureState`, `DegradeState`, `RecoveryState` | show overlays and next actions |
| packet compatibility truth | sdk | `SchemaRevision`, `CompatibilityWindow`, `NormalizationRule` | consume |
| transaction/artifact truth | tooling | `TransactionState`, `ArtifactRecord`, `RetentionState` | review |
| operator focus truth | editor | `FocusTarget`, `LabViewState`, `SelectedArtifactState` | request only |


## Material-first closure note
The active contour explicitly owns the following truth classes as first-class runtime truth:
- `MaterialArchetypeState`;
- `SurfaceFamilyState`;
- `MaterialInstanceStackState`;
- `ResponseProfileState`;
- `TerrainLayerBlendState`;
- `BiomeOverlayState`;
- `AftermathOverlayState`;
- `ConsequenceTierState`;
- `SleepWakePolicyState`.

## Material-first ownership law
- geometry does not own consequence law;
- a mesh or model may point at a material instance stack but may not redefine archetype behavior ad hoc;
- surface family, response profile, and consequence tier are declared truth classes, not inferred view hacks;
- aftermath overlays are world/material truth, not renderer-only decals.

## Forbidden ownership drift
- editor may not own runtime truth;
- tooling may not own pass graph truth;
- sdk may not define runtime behavior;
- validation workloads may not become truth owners;
- model import may not smuggle material behavior that bypasses archetype and response registries.

## Material-centric truth additions
The active contour now treats the following truth classes as first-class and mandatory when material law is the owner:
- `MaterialResponseFamilyState`
- `MaterialLightResponseState`
- `MaterialVisualResponseFamilyState`
- `MaterialAcousticResponseState`
- `MaterialCheapRuntimePostureState`
- `MaterialRouteClosureState`
- `MaterialCentricOperatorSurfaceState`
