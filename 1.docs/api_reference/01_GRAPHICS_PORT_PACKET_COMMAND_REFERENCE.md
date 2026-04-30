# Graphics Port Packet and Command Reference

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **API reference**.


| Name | Kind | Purpose |
|---|---|---|
| `graphics.backend.capability.v1` | packet | Backend availability, tier, shader target, limits. |
| `graphics.backend.policy.resolve.v1` | command | Resolve backend policy. |
| `graphics.frame.plan.v1` | packet | Frame plan identity, passes, resources, capture intent. |
| `graphics.present.result.v1` | packet | Present outcome and failure code. |
| `graphics.black_frame.trace.v1` | packet | Black-frame reason and recovery. |
| `route.graphics.doctor` | route | Probe backend and explain state. |
