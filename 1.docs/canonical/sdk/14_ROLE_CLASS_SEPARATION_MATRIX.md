# Role Class Separation Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

| Role | Allowed class | Forbidden confusion |
|---|---|---|
| ingress packet | mutation envelope | control, ref |
| ingress control | binding or execution signal | packet, verdict |
| observation egress | observation record | metric frame |
| metric egress | metric frame | observation record |
| compatibility fact | immutable fact | verdict |
| compatibility verdict | derived decision | fact |
| legality gate | derived legality decision | transport policy |
| opaque handle | live token | ref |
| opaque ref | read projection token | handle |
| opaque artifact ref | generated product token | state ref |
