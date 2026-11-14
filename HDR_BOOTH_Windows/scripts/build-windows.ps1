$ErrorActionPreference = "Stop"
$ProjectRoot = Split-Path -Parent $PSScriptRoot
Set-Location $ProjectRoot
Write-Host "HDR BOOTH Professional - Windows x64 installer build" -ForegroundColor Cyan
foreach ($cmd in @("node", "npm", "cargo", "rustc")) {
    if (-not (Get-Command $cmd -ErrorAction SilentlyContinue)) {
        throw "$cmd belum tersedia. Pasang Node.js 22 dan Rust stable (MSVC) terlebih dahulu."
    }
}
Write-Host "1/3 Installing frontend dependencies..."
npm install
if ($LASTEXITCODE -ne 0) { throw "npm install gagal." }
Write-Host "2/3 Building frontend..."
npm run build
if ($LASTEXITCODE -ne 0) { throw "Frontend build gagal." }
Write-Host "3/3 Building NSIS installer..."
npm run tauri -- build --bundles nsis
if ($LASTEXITCODE -ne 0) { throw "Tauri/NSIS build gagal." }
$installerDir = Join-Path $ProjectRoot "src-tauri\target\release\bundle\nsis"
if (-not (Test-Path $installerDir) -or -not (Get-ChildItem $installerDir -Filter *.exe)) {
    throw "File installer .exe tidak ditemukan di $installerDir"
}
Write-Host "Berhasil. Installer ada di: $installerDir" -ForegroundColor Green
