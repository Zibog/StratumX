# Communication

## Ingress
- preview/build/release/validation completion publications
- task cancellation or failure completions
- post-processing summaries

## Egress
- task result rows keyed by task_result_id
- result-state publications
- result refs and completion cursors

## Communication law
`tool_task_results` is publication-oriented. It may ingest bounded upstream signals, normalize them into typed sidecar rows, and emit only the declared publication classes above. Direct mutation authority, hidden callbacks, or implicit UI ownership are illegal.
