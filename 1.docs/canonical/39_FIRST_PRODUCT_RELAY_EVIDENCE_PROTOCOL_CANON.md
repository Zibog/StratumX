# First Product Relay Evidence Protocol Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define what evidence is required before the proving rail may claim stronger closure.

## Mandatory relay bundle
A lawful first-product or proof-region relay bundle contains:
- workspace bootstrap ref;
- canonical content snapshot ref;
- runtime binding graph ref;
- proof-region recipe ref;
- baseline run ref;
- failed-run ref when any blocker occurred;
- recovery-run ref when recovery was exercised;
- compare digest ref;
- evidence append ref;
- certification verdict ref;
- freeze review ref;
- executable artifact ref;
- launch trace ref;
- first-result verification ref;
- old-floor result ref when the active pack is freeze-relevant on old hardware.

## Append rules
- append is illegal if the bundle hides baseline, failed-run, or recovery-run identity;
- append is illegal if first failure code or next legal recovery action is missing;
- append is illegal if the proof-region recipe ref is absent from a claimed phase-5 relay bundle.

## Promotion gates
A relay bundle may promote phase-5 posture only when:
- the same proof-region is authorable from editor-only routes;
- the bundle includes compare, capture, recover, certify, freeze, build, export, launch, and first-result artifacts;
- mixed-pack continuity is explicit rather than inferred;
- no hidden manual code path is required to make the product launch.

## Invalid evidence
The following do not count as relay evidence:
- screenshots without route and packet lineage;
- launch success without first-result verification;
- one green run without retained baseline and recovery anchors;
- a build artifact whose proof-region recipe cannot be reconstructed.

## Audit law
Phase-5 closure is invalid if any relay claim omits proof-region recipe, retained baseline, first-result verification, or old-floor posture when required by the reviewed packs.
