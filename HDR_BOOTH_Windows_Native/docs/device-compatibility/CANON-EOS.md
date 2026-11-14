# Canon EOS compatibility strategy

## Target
Support the broadest practical range of EOS DSLR and mirrorless cameras on Windows 10/11 x64 using Canon's official EOS Digital SDK (EDSDK) where the model is listed, with a capability probe at runtime.

Canon's published ED-SDK V13.21.10 support page lists 60+ EOS/PowerShot models. This is not the same as a guarantee that every camera exposes identical remote capture or live-view capabilities. Some EOS M models are explicitly listed as not supporting remote capture. See Canon's official developer resources:
- https://asia.canon/en/campaign/developerresources/sdk
- https://asia.canon/en/campaign/developerresources/cap

## Adapter design
- `ICameraProvider`: enumerate, connect, disconnect, get capabilities.
- `ICameraSession`: get device info, start/stop live view, capture, download, set supported properties.
- `CameraCapabilities`: `CanLiveView`, `CanRemoteCapture`, `CanSetExposure`, `CanSetIso`, `CanSetAperture`, `CanSetShutter`, `CanSetWhiteBalance`.
- `CameraProviderEdsdk`: official Windows x64 EDSDK implementation.
- Optional `CameraProviderCcapi`: Wi-Fi for camera models with CCAPI support enabled.
- Optional fallback: manual import/watch-folder capture mode for unsupported models. Never pretend fallback mode is tethered live view.

## Runtime compatibility
At connection time, identify model and firmware, query capabilities and expose unsupported controls as disabled with a reason. Live-view and capture capability must be reported separately. Keep EDSDK calls serialized on one worker thread; copy downloaded EVF frames before passing them to UI. Handle camera busy, sleep, USB disconnect, card full, battery low, and SDK error events.

## Distribution
EDSDK binaries and headers are not included in this project. Obtain the current SDK under Canon's developer terms, use the correct x64 library, and comply with its redistribution/licensing terms. Do not bundle an unofficial or reverse-engineered binary.
