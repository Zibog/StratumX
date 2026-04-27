# tool_panel_refs

Role class: published tooling sidecar fact set

Canonical role:
own panel refs visible to tooling services while keeping actual layout and widget ownership in editor.

One data kind law:
this local pack owns only `tool_panel_refs` publications and the fields declared for them.

Minimal operational meaning:
publish panel refs visible to tooling services while keeping actual layout and widget ownership in editor so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
actual layout topology, widget hierarchy, panel-local transient UI state, render timing truth.
