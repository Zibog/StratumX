# Threading Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Single-writer zones
- `L6 authority_core`
- `L6 transaction commit`
- `L6 undo/revert ownership`
- `L6 workspace session/layout/focus ownership`
- `L6A active assistant session ownership`

## Parallel-friendly zones
- snapshot builders
- index rebuilds
- artifact builders
- derived projection jobs
- validation scans
- preview jobs
- cache builders
- stream consumers
- evidence pack construction
- proposal preparation
- campaign compilation
- plan construction, optimization, and migration reasoning

## Forbidden
- no multi-writer editor truth
- no hidden cross-thread mutation around the transaction ledger
- no preview or build writes around `L6` authority
- no assistant apply path around `L6` command and transaction ownership
