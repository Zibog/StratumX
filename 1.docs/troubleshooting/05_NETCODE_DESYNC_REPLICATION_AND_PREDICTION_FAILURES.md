# Netcode Desync, Replication, and Prediction Failures

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **troubleshooting**.


| Symptom | Recovery |
|---|---|
| Entity missing on client | inspect interest reason and replication family. |
| Rubber-banding | inspect prediction reconcile packet. |
| Desync detected | capture desync trace and open replay. |
| Bandwidth spike | inspect replication priority and rate. |
| Authority mismatch | open authority explanation route. |
