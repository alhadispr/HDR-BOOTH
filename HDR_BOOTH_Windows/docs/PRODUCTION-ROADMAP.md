# Release checklist

## Implemented in this source iteration
- Tauri 2 desktop shell and Windows NSIS/MSI bundle configuration
- SQLite schema and Rust commands for sessions, captures and settings
- Original HDR BOOTH navigation and workflow scaffold
- Feature inventory for the supplied Fotobu APK

## Required before production release
- [x] Wire session creation/list UI to SQLite commands
- [ ] Wire remaining UI state to SQLite commands; implement migrations/backup/restore
- [ ] Canon EOS adapter: exact camera model + Windows SDK, USB discovery, EVF live view, capture, transfer, error recovery
- [ ] Epson adapter: installed Windows driver, paper sizes, borderless, orientation, multi-copy and two-profile routing
- [ ] Frame designer: PNG alpha overlays, layout editor, rendering pipeline and export
- [ ] QR sharing backend: authenticated upload, expiring public URL, QR generation and download page
- [ ] Offline-first sync worker: persistent queue, retries, conflict handling, resumable uploads
- [ ] DOKU server integration: create payment, signed callback verification, status polling, idempotency; never store secret keys in client
- [ ] Voucher rules and server validation; wedding/free session policy
- [ ] Authentication, roles, audit log, privacy and retention controls
- [ ] Hardware QA on Windows 10 and 11, installer signing, crash recovery and update strategy

## External inputs needed
- Exact Canon EOS model(s) and firmware
- Exact Epson printer model(s), paper sizes and driver version
- DOKU merchant sandbox/production credentials and backend endpoint
- Cloud hosting/API and domain for photo galleries
- Branding assets (logo, colors) if not using the starter defaults

## Device family requirement
The requested target is all Canon EOS families and all Epson L-series printers. Treat this as a broad compatibility target, not a blanket guarantee. Maintain a tested model/firmware/driver matrix and capability flags. Canon ED-SDK support list and feature limitations are authoritative; Epson driver capabilities are model-specific.
