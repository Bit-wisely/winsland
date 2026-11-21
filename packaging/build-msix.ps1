# Build a (dev-signed) MSIX. Requires the Windows SDK (makeappx, signtool).
$ErrorActionPreference = "Stop"
cargo build --release

$stage = "target\msix"
Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
New-Item $stage -ItemType Directory | Out-Null
Copy-Item target\release\winisland.exe $stage
Copy-Item packaging\AppxManifest.xml $stage
Copy-Item assets $stage -Recurse

makeappx pack /d $stage /p target\WinIsland.msix /o
# signtool sign /fd SHA256 /a /f dev.pfx /p <password> target\WinIsland.msix
Write-Host "Built target\WinIsland.msix (sign it before installing)"
