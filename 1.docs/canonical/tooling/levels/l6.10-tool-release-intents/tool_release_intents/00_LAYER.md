# tool_release_intents

Role class: published tooling sidecar fact set

Canonical role:
own release-facing intents such as build channel selection, packaging class, or publish requests.

One data kind law:
this local pack owns only `tool_release_intents` publications and the fields declared for them.

Minimal operational meaning:
publish release-facing intents such as build channel selection, packaging class, or publish requests so downstream tooling can route work without stealing editor ownership.

Forbidden drift:
actual release artifacts, signing secrets, distribution credentials, package manager installation truth.
