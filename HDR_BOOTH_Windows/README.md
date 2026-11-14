# HDR BOOTH Professional — Windows Edition (Starter)

Modern Windows desktop photobooth application scaffold inspired by the feature areas found in the supplied Fotobu 1.1.1 APK. This is an original implementation scaffold, not a copy of Fotobu source code or assets.

## Included
- Tauri 2 + Vite + vanilla TypeScript UI shell
- Dashboard and navigation for Studio, Sessions, Gallery, Frames, Print, QR & Sharing, Sync, Vouchers, Payments, Settings
- Local demo interactions: create session, countdown/capture simulation, retake/delete, print queue, sync queue, settings persistence in browser local storage
- Rust/Tauri command scaffold for local app services
- Camera/printer/cloud integration boundaries and Windows build instructions

## Not yet production-ready
This source package is a UI + application scaffold. It does not yet contain Canon EOS live-view implementation, Epson printer driver integration, DOKU credentials/API integration, cloud backend, production QR upload endpoint, or a signed installer. These require hardware/model tests, credentials, and backend details.

## Requirements
- Windows 10/11 x64
- Node.js LTS, Bun or npm
- Rust stable + MSVC C++ Build Tools
- WebView2 Runtime
- Canon EDSDK (if using Canon EDSDK adapter) or validated gphoto2 Windows build
- Epson printer driver

## Run
1. Install Rust and Visual Studio Build Tools (Desktop development with C++).
2. Install Node.js LTS.
3. In this folder: `npm install`
4. `npm run tauri dev`

## Build installer
`npm run tauri build`

The generated installer will be under `src-tauri/target/release/bundle/` after building on Windows.

## Local database foundation
The Rust backend now initializes SQLite in the OS app-data directory and exposes commands for sessions, captures, and key/value settings. The migration is in `src-tauri/migrations/001_init.sql`. The frontend demo screens are still being wired to these commands; do not treat demo interactions as persisted production data until that integration is completed.

## Confirmed deployment scope
Single Windows workstation per booth. The workstation owns local sessions, photos, SQLite data and print queues. Cloud is used only for guest links, backup/community services and payment verification; no multi-booth PC management is required. See `docs/PRODUCT-REQUIREMENTS.md` for the complete Fotobu feature-parity acceptance checklist.

Latest iteration: session creation and session listing are wired to the SQLite Tauri commands. See `docs/IMPLEMENTATION-STATUS.md` for the exact implementation and validation status.


## Automated Windows installer build
This repository includes `.github/workflows/windows-installer.yml`. Push this folder to a GitHub repository, then open **Actions → Build HDR BOOTH Windows Installer → Run workflow**. The Windows runner builds the NSIS `.exe` and publishes it as a downloadable workflow artifact. A tagged release (`v1.0.0`, for example) also triggers the workflow.

Important: this pipeline packages the current application scaffold. It does **not** mean camera capture, Epson printing, cloud QR delivery, or DOKU payment have passed production/hardware acceptance tests. Those integrations remain prerequisites before using this as a paid-event production booth.


## PENTING: ZIP ini bukan installer
Arsip ini belum berisi file `.exe`. Ini adalah source project dan konfigurasi build. Untuk membuat installer pada Windows 10/11 x64:
1. Install Node.js 22.
2. Install Rust stable dengan MSVC toolchain dan Visual Studio Build Tools (Desktop development with C++).
3. Klik dua kali `BUILD-INSTALLER-WINDOWS.bat`.
4. Jika build berhasil, installer berada di `src-tauri\\target\\release\\bundle\\nsis\\`.

Jangan mengganti nama ZIP/source menjadi `.exe`; itu tidak membuat installer. Build memerlukan internet untuk mengunduh dependensi. Installer yang berhasil dibangun juga belum berarti kamera, printer, pembayaran, dan cloud sudah lolos uji perangkat nyata.


## Source validation note (2026-11-14)
- TypeScript source passed a global `tsc --noEmit` syntax/type check in the available Linux workspace after correcting a malformed dashboard template interpolation.
- This is not a full frontend build: Vite/Tauri npm dependencies could not be installed before the execution time limit.
- A native Windows `.exe` has not been produced. The current environment has no Rust/Cargo or Windows MSVC toolchain.
