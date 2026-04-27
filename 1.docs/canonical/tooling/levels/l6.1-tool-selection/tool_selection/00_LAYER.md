# tool_selection

Role class: published tooling sidecar fact set

Canonical role:
own published selection refs emitted by editor surfaces so tooling services can target the same objects without owning UI selection truth.

One data kind law:
this local pack owns only `tool_selection` publications and the fields declared for them.

Minimal operational meaning:
publish published selection refs emitted by editor surfaces so tooling services can target the same objects without owning UI selection truth so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
widget highlight state, hover-only ephemeral cursor state, object truth beyond published refs, mutation commands.
