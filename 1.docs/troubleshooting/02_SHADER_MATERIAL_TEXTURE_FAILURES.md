# Shader, Material, and Texture Failures

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **troubleshooting**.


| Failure | Recovery |
|---|---|
| Shader compile failed | open shader error packet, fix source, rebuild variant. |
| Pipeline layout mismatch | inspect binding layout and material requirements. |
| Missing texture | assign fallback or reimport texture. |
| Wrong color | check sRGB/linear verdict and output color chain. |
| Broken normal map | check normal convention and channel mapping. |
