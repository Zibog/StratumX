# Validation command order

## Fast full run on Windows PowerShell

```powershell
Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass -Force
Get-ChildItem .\scripts\*.ps1 | Unblock-File
.\scripts\gold-validation.ps1
```

## Manual order

```powershell
.\scripts\setup-dev.ps1
.\scripts\test-all.ps1
.\scripts\smoke-all.ps1
.\scripts\bench-all.ps1
.\scripts\build-evidence-pack.ps1
.\scripts\build-phase-status.ps1
```
