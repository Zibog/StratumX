# Editor Threading Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the editor-wide threading model: UI thread, background jobs, cancellation, and ownership rules.

## Scope

The editor package (`L8` through `L11`) uses structured threading for:
- UI thread (shell, panels, viewports, overlays)
- Background jobs (preview, import, export, build, diagnostics, indexing)
- Cancellation and ownership rules
- Resource lifetime and cleanup

## UI Thread

### UI Thread Responsibilities
- Shell composition and layout
- Panel and viewport rendering
- Input routing and event handling
- Command dispatch
- Selection and focus management
- Invalidation and refresh coordination

### UI Thread Discipline
- All UI operations must run on the UI thread
- No blocking operations on the UI thread
- No long-running operations on the UI thread
- No direct file I/O on the UI thread
- No direct network I/O on the UI thread
- No direct compilation on the UI thread

## Background Jobs

### Preview Jobs
- Preview requests are dispatched to background threads
- Preview results are posted back to the UI thread
- Preview jobs may be cancelled
- Preview jobs may be prioritized (visible viewport > hidden viewport)

### Import/Export Jobs
- Import/export requests are dispatched to background threads
- Import/export results are posted back to the UI thread
- Import/export jobs may be cancelled
- Import/export jobs may report progress

### Build Jobs
- Build requests are dispatched to background threads
- Build results are posted back to the UI thread
- Build jobs may be cancelled
- Build jobs may report progress
- Build jobs may spawn sub-jobs (compilation, linking, packaging)

### Diagnostics Jobs
- Diagnostics requests are dispatched to background threads
- Diagnostics results are posted back to the UI thread
- Diagnostics jobs may be cancelled
- Diagnostics jobs may be incremental

### Indexing Jobs
- Indexing requests are dispatched to background threads
- Indexing results are posted back to the UI thread
- Indexing jobs may be cancelled
- Indexing jobs may be incremental
- Indexing jobs may be low-priority

### Collaboration Jobs
- Collaboration requests are dispatched to background threads
- Collaboration results are posted back to the UI thread
- Collaboration jobs may be cancelled
- Collaboration jobs may be real-time (low latency)

## Cancellation Rules

### Cancellation Structure
```rust
struct CancellationToken {
    id: JobId,
    cancelled: AtomicBool,
}
```

### Cancellation Discipline
- All background jobs must accept a cancellation token
- All background jobs must check cancellation periodically
- All background jobs must clean up resources on cancellation
- All background jobs must post cancellation status to UI thread

### Cancellation Triggers
- User cancellation (cancel button, close panel, close editor)
- Superseding request (new preview request supersedes old preview request)
- Resource pressure (low memory, low disk space)
- Timeout (long-running job exceeds timeout)

## Ownership Rules

### Job Ownership
- Jobs are owned by the surface that requested them
- Jobs are cancelled when the owning surface is closed
- Jobs are cancelled when the owning surface is deactivated (if not persistent)
- Jobs may be transferred to a new owner (e.g., background import continues after panel close)

### Resource Ownership
- Resources are owned by the surface that allocated them
- Resources are released when the owning surface is closed
- Resources are released when the owning surface is deactivated (if not persistent)
- Resources may be shared across surfaces (e.g., asset preview cache)

### Lifetime Discipline
- All jobs must have a clear owner
- All resources must have a clear owner
- All jobs must clean up resources on completion or cancellation
- All surfaces must cancel jobs and release resources on close

## Thread Safety

### Shared State
- Shared state must be protected by locks or atomics
- Shared state must be minimized
- Shared state must be documented
- Shared state must be audited for deadlocks

### Message Passing
- Prefer message passing over shared state
- Use typed message classes
- Use bounded queues to prevent unbounded memory growth
- Use backpressure to prevent queue overflow

## Forbidden Patterns

The editor must NOT:
- Block the UI thread with long-running operations
- Spawn unbounded background threads
- Leak jobs or resources
- Maintain hidden background state
- Bypass cancellation discipline

## Version

This document is part of the `SX-CANON/1.0.24/STACK-v30` editor package.
