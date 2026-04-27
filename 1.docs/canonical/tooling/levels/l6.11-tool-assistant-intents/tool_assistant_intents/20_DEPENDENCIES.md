# tool_assistant_intents dependencies

Legal dependencies:
- `l6.0-authority-core`
- `l6.0-tool-session`
- `l6a.2-proposal-runtime`
- `l6a.3-lowering-runtime`
- `l6a.4-safety-gates`

Dependency law:
`tool_assistant_intents` may depend only on the listed surfaces because it publishes assistant-originated suggestions or non-authoritative intents before they are lowered into legal commands.
