# Budget Envelope

This contract belongs specifically to the tool activation rules level and may not be reused verbatim by another tooling level.


## Budget posture for `tool_activation_rules`
- correctness of `activation_rule_id`, `tool_kind` is non-degradable
- lower-priority outputs such as published `tool_activation_rules` rows for downstream services; bounded status or routing updates where applicable degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.12-tool-activation-rules` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
