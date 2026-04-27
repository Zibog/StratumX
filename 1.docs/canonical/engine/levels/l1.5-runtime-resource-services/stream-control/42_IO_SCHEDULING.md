# IO Scheduling

## Role

Async read scheduling.

## Data Model

Prioritized async read queues, batching, cancellation, completion visibility, page-locality grouping, and read-amplification control.
Seekable page products are canonical-compatible. Linux-specific `io_uring` backends are optional accelerators only.
Numeric ceilings are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical IO Rules

- runtime page target size is 128 KiB compressed with a 256 KiB hard cap;
- canceled work must be visible before decode, before staging, and before upload;
- steady-state read amplification must obey `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- seek locality and cancellation debt must be observable diagnostics.

## Queue Ceilings

| Priority class | Queue depth ceiling | Cancel-to-visible deadline |
|---|---:|---:|
| immediate | <= 256 page jobs | <= 2 frames |
| predicted | <= 512 page jobs | <= 4 frames |
| maintenance | <= 256 page jobs | <= 8 frames |
