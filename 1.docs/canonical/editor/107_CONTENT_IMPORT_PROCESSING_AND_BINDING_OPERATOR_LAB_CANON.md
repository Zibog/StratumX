# Content Import Processing And Binding Operator Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the production contour for import, canonicalization, runtime binding, and proof-region staging.
This lab may not stop at raw asset intake; it must produce stageable runtime-ready payloads.

## Required production surfaces
- source ingest;
- canonicalization profile;
- runtime binding graph;
- identity drift board;
- proof-region staging board;
- unresolved dependency board.

## Phase-5 obligations
- every imported asset that participates in the brutal proof-region must publish a placement or binding backlink into the region recipe;
- staging must expose what is still missing before the proof-region can be played;
- no staged payload may bypass canonical ids, runtime binding refs, or artifact lineage.

## Required outputs
- canonical asset ref;
- binding graph ref;
- staged proof-region payload ref;
- unresolved blocker digest;
- evidence backlink into `editor/105` when staging is freeze-relevant.

## Day-zero import additions
- this lab owns canonical intake for `btn.import.heightmap_source`;
- imported heightmaps must publish `source_lineage_ref`, target `world_ref`, and the canonical terrain import payload consumed by editor `108`;
- import may not bypass archetype, surface-family, or blend-policy binding when terrain becomes world truth.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
