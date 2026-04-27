# Audio Authoring and Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document maps audio asks into tooling runtimes and legal lower-stack routes.

## Canonical route
`editor audio ask -> tooling audio authoring intent -> preview/transaction/artifact classification -> sdk bridge only when lower truth must change -> engine audio runtime or observation -> editor projections and diagnostics`

## Audio task classes
| Task class | Tooling result class |
|---|---|
| import or index audio asset | content transaction or artifact result |
| assign emitter/listener/zone parameters | authoring transaction |
| audition sound or bus mix | preview-only result |
| inspect occlusion, virtualization, or voice priority | diagnostics-only result |
| prepare packaged audio content | artifact result |

## Preview law
An audition or preview mix may be live and useful without becoming authoritative runtime truth.
Tooling must mark that distinction explicitly.

## Current posture
Audio routing is canonically explicit after this wave, but broad engine-backed proof remains future work.
