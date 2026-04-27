# l9.8-audio-voice-authoring-suite Libraries

## Allowed library classes
- audio event and routing descriptors
- voice/dialogue clip helpers
- attenuation/spatialization adapters
- mix and playback preview helpers

## Forbidden library posture
- direct dependency on project-specific runtime/game code
- convenience wrappers that silently widen `l9.8-audio-voice-authoring-suite` beyond its declared editor role
- hidden ownership through undeclared caches, widget internals, or private lower-package surfaces

## Audit rule
Every imported library class must preserve the exact ownership, invalidation, and budget posture declared for `l9.8-audio-voice-authoring-suite` and must stay specific enough to support implementation work without reinterpretation.
