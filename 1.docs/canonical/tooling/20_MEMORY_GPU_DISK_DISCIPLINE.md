# Memory GPU Disk Discipline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Order of protection
1. authority integrity
2. transaction and snapshot continuity
3. validation correctness
4. build/release determinism
5. editor responsiveness
6. preview richness
7. cache retention

## Memory law
- authority state stays minimal and dense;
- snapshot and index slabs are versioned and swappable;
- caches are optional and evictable;
- preview payloads are disposable;
- bulk artifact bodies live in artifact-managed storage, not in authority memory.

## GPU law
- only active preview or viewport-adjacent tooling may hold persistent GPU allocations;
- hidden preview worlds and dormant suites must release GPU-heavy state;
- bake and build coordination must not silently pin live viewport resources.

## Disk law
- imported, baked, cooked, and release outputs are manifest-backed artifacts;
- transient scratch outputs are classed as disposable and cleanup-safe;
- autosave/recovery outputs are separate from build outputs;
- asset processor and bake queues must expose disk pressure to diagnostics.

## Practical editor consequences
When the editor is under pressure, tooling must degrade in this order:
1. thumbnails and secondary previews;
2. speculative derived caches;
3. dormant suite caches;
4. background prefetch;
5. warm-but-hidden preview worlds.

It must never degrade by corrupting authority or hiding queues.
