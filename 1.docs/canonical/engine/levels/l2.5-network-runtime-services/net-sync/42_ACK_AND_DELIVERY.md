# Ack and Delivery

## Role

Delivery feedback and backlog law.

## Data Model

Ack windows are bounded, backlog is observable, and delivery state may not grow unbounded per connection.
Numeric values are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Ceilings

| Item | Ceiling |
|---|---:|
| ack window | <= 256 sequence ids or ranges |
| outstanding reliable control packets | <= 64 |
| outstanding reliable bulk packets | <= 128 |
| resend horizon for control lane | <= 500 ms |
| resend horizon for bulk lane | <= 1,500 ms |
| ack-state bytes per peer | <= 32 KiB |
| resend-buffer bytes per peer | <= 256 KiB |
| delivery-state bytes per peer | <= 384 KiB on `interactive-60`; <= 512 KiB on `listen-host-60` and `headless-20` |

Unreliable lanes carry no reliable resend debt.
