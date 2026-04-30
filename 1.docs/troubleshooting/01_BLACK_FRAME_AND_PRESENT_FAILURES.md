# Black Frame and Present Failures

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **troubleshooting**.


## Symptoms and actions

| Symptom | Likely cause | Recovery |
|---|---|---|
| Black viewport | no frame submitted, invalid camera, missing material, shader failure | Open Render Doctor and black-frame trace. |
| Present failed | swapchain lost, surface invalid, device lost | Recover swapchain or recreate surface. |
| Clear color only | framegraph has no scene passes or all objects culled | Inspect visible set and framegraph. |
| Capture missing | capture path unavailable or no present image | Run capture recovery route. |
