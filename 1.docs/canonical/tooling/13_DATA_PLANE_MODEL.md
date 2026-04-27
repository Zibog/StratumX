# Data Plane Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical `L6` planes
- authority plane
- command plane
- transaction plane
- snapshot plane
- index plane
- derived plane
- artifact plane
- stream plane
- cache plane
- budget plane
- workspace plane
- validation plane
- preview plane
- build plane
- release plane

## Canonical `L6A` planes
- session plane
- evidence plane
- proposal plane
- lowering plane
- safety plane
- apply/revert plane
- assistant-ui plane
- model-request plane

## Canonical `L7` planes
- campaign plane
- governance plane
- automation plane
- reporting plane

## Canonical `L7A` planes
- goal plane
- plan plane
- canon-reasoning plane
- generation plane
- optimization plane
- migration plane
- routing plane

## Plane law
- clients live around planes;
- equal data classes must co-reside when they share locality, lifetime, and access pattern;
- families and tools do not own secret stores when a shared plane exists;
- every plane must declare storage form, mutability, rebuild policy, invalidation policy, and eviction policy when applicable.

## L6 plane responsibilities for the editor dream
- **authority plane**: minimal mutable authoring truth such as entity hierarchy edges, component attachments, prefab-instance bindings, layer memberships, streaming-policy assignments, and stable authoring ids;
- **snapshot plane**: immutable frame-safe authoring projections;
- **index plane**: hierarchy, search, dependency, reverse-reference, region/cell, data-layer, package, and validation lookup indices;
- **derived plane**: discardable panel/view projections, thumbnails, diffs, summaries, and staged comparison products;
- **artifact plane**: deterministic imported, baked, cooked, built, and packaged outputs and manifests;
- **stream plane**: diagnostics, traces, playtest, runtime-bridge, and queue event streams;
- **workspace plane**: session ids, activation scopes, attachment bindings, public routing refs, and cursor-safe runtime coordination; never product UI state;
- **validation plane**: rule registries, queued scans, verdict sets, remediation refs;
- **preview plane**: speculative previews, runtime attach previews, shot/camera previews;
- **build plane**: asset-processor queues, reimport queues, bake graphs, build graphs, source-change invalidation records, filesystem watch state, dependency tracking, and remediation state;
- **release plane**: packaging presets, output graphs, manifest closure, and publication metadata.
