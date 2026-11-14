# Product requirements — single workstation / Fotobu feature parity

## Non-negotiable product decisions
- Product name: HDR BOOTH Professional.
- One Windows computer per booth installation. No multi-PC synchronization or multi-workstation administration is required.
- UI language: Bahasa Indonesia first; plain labels, guided setup and clear status indicators.
- Visual direction: modern, uncluttered HDR BOOTH design, not a pixel-copy of Fotobu.
- Feature goal: functional parity with the feature areas of the supplied Fotobu 1.1.1 reference APK, plus the requirements explicitly given in this project.
- Platform: Windows 10/11 x64.
- Cameras: broad Canon EOS family support with runtime capability detection.
- Printers: Epson L-series using installed official Windows drivers.

## Single-computer operating model
The local Windows PC is the source of truth for active booth operations:
- SQLite database stored under the Windows app-data directory.
- Original captures stored locally under `Pictures/HDR BOOTH` (configurable).
- Sessions, captures, frame selection, print queue, voucher usage and sync queue remain usable offline.
- One local operator/admin profile can manage the booth; optional PIN lock can be added.
- Printer profiles 1 and 2 are queues installed on this same Windows PC.
- Cloud is only for guest delivery, remote backup, community resources, account/license and payment verification. Cloud outage must not block offline/free workflows.
- No server-to-server sync between booth PCs is in scope.

## Feature parity checklist
These are acceptance requirements, not claims that the starter already implements them.

### Device and capture
- [ ] Detect camera over USB and show model/connection state.
- [ ] Connect/disconnect and recover from cable loss.
- [ ] Live view with mirror toggle.
- [ ] Remote capture, configurable countdown, capture preview.
- [ ] Retake, delete, review and save original.
- [ ] Expose only camera settings supported by the connected EOS model.
- [ ] Manual import/fallback workflow for unsupported tethered models.

### Sessions and gallery
- [ ] Create/edit/close event sessions.
- [ ] Session history and per-session gallery.
- [ ] Search/filter photos; select, preview, retake where source exists, delete with confirmation, export.
- [ ] Local file organization and safe database/file cleanup.
- [ ] Session-level upload mode and guest access controls.

### Frames and output
- [ ] Community frame discovery/download when online.
- [ ] Custom frame import (PNG transparency) and frame assignment.
- [ ] Layout editor / templates, portrait and landscape.
- [ ] Photo strip and standard print layouts.
- [ ] Render final composite at print resolution and save digital original.
- [ ] Preview frame/layout before a live session.

### Printing
- [ ] Detect installed Epson L-series Windows printer queues.
- [ ] Configure two printer profiles on this computer.
- [ ] Select paper size, orientation, copies and driver-supported borderless/media settings.
- [ ] Print one or multiple copies; queue status, retry and error reporting.
- [ ] Test print and printer selection per layout.

### Guest delivery and offline sync
- [ ] QR code / expiring gallery link per photo or session.
- [ ] Guest photo download page and access controls.
- [ ] Upload automatically, at end of session, or manually.
- [ ] Persistent offline queue with pending/uploading/synced/failed states.
- [ ] Retry after internet returns; never lose local originals when upload fails.
- [ ] Optional cloud backup and session sharing.

### Business workflow
- [ ] Voucher codes and discount rules.
- [ ] DOKU payment intent creation and server-side payment verification before paid capture.
- [ ] Wedding/free mode with no payment step.
- [ ] Payment, refund/cancel and transaction history where supported by merchant backend.
- [ ] Operator-friendly session status and simple start/end flow.

### Settings and security
- [ ] Guided first-run setup for camera, printers, storage and cloud.
- [ ] Workstation/device status dashboard.
- [ ] Local backup/restore and log export.
- [ ] Credentials stored securely; DOKU secret never stored in frontend/client.
- [ ] Guest privacy, configurable retention/deletion, and safe error messages.

## Definition of done
A feature is complete only when it has:
1. A real backend implementation, not a demo toast or placeholder.
2. Persisted data where appropriate.
3. A clear success/failure state and recovery behavior.
4. Automated or repeatable test evidence.
5. Hardware or service validation for integrations.
6. Documentation for operator setup.

## Important scope limits
“Support all Canon EOS” means the application attempts to identify the camera and uses the supported Canon SDK path where that exact model and feature are listed. It must show unavailable capabilities honestly. “Support all Epson L-series” means using the exact installed Windows driver; driver-specific options vary by printer model and region. Maintain a tested model/firmware/driver matrix rather than claiming universal identical functionality.
