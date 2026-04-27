# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| help_surface_id | HelpSurfaceId | active help surface identity | unique per host |
| help_topic_ref | HelpTopicRef | current topic, tutorial, or onboarding flow | typed and explicit |
| command_hint_set | CommandHintSet | visible command/help hints | bounded and explicit |
| learning_progress_ref | LearningProgressRef | progress of guided onboarding flow | typed and explicit |
| help_action_set | HelpActionSet | legal next/back/open-doc actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `learning_onboarding_and_help_surface` without stealing truth from neighboring levels or lower packages.
