# content_browser_system Level

Canonical layer: `content_browser_system`
Activation class: `warm-panel`.

## Owns
- browser tab state
- filter/search/sort state
- folder/path navigation state
- selection presentation state
- context-menu and action-bar state

## Consumes
- asset/dependency/reference projections
- build/release/quarantine status
- source-control and validation posture where legal

## Emits
- import/reimport/rebuild/export intents
- validate/find-reference intents
- create variant/prefab intents
- bundle/addressable/streamable intents

## Never owns
- asset authority
- import/build/release truth
- dependency truth
