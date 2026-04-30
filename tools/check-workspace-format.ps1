$ErrorActionPreference = 'Stop'

$metadataJson = cargo metadata --format-version 1 --no-deps
if ($LASTEXITCODE -ne 0) {
    throw "cargo metadata failed with exit code $LASTEXITCODE"
}

$metadata = $metadataJson | ConvertFrom-Json
$workspaceMembers = @{}
foreach ($member in $metadata.workspace_members) {
    $workspaceMembers[$member] = $true
}

$manifests = @(
    $metadata.packages |
        Where-Object { $workspaceMembers.ContainsKey($_.id) } |
        Select-Object -ExpandProperty manifest_path |
        Sort-Object
)

foreach ($manifest in $manifests) {
    cargo fmt --manifest-path $manifest --check
    if ($LASTEXITCODE -ne 0) {
        throw "cargo fmt --manifest-path $manifest --check failed with exit code $LASTEXITCODE"
    }
}
