# Netcode Packet and Command Reference

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **API reference**.

---

# V34 netcode packet closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Required packet families

| Packet | Meaning |
|---|---|
| `net.authority_mode.v1` | local, listen_server, dedicated_server, replay |
| `net.interest_scope.v1` | cell/region/actor interest window |
| `net.intent_command.v1` | client/user intent |
| `net.state_delta.v1` | authoritative state delta |
| `net.prediction_sample.v1` | local prediction data |
| `net.reconciliation_result.v1` | server correction result |
| `net.retained_summary.v1` | far/degraded world consequence summary |
| `net.desync_report.v1` | mismatch, source, recovery route |
