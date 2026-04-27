# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| profile_selection | ProfileSelection | required | runtime profile choice | owned by `engine_startup` |
| runtime_config | RuntimeConfig | required | runtime initialization config | owned by `engine_startup` |
| service_wiring | ServiceWiring | required | service dependency wiring | owned by `engine_startup` |
| network_role | NetworkRole | required | network participation role | owned by `engine_startup` |
