# Task Routing and Ownership Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This document maps asks to legal routes across the stack.

## Canonical route grammar
`operator ask -> editor command -> tooling intent/transaction -> sdk packet/observation -> engine truth consequence -> sdk observation -> tooling publication -> editor surface`

## Representative asks
| Ask | Primary owner | Route summary | Forbidden shortcut |
|---|---|---|---|
| Create a new project | tooling workspace route | editor.project.new -> tooling/60 -> sdk/58 -> workspace identity | editor may not fabricate project identity locally |
| Build desktop executable | tooling build route | editor.build.desktop -> tooling/61 -> sdk/61 -> engine/runtime packaging | editor may not call an emitted artifact built if no artifact exists |
| Why did the frame fall below budget? | engine tech diagnostics | editor.tech.budget.inspect -> tooling/71 -> sdk/70 -> engine/98,97 | editor may not invent a reason chain |
| Why is this sound occluded? | engine audio truth | editor.tech.audio.trace -> tooling/68 -> sdk/66,69 -> engine/92,97 | tooling may not rewrite runtime occlusion decisions as authored truth |
| Which platform fallback fired? | engine platform truth | editor.tech.platform.inspect -> tooling/69 -> sdk/68 -> engine/96 | sdk may not hide fallback class or downgrade reason |
| What placeholder blocks this route? | root/engine placeholder policy | editor.tech.stub.audit -> tooling/73 -> sdk/76 -> root/45,51 and engine/99 | no layer may present placeholder output as production result |
| Guide me from empty project to playable build | root operator playbook | editor.playbook.start -> tooling/75,75,76 -> sdk/73,74 -> engine truth and product relay | no step may be skipped silently |


## Ownership law
Ownership follows the route, not the UI.
The editor owns visibility and operator intent.
Tooling owns transaction execution.
SDK owns typed transport and compatibility.
Engine owns runtime truth.
Root owns doctrine and certification policy.
