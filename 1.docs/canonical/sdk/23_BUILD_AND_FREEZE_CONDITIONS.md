# Build And Freeze Conditions

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

L5 local document gold is frozen only when:
- all bridge classes are typed and bounded
- artifact refs are distinct from state refs
- verdicts are distinct from facts
- controls are distinct from packets
- no direct upper-layer truth appears inside L5
- no hidden cache, graph, or disk store appears inside L5
- no hot-path publication requires per-envelope heap allocation by default
- ingress publication, snapshot publication, and egress batch publication are distinct physical planes
- legality and compatibility hot paths are resolved through declared compiled tables rather than ad-hoc mutable logic
- every declared level, including `l5.15-engine-artifact-refs`, has complete local layer contracts
- `27_ACCEPTANCE_MATRIX.md` has no `fail` row and any historical-local freshness is declared honestly
- `30_EVIDENCE_REGISTRY.md` has no missing evidence rows for the active SDK-local contour
- `99_AUDIT_READINESS_MATRIX.md` has no unresolved blocker that contradicts local document gold
- the implementation-facing bridge target matrix is explicit enough to become CI and bridge-integration work without reinterpretation
- active test-result artifact exists with executed documentation-package evidence

Umbrella production-gold requires more: rebasing the current v13 evidence contour to the active umbrella stack marker.

## Anti-template rule
A local contract pack is not complete if two different L5 levels can swap their local `levels/*/20_DEPENDENCIES.md` files, local `levels/*/30_COMMUNICATION.md` files, local `levels/*/32_BOUNDARY_PRESERVATION.md` files, or local `levels/*/40_FIELDS.md` files without changing meaning. Domain-specific field classes, legal edges, and forbidden ownership must remain level-specific.
