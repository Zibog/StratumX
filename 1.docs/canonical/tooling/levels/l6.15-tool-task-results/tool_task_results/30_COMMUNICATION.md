# tool_task_results communication

Ingress:
- preview/build/release/validation completion publications
- task cancellation or failure completions
- post-processing summaries

Egress:
- task result rows keyed by task_result_id
- result-state publications
- result refs and completion cursors

Communication law:
the sidecar is append-only publication traffic, never hidden authority mutation.
