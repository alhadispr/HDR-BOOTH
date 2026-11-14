# Epson L-series printing strategy

## Target
Support Epson EcoTank / L-series printers for which Epson provides a Windows 10/11 driver. Avoid model-specific printer SDK dependencies for ordinary photo printing.

## Print pipeline
1. Windows enumerates installed printers and their driver names.
2. Operator chooses Printer Profile 1 or 2, each storing a Windows printer queue name.
3. HDR BOOTH renders the final image to a print-ready bitmap/PDF at the selected physical dimensions and DPI.
4. Send the job to the selected Windows print queue, respecting paper size, orientation, copies, margins and color mode supported by the installed driver.
5. Track queued / printing / completed / error where the Windows spooler exposes it.

## Compatibility caveat
The L-series name covers many generations and regional variants. Availability of a driver, paper sizes, borderless support, duplex and status reporting differs by model and driver. If Windows can install and print to the queue, the generic print path can usually print raster output; advanced options must be checked per driver. Use Epson's official support page to obtain the exact model's driver:
https://www.epson.com/support

## QA checklist per model
- Install official driver on clean Windows 10 and Windows 11.
- Print 4x6 / 4R, 5R, A4 and photo-strip layout as required.
- Test portrait and landscape, borderless and margins.
- Test 1/2/4 copies and both printer profiles.
- Verify color management and paper/media selection.
- Disconnect/reconnect printer during a job and confirm recovery.
