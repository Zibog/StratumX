# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| transaction_id | TransactionId | stable identity for one accepted mutation | unique in ledger |
| command_envelope_id | CommandEnvelopeId | source command that produced the transaction | must resolve through command_envelopes |
| before_authority_epoch | AuthorityEpoch | writer epoch before commit | must precede after_authority_epoch |
| after_authority_epoch | AuthorityEpoch | writer epoch after commit | must be monotonic |
| transaction_outcome | TransactionOutcome | commit/reject/revert result | finite enum only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `transaction_ledger` without consulting a hidden mirror.
