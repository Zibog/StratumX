# Forbidden Connections

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

The editor package must not:
- bypass `L6 command/transaction` for mutations
- bypass `L6 validation/preview/build/release` with private panel jobs
- keep shadow copies of entity/component/prefab/layer/package truth
- let plugins own hidden mutable authority stores
- let assistant surfaces mutate through undisclosed side channels
- let runtime-inspector projections masquerade as committed authoring state
- let build/release surfaces invent artifact truth not backed by manifests
- let an app host or prototype UI bypass the editor package into sdk/tooling for active sky/weather features
- keep two parallel editor paths alive for the same active feature thread
- let the viewport consume mock-only sky state after the canonical delivery path is frozen
