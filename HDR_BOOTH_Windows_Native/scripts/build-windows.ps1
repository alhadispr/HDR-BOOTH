$ErrorActionPreference = "Stop"
Write-Host "HDR BOOTH Professional - Windows x64 installer build"
foreach ($tool in @("node", "npm", "cargo", "rustc")) {
  if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
    throw "$tool belum terpasang atau belum masuk PATH."
  }
}
$target = rustc -vV | Select-String "host:"
if ($target -notmatch "x86_64-pc-windows-msvc") {
  Write-Warning "Target Rust bukan x86_64-pc-windows-msvc. Build installer mungkin gagal."
}
Write-Host "Installing frontend dependencies..."
npm install --no-audit --no-fund
if ($LASTEXITCODE -ne 0) { throw "npm install gagal." }
Write-Host "Checking frontend..."
npm run build
if ($LASTEXITCODE -ne 0) { throw "Frontend build gagal." }
Write-Host "Building NSIS + MSI installers..."
npm run tauri -- build --bundles nsis,msi
if ($LASTEXITCODE -ne 0) { throw "Tauri installer build gagal." }
$bundle = Join-Path $PSScriptRoot "..\src-tauri\target\release\bundle"
Write-Host "Installer output:"
Get-ChildItem $bundle -Recurse -Include *.exe,*.msi | Select-Object FullName,Length
