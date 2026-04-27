# Dependencies

This contract belongs specifically to the l6.f0 editor shell family and describes family-only coordination.


## Family dependency posture for `editor_shell_family`
- family composition may touch shell composition, command bar, view hosting, docking, and editor frame routing
- family composition may touch authority-facing minimal truth: minimal shell authority refs only
- family composition may touch snapshot classes: shell snapshots and panel/view composition snapshots
- family composition may touch index classes: panel/view lookup indices
- family composition may touch derived classes: derived shell layouts and command routing views
- package-root registries and lower packages only through member-legal public surfaces

## Forbidden widening
- new lower-stack truth not already legal for these members
- editor product UI ownership outside the family role
- hidden caches or mutable mirrors standing in for family coordination
