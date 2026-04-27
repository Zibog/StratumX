# tool_content_intents

Role class: published tooling sidecar fact set

Canonical role:
own content-authoring intents that later become commands, imports, or build jobs.

One data kind law:
this local pack owns only `tool_content_intents` publications and the fields declared for them.

Minimal operational meaning:
publish content-authoring intents that later become commands, imports, or build jobs so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
actual asset mutation truth, import/build result truth, selection ownership, package manager state.
