# STRATUMX_L6_AUTHORITY_AND_TRANSACTION_CONSTITUTION

## Scope
This constitution fixes how authority and transactions behave in tooling.

## Binding laws
- authority rows are explicit and minimal;
- mutating work passes through declared transaction envelopes or command/result paths;
- derived or cached products never masquerade as source authority.

## Audit checks
- authority, transaction, snapshot, and derived planes remain distinct;
- transaction provenance is auditable;
- no service mutates truth behind view/state projections.
