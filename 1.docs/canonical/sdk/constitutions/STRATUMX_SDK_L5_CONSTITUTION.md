# STRATUMX_SDK_L5_CONSTITUTION

## Scope
This is the top-level constitution for the `sdk/L5` bridge package.

## Canonical mission
`L5` is the thin typed bridge between engine `L4` and the upper tooling/editor stack.
It exists to carry bridge facts, not to become an editor runtime or orchestration layer.

## Binding laws
- `L5` owns only packets, observations, metrics, verdicts, refs, handles, cursors, epochs, and artifact refs declared by the package root;
- `L5` must stay authority-thin, replay-safe, and locality-aware;
- any proposed `L5` addition must justify why it belongs below tooling/editor rather than above.

## Audit checks
- bridge exports remain typed and semantically narrow;
- no editor/tooling/product ownership leaks into the bridge;
- evidence and readiness remain aligned with the declared `L5` mission.
