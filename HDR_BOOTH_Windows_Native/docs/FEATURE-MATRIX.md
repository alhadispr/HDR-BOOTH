# HDR BOOTH feature matrix (Fotobu reference)

| Feature area identified from supplied APK / requested scope | Windows scaffold |
|---|---|
| Tauri 2 desktop shell | Included |
| USB camera support (UVC + gphoto2 family visible in APK) | UI and adapter boundary; Windows driver/SDK integration required |
| Canon EOS capture + live view | Integration boundary; hardware validation required |
| Camera controls and preview | Studio screen placeholder |
| Sessions / event workflow | Demo UI |
| Gallery / photo actions | Demo UI |
| Community frames / custom overlays | Feature screen scaffold |
| Portrait / landscape / photo-strip layouts | Feature screen scaffold |
| Printer settings / print dialog | Print Center scaffold; Epson driver integration required |
| QR / share / download | Screen scaffold; real URL requires backend |
| Offline-first local storage / sync queue | UI demo; SQLite + uploader still required |
| Upload modes / manual sync | Screen scaffold |
| Voucher / discounts | Screen scaffold |
| Payment gate / DOKU / wedding no-payment mode | Screen scaffold; server-side verification required |
| Multi-printer profiles | UI scaffold |
| User/admin authentication and cloud service | Not implemented; backend required |

## Device compatibility scope
- Canon: broad EOS family via official EDSDK, with per-model runtime capability detection; not every model guarantees live view/remote capture.
- Epson: L-series via installed Windows printer queues and official model drivers; advanced paper/borderless options vary by driver.
