# Scene Certification, Benchmark, and Artifact Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This catalog freezes surfaces for scenario-bound benchmark runs, evidence artifact publication, and certification verdict retrieval.

## Surface families
- ingress/control: `CertificationScenarioRequest`, `BenchmarkArtifactCaptureRequest`, `ProfileBoundRunRequest`
- egress/observation: `ScenarioResultObservation`, `CertificationArtifactObservation`, `ThresholdComparisonObservation`, `RegressionRetentionObservation`
- refs/handles: `ScenarioRef`, `CertificationArtifactRef`, `ThresholdSetRef`
- legality/verdict: `ScenarioAvailabilityVerdict`, `ThresholdVerdict`, `ArtifactRetentionVerdict`
