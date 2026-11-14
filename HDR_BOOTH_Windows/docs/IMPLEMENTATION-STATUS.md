# Implementation status — current development build

Updated in this iteration:
- Frontend session list now requests persisted session records from the Tauri Rust backend.
- Creating a session now invokes the Rust `create_session` command and refreshes the SQLite-backed list.
- App startup attempts to load sessions from SQLite and displays a desktop/database readiness status.
- Existing database schema and commands remain in `src-tauri`.

Still not implemented as production features:
- Camera transport/EDSDK, live view and real capture.
- Saving real captured image files and linking their metadata to the SQLite capture table.
- Real Windows printer enumeration/spooler printing.
- Frame editor and high-resolution composition renderer.
- QR generation backed by an expiring gallery service.
- Cloud uploader and retry worker.
- DOKU backend/payment verification.
- Authentication, secure credential storage, backup/restore.

Validation in this environment:
- Node.js/npm are present.
- Rust/Cargo toolchain is not installed in the build environment.
- Dependency installation did not complete, so no frontend or Windows installer build has been certified here.
- This archive is source code, not an installable EXE.


## Update: local session workflow
- SQLite session listing now includes per-session photo counts.
- Session close operation is available and updates the local session status.
- Dashboard totals are derived from the local session database rather than hard-coded sample totals.
- Studio capture control no longer claims a simulated capture was a real saved photo. Real capture remains blocked until a Canon Windows camera adapter is integrated and tested.
- Build and hardware validation have not been completed in this environment.


## Windows installer pipeline
- Added a GitHub Actions workflow targeting Windows x64 and NSIS `.exe` packaging.
- The workflow has not been executed from this workspace; no installer binary is being claimed as built.
- Local environment is Linux and lacks Rust/Cargo, so native Windows Tauri compilation cannot be performed here.
