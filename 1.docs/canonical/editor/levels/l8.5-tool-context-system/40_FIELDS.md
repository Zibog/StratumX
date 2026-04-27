# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| tool_context_id | ToolContextId | active tool context identity | unique per active surface |
| active_tool_kind | ToolKind | current active tool kind | finite enum only |
| mode_state | ToolModeState | local/world/snap etc mode flags | explicit and bounded |
| selection_scope | SelectionScope | scope the tool currently operates on | must resolve through editor selection state |
| tool_capability_set | ToolCapabilitySet | legal operations in the current context | must remain typed |

## Field law
The records above are the minimum editor-owned state needed to drive `tool_context_system` without stealing truth from neighboring levels or lower packages.


## Exact transform tool labels
- Select
- Move
- Rotate
- Scale
- Rect / Area select
- Pivot edit
- Snap toggle
- Surface align
- Vertex snap
- Grid snap
- Local / World toggle

## Exact placement tool labels
- Asset drag-drop spawn
- Brush placement
- Scatter / foliage tool
- Spline placement
- Road/river tool
- Decal placement
- Light placement
- Volume placement
- Spawn marker placement

## Exact prefab/world/debug tool labels
- Create prefab from selection
- Open prefab in isolation
- Create variant
- Nest prefab
- Apply selected override
- Revert selected override
- Diff prefab
- Unpack
- Validate prefab contract
- Find all instances
- Cell visualizer
- Cell Load/Unload
- Region lock
- World chunk save
- HLOD bake trigger
- Navigation rebuild region
- Streaming preview
- Layer load mask preview
- Runtime spawn heatmap
- Reference graph by region
