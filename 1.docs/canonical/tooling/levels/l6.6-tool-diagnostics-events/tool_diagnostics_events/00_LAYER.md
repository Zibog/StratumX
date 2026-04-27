# tool_diagnostics_events

Role class: published tooling sidecar fact set

Canonical role:
own bounded diagnostics events emitted by validation, build, release, preview, and runtime services.

One data kind law:
this local pack owns only `tool_diagnostics_events` publications and the fields declared for them.

Minimal operational meaning:
publish bounded diagnostics events emitted by validation, build, release, preview, and runtime services so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
grouped issue views, UI filter state, mute/acknowledge policy state, authority mutation truth.
