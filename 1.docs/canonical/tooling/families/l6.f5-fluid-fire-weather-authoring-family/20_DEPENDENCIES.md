# Dependencies

This contract belongs specifically to the l6.f5 fluid fire weather authoring family family and describes family-only coordination.


## Family dependency posture for `fluid_fire_weather_authoring_family`
- family composition may touch fluid, fire, weather authoring, field rules, and weather cell authoring
- family composition may touch authority-facing minimal truth: fluid/fire/weather edit intents
- family composition may touch snapshot classes: fluid/fire/weather snapshots
- family composition may touch index classes: field lookup indices
- family composition may touch derived classes: derived field views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination
