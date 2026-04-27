# tool_preview_results

Role class: published tooling sidecar fact set

Canonical role:
own preview result refs and status summaries returned by preview_runtime.

One data kind law:
this local pack owns only `tool_preview_results` publications and the fields declared for them.

Minimal operational meaning:
publish preview result refs and status summaries returned by preview_runtime so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
preview raster or mesh cache ownership, request scheduling truth, diagnostic events, layout refresh state.
