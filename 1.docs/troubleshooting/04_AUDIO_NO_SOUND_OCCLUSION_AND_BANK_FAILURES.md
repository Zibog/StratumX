# Audio No Sound, Occlusion, and Bank Failures

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **troubleshooting**.


| Symptom | Recovery |
|---|---|
| No sound | run `route.audio.explain_silence`. |
| Event missing | create or bind event definition. |
| Bank missing | cook bank. |
| Material step silent | add material sound row or fallback. |
| Sound muffled/wrong | inspect occlusion/portal trace. |
| Sound culled | inspect mix priority and bus. |
