# Document Rules

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Every document in this package must preserve:
- L5 has no hidden truth store
- bridge classes stay typed and bounded
- refs never become mutation backdoors
- verdicts never masquerade as facts
- artifact refs never masquerade as state refs
- no whole-project dumps through bridge surfaces
- no silent growth in memory, disk, or GPU ownership
