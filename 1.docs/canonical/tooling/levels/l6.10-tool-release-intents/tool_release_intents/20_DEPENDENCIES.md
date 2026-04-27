# tool_release_intents dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6.13-tool-activation-state`
- `l6.13-build-runtime`
- `l6.14-release-runtime`

Dependency law:
`tool_release_intents` may depend only on the listed surfaces because it publishes release-facing intents such as build channel selection, packaging class, or publish requests.
