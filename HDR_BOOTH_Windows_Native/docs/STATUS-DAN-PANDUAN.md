# HDR BOOTH Professional — status dan panduan operator

## Status build ini

Build ini adalah prototipe desktop lokal yang dapat dikembangkan menjadi aplikasi booth. Bukan pengganti Fotobu yang sudah terverifikasi dan belum boleh dipakai untuk acara berbayar tanpa uji perangkat.

| Modul | Status nyata |
|---|---|
| Desktop Windows (Tauri 2) | Source tersedia; installer belum dibangun di lingkungan ini |
| Database SQLite lokal | Aktif; membuat dan membaca sesi/foto |
| Kamera | WebView `getUserMedia`; kamera yang dikenali Windows termasuk webcam utility bila kompatibel |
| Canon EOS EDSDK langsung | Belum tersedia |
| Simpan foto | Aktif; JPEG ke Pictures/HDR BOOTH dan indeks SQLite |
| Galeri lokal | Aktif untuk sesi yang dipilih |
| Frame overlay | Fitur kompositor PNG sederhana pada modul yang disertakan; uji output diperlukan |
| Voucher | CRUD, validasi, penebusan dan hitung pemakaian lokal tersedia |
| Printer/Epson | Belum diimplementasikan pada build ini |
| QR tamu / galeri online | Belum tersedia |
| Supabase / Google Drive | Belum aktif/teruji |
| DOKU / QRIS | Belum terintegrasi; jangan menerima pembayaran dengan aplikasi ini |
| GIF, filter, editor layout multi-slot | Belum selesai |

## Menjalankan di PC Windows 10/11 x64

1. Pasang Node.js LTS.
2. Pasang Rust stable melalui rustup.
3. Pasang Microsoft Visual Studio Build Tools dengan workload **Desktop development with C++** dan Windows SDK.
4. Ekstrak source project ke folder lokal.
5. Buka PowerShell pada folder project dan jalankan:

```powershell
npm install
npm run tauri dev
```

Untuk membuat installer:

```powershell
npm run tauri build
```

Artefak installer Tauri biasanya berada di `src-tauri/target/release/bundle/`. Lokasi dan keberhasilan harus diperiksa pada PC build.

## Alur uji booth lokal

1. Jalankan aplikasi dan buat sesi baru.
2. Buka Studio, pilih perangkat kamera yang terdeteksi Windows.
3. Pastikan preview tampil, lalu ambil foto percobaan.
4. Periksa file JPEG pada `Pictures/HDR BOOTH/<NamaSesi>/`.
5. Buka Galeri setelah memilih sesi tersebut.
6. Buat voucher uji dan gunakan fitur hitung (tidak mengurangi kuota); tombol Tebus akan menambah pemakaian.
7. Tutup dan buka kembali aplikasi, lalu pastikan sesi dan foto masih ada.

## Catatan kamera

Kamera yang tampil sebagai perangkat video Windows dapat dipakai lewat `getUserMedia`. Untuk Canon EOS, hasilnya bergantung model, firmware, koneksi, dan apakah EOS Webcam Utility menyediakan perangkat video. Jalur ini bukan kendali kamera EDSDK: tidak menjamin remote shutter, live view penuh, fokus, atau semua model Canon.

## Keamanan

- Jangan simpan DOKU Secret Key, Supabase service-role key, atau Google OAuth client secret di frontend.
- Publishable/anon key Supabase bukan pengganti aturan RLS. Terapkan RLS dan validasi server-side.
- Pembayaran hanya boleh dianggap lunas setelah callback/webhook diverifikasi backend.
- Voucher lokal hanya promosi/discount record, bukan bukti pembayaran.
- Buat backup manual folder foto dan database sebelum acara penting.

## Kriteria sebelum produksi

Lakukan build Release di PC target, tes kamera dengan model yang akan digunakan, tes cetak pada printer dan media aktual, tes offline, tes restart/restore, serta audit integrasi cloud dan pembayaran. Jangan aktifkan fitur berbayar sebelum webhook dan entitlement diuji.

## Build installer otomatis melalui GitHub Actions

Workflow `.github/workflows/windows-installer.yml` disiapkan untuk runner Windows x64. Setelah source diunggah ke repository GitHub milik Anda:
1. Buka tab **Actions**.
2. Pilih **Build HDR BOOTH Windows Installer**.
3. Klik **Run workflow** dan jalankan pada branch utama.
4. Tunggu job selesai, lalu unduh artifact `HDR-BOOTH-Professional-Windows-x64`.
5. Artifact memuat installer `.exe` (NSIS) dan `.msi` bila kedua bundler berhasil.

Workflow ini belum dijalankan dari repository Anda, jadi belum ada installer yang dihasilkan atau diverifikasi.
