# Dependencies

This contract belongs specifically to the l6.f7 combat ballistics damage authoring family and describes family-only coordination.


## Family dependency posture for `combat_ballistics_damage_authoring_family`
- family composition may touch combat, ballistics, and damage authoring graphs and tuning rules
- family composition may touch authority-facing minimal truth: combat/ballistics edit intents
- family composition may touch snapshot classes: combat snapshots
- family composition may touch index classes: combat lookup indices
- family composition may touch derived classes: derived combat views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination
