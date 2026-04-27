# STRATUMX_NUMERIC_SOURCE_DIGEST_SET

## Purpose

This document resolves validator source digest set `NUMSRC-R13-02` for the current package stack marker.
It exposes the exact file-level SHA-256 digests for the deterministic scanned manifest frozen by `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md`.

## Digest Set

Digest set id: `NUMSRC-R13-02`
Applies to stack marker: `SX-CANON/1.0.24/STACK-v30`
Aggregate digest: `sha256:9c0b75eb32355ad01aeb45992cfd0bca00c6e31cb479e81cf677416aa0e913ab`
Aggregate digest derivation rule: Sort file-digest rows by canonical path ascending (lexicographic UTF-8). For each row, concatenate `canonical path` + newline + `sha256:` + digest hex (without colons) + newline. Concatenate all rows in sorted order into a single byte sequence. Compute SHA-256 over this concatenated sequence. The resulting 32-byte hash, encoded as lowercase hex with `sha256:` prefix, must equal the registered aggregate digest `sha256:9c0b75eb32355ad01aeb45992cfd0bca00c6e31cb479e81cf677416aa0e913ab`.

## File Digests

| # | Canonical path | SHA-256 |
|---:|---|---|
| 1 | `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md` | `sha256:36afac45a583da35d01849b92ec1211fda375f91cff9630d0fec282f2bea8ff4` |
| 2 | `constitutions/STRATUMX_BENCHMARK_PROTOCOL.md` | `sha256:216ac79b2afff1e9b73bf4e456de0e5d51d730857592caf4adb4ad9de0735374` |
| 3 | `constitutions/STRATUMX_CRATE_PERFORMANCE_BUDGETS.md` | `sha256:b5760ecea43600793111f0fec868672530c08179fb76703aa223534302e6340a` |
| 4 | `constitutions/STRATUMX_HARDWARE_PROFILE_CONTRACT.md` | `sha256:dc5761c6a9d2b63f5c2db1c9660d31126f8c11377948a63511363bd957787c5d` |
| 5 | `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md` | `sha256:b43d49b75e63f18f3aa0cc2ab4188cbb847af06b6ed6af1cd235e54bf7d34648` |
| 6 | `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` | `sha256:13ee7f230e8be1526c9bc5066e2077202b087914de7920c41aae28d852dd49e2` |
| 7 | `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` | `sha256:90584bcb93944df42655864768b5780a856d82b0bbb01c7fadd6bc1c6a329df2` |
| 8 | `constitutions/STRATUMX_NUMERIC_VALIDATION_RULE.md` | `sha256:cec7c18abb34c5e5479217f90bbc2d697011b9539d5aa5c47b95861ce42b7f39` |
| 9 | `constitutions/STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` | `sha256:03a9d69f1c9826f4ee063407dff228d2ad1184abc552e432db15bb5d78da107b` |
| 10 | `constitutions/STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION.md` | `sha256:5e9488cb48a2c3c174832e110bbfe5131bfb008c99e79e2e60e3ea15de105440` |
| 11 | `levels/l-0.3-ecs-query/ecs-query/40_QUERY_DESCRIPTORS.md` | `sha256:eb7a0482a9030ef0d26aac4da015489ede6ef6d64c91132cd29e25a6c3cd172b` |
| 12 | `levels/l-0.6-storage-access/storage-access/43_TRAVERSAL_ENTRY.md` | `sha256:10748f034a140d240ae13a3068751bac4489640a12e369d8b770a55f8c45bba4` |
| 13 | `levels/l0.5-shared-world-properties/material/43_LOOKUP_MODEL.md` | `sha256:08693063a68508e3b4a6b350242a0d045e7cca55a9343e56be6d7671dac6998f` |
| 14 | `levels/l1-runtime-kernel/runtime-headless/40_HEADLESS_PROFILE.md` | `sha256:73c1543c96b1f0363586cf30326b7543ef370d1c8bc3ca7c76f229c3c2d958ff` |
| 15 | `levels/l1-runtime-kernel/runtime-headless/41_SIMULATION_CADENCE.md` | `sha256:abdd4470a9a5a0a70eee8eaf23b4dd9db8a50e3d6633eece6bf8da606589278a` |
| 16 | `levels/l1-runtime-kernel/runtime-headless/42_HEADLESS_OUTPUTS.md` | `sha256:737d5a329fe1c68646ff5d80d1e5a2c4fb54a6eacc912b23e514fb8eb6b1b1eb` |
| 17 | `levels/l1-runtime-kernel/runtime-headless/43_HEADLESS_ROLE_MODEL.md` | `sha256:35fbc39cce8bda17b546a373953da2b7f377c5b3c77fc4f9b4f5279073cae9f0` |
| 18 | `levels/l1-runtime-kernel/runtime-realtime/40_REALTIME_PROFILE.md` | `sha256:71e7ac07f36f85efc402adc0d49208cb179d7295d6d06640ad35772967c7b3fa` |
| 19 | `levels/l1-runtime-kernel/runtime-realtime/41_FRAME_CADENCE.md` | `sha256:479524fe8649284f16912f2d3e81f0154c9567469b5ae0a0d3550ac9803ea639` |
| 20 | `levels/l1-runtime-kernel/runtime-realtime/42_REALTIME_OUTPUTS.md` | `sha256:de26d8deeb349fe1f767df74ff1e2c515cbe89d96b79d89aad33e4c3ff45e17a` |
| 21 | `levels/l1-runtime-kernel/runtime-realtime/43_PRESENTATION_POLICY.md` | `sha256:822bdc24c05c02753c703d896f3217785bdc6901907b6e5077a74b409e4d30e5` |
| 22 | `levels/l1-runtime-kernel/runtime-realtime/44_REALTIME_ROLE_MODEL.md` | `sha256:f53769bc57a1da284393e795f3016aa8e40786dfae9fb1787bff290281fc3e80` |
| 23 | `levels/l1-runtime-kernel/runtime/45_RESOURCE_COORDINATION.md` | `sha256:90bcba671128042ba007229f6688860d669eebf71eb672bdb585113bc5d4b5c4` |
| 24 | `levels/l1-runtime-kernel/runtime/46_LOW_LATENCY_FRAME_PATH.md` | `sha256:3b4198d881bb477c0dccad83b513c22b45c645e13c953337c77d9a4fcd44b994` |
| 25 | `levels/l2-critical-simulation/agents/40_NAVIGATION.md` | `sha256:c5251df63f3418b67e6086e54e0d651bc01a25369c46884c9239a04e854a2935` |
| 26 | `levels/l2-critical-simulation/agents/41_PERCEPTION.md` | `sha256:13c98a49d337623e850af93c58c9dca4ab22662bf9c5a0f8b824d130864a7bf4` |
| 27 | `levels/l2-critical-simulation/agents/42_DECISION.md` | `sha256:f8d3e64c83bd5efd439ef56686c16903cde88c1e5dbcf5d77fbc70694146dc58` |
| 28 | `levels/l2-critical-simulation/agents/43_SOCIETY.md` | `sha256:fab827ba719093b74139de1405daeb8eddefac2d3c71fe52f48721f2a8de9d70` |
| 29 | `levels/l2-critical-simulation/agents/44_SCHEDULE.md` | `sha256:865c896e0f685bf463f5a6fdb637d953a917fc8c708a1413f327466c6fb1afaf` |
| 30 | `levels/l2-critical-simulation/field/40_FLUID_FIELD.md` | `sha256:a420a7f834017d9a0455739e9dea77a3110593d91a9aee893aa072729e955cca` |
| 31 | `levels/l2-critical-simulation/field/41_THERMAL_FIELD.md` | `sha256:2f742749773ce35ecd261f865f72fb6f4e03ab81c57d5ff4e4cbd77487498caf` |
| 32 | `levels/l2-critical-simulation/field/42_TERRAIN_FIELD.md` | `sha256:21cacda93c2a894709fbf93d2e8825893bf20f66bba9fafc10da4f99dffbc0eb` |
| 33 | `levels/l2-critical-simulation/field/43_STRUCTURAL_FIELD.md` | `sha256:7894ecd23d0bd4d7eb1b32c91eac49e30a800649aef97efa08b68137b78e67e8` |
| 34 | `levels/l2-critical-simulation/field/44_ATMOSPHERIC_FIELD.md` | `sha256:e8d8969063cf173e172825fd7aa356be461c9b790c2bc232af6ed13d1248b97d` |
| 35 | `levels/l2-critical-simulation/kinetics/40_COLLISION.md` | `sha256:305ad9af588069d3fffcd49c605f5178e8865099a1c4a9ee5440918fdc049b6f` |
| 36 | `levels/l2-critical-simulation/kinetics/41_RIGIDBODY.md` | `sha256:930d3d1cdded5f580a90bae03abfcf03a7bf4999b6ae7c1b14dd9a87275917d7` |
| 37 | `levels/l2-critical-simulation/kinetics/42_TRAJECTORY.md` | `sha256:8830f3b57314350ec6a6eaacde5134792f00b118e9205d2832c4666fe3e43711` |
| 38 | `levels/l2-critical-simulation/kinetics/43_IMPACT.md` | `sha256:7b37680e625aa3174328de0df7d203e3cb3497f09d79132b7b4da2a10632922d` |
| 39 | `levels/l2.5-network-runtime-services/net-transport/40_CONNECTION_MODEL.md` | `sha256:35fb16379bc67e845bcddb4b02004ef6c159fdbfc0821ef7cd1acb4f1b8561c3` |
| 40 | `levels/l2.5-network-runtime-services/net-transport/41_PACKET_LANES.md` | `sha256:b47f9f87bd85f70731865f414bc7fb36ad837352d67627b522ba81aa61030796` |
| 41 | `levels/l2.5-network-runtime-services/net-transport/42_SECURITY_AND_SESSION.md` | `sha256:5c2c66a0bd020a097f29999c5b38e695420d399bcd62ebd6f63242ffdb78c10e` |
| 42 | `levels/l3.1-synthesis-systems/acoustics/40_PROPAGATION.md` | `sha256:cfcb94f450c44dde81deb347e947c8c6223b41a7b2e31cca68e9c42289f66d64` |
| 43 | `levels/l3.1-synthesis-systems/acoustics/41_REFLECTION_AND_OCCLUSION.md` | `sha256:f6c706fcd33c795ed0d90226128607d198e0ca90fe744336a5cbe40f0917fc65` |
| 44 | `levels/l3.1-synthesis-systems/acoustics/43_ACOUSTIC_OUTPUTS.md` | `sha256:b916bd5362343afaed0ae25488769541827ace6e6768b86cb8ed3903ca966f93` |
| 45 | `levels/l3.1-synthesis-systems/imaging/44_RESOURCE_RESIDENCY.md` | `sha256:198984fd1c01a2eafd499b7639f35b2c56f2ba8572127cc08dad5e60fd060bdb` |
| 46 | `levels/l3.1-synthesis-systems/imaging/45_UPLOAD_STAGING.md` | `sha256:6b3c6d1018113c4bda2a2a6e83ef69254ace6d563d116825d6f61ac996177e90` |
| 47 | `levels/l3.1-synthesis-systems/imaging/46_FRAME_RESOURCE_POLICY.md` | `sha256:acb6fa418b4ac057f1cfc870ce5d8fb091872ee7e8e694ff84accce6f978692b` |
| 48 | `levels/l4-startup/startup/41_PROFILE_SELECTION.md` | `sha256:1f912197685478cb87db53564fb39d64315d5e45b81a3c8cae785b2497d936f9` |
| 49 | `levels/l4-startup/startup/42_RUNTIME_WIRING.md` | `sha256:671699488dfd510401176144f9cee34a4a068bdce5fba776039b6544266bcb05` |
| 50 | `levels/l4-startup/startup/44_NETWORK_ROLE_SELECTION.md` | `sha256:85d762b08e6f677bc6921baf684eacde4144fa0e860dfd3699c91da90b754386` |
| 51 | `levels/l4-startup/startup/45_RESOURCE_SERVICE_WIRING.md` | `sha256:e7a37351991224b188de9656a185b6d0638c65bf98b7fed24cbded2fe78bbd2a` |

## Rule

`NUMSRC-R13-02` is legal only for stack marker `SX-CANON/1.0.24/STACK-v30` and only for the exact file digests registered here.
