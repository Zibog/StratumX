# Request Result Model

## Request classes
- shell composition updates
- global mode broadcasts
- dock host registration updates

## Result posture
- every long-running request returns a typed result ref, status stream, or failure code
- cancellation and supersede behavior must be explicit
- no request may mutate truth outside declared ownership boundaries
