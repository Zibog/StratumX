# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| plugin_host_session_id | PluginHostSessionId | active plugin host session | unique per editor process |
| registered_dock_set | RegisteredDockSet | docks registered through register_dock | typed and ordered |
| inspector_renderer_set | InspectorRendererSet | registered inspector renderers and component editors | typed and ordered |
| extension_capability_set | ExtensionCapabilitySet | capabilities granted to active plugins | bounded and explicit |
| plugin_validation_state | PluginValidationState | current validation/isolation posture | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `plugin_and_extension_host` without stealing truth from neighboring levels or lower packages.


## Exact plugin registration signatures
- register_dock(tool_id, dock_slot, widget_factory)
- register_inspector_renderer(type_id, renderer)
- register_component_editor(component_type, editor_factory)
- register_asset_importer(asset_kind, importer)
- register_asset_postprocessor(asset_kind, processor)
- register_validation_rule(scope, rule)
- register_command(command_id, handler)
- register_timeline_track(track_type, editor)
- register_viewport_overlay(tool_id, overlay_renderer)
- register_context_menu_extension(target_type, items_provider)
