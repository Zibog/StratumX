# Tool Context and Mode Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Context classes
- global editor context
- viewport interaction context
- suite-local domain context
- graph context
- review context
- build/ops context
- runtime-attach context

## Mode classes
- persistent shell mode
- transient modal tool mode
- suite mode
- preview mode
- play/simulate/debug mode
- runtime-inspector attach mode

## Required tool groups
- transform tools
- placement tools
- world tools
- prefab tools
- inspection/debug tools
- timeline/camera tools

## Laws
- only one focused manipulator context may own high-priority pointer routing at a time
- suite modes must be activation-bounded
- mode switches may invalidate overlays, inspectors, and contextual panels but may not mutate truth implicitly
