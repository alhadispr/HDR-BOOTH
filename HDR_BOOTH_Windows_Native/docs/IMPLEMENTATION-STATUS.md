# HDR BOOTH Professional — Implementation Status

## Implemented in this native-app development milestone
- Tauri 2 Windows desktop shell and Vite/TypeScript frontend.
- SQLite database stored in the app's local data directory.
- Create/list/close/reopen session commands.
- Session photo counts from SQLite.
- USB webcam capture through WebView `getUserMedia` where the Windows WebView and privacy permissions allow it.
- JPEG capture saved to `Pictures/HDR BOOTH/<session>/` and indexed in SQLite.
- Local gallery preview for captures in the selected session.
- Asset protocol scoped to the Pictures directory for local image previews.

## Not yet implemented / not validated
- Direct Canon EOS EDSDK control, live view and remote shutter across all EOS models.
- Epson printer discovery, print profiles, job queue and real print execution.
- PNG frame editor, image compositing and final print-resolution rendering.
- Public QR gallery, guest download links, cloud sync and Google Drive integration.
- DOKU/QRIS server-side transaction creation and verification.
- Voucher engine, subscription/licensing server and add-on entitlements.
- Windows kiosk lockdown and admin exit protection.
- Automated installer build and physical hardware acceptance tests.

## Build verification
- TypeScript parser/type check was run; only the Tauri API module could not be resolved because npm dependencies could not be installed in this environment.
- Rust/Cargo are unavailable in this environment, so native compilation and `.exe` generation have not been performed.
- Windows CI workflow is included to build the NSIS installer on a Windows runner.


## Voucher engine (latest)
- Voucher codes are stored in SQLite with unique case-insensitive codes.
- Supports percentage and fixed-Rupiah discounts, optional use limits, expiry dates, enable/disable.
- `validate_voucher` calculates a quote without consuming a use.
- `redeem_voucher` rechecks eligibility and records redemption atomically in a SQLite transaction.
- Redemption is a local promotion record only. It is NOT payment verification and must not unlock paid features by itself.
