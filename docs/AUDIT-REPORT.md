# Laporan Audit Proyek — Personal/Portfolio

**Tanggal:** 22 Februari 2026  
**Scope:** Keamanan, konfigurasi, keselarasan ToDo vs implementasi, siap production.

---

## 1. Ringkasan Eksekutif

| Area | Status | Catatan |
|------|--------|--------|
| **Backend (Go)** | ✅ Baik | Security headers, CORS aman, auth JWT, tidak ada secret hardcode di kode production. |
| **Frontend (Vue)** | ✅ Baik | Graceful degradation, CSP meta, proxy dev. |
| **Keamanan rahasia** | ⚠️ Perlu tindakan | Pastikan `api/configs/.env` tidak ter-commit; perkuat .gitignore. |
| **Siap production** | 🟡 Belum | Domain placeholder, aset OG/CV, CSP connect-src untuk API beda origin. |
| **Dokumentasi vs kode** | ✅ Selaras | ToDo.md dan README sesuai implementasi. |

---

## 2. Keamanan

### 2.1 Environment & rahasia

- **`api/configs/.env`**  
  - File berisi nilai nyata (DB_PASSWORD, ADMIN_PASSWORD, JWT_SECRET).  
  - `api/.gitignore` sudah memuat `.env` dan `.env.local` (seharusnya mengabaikan juga `configs/.env`).  
  - **Rekomendasi:**  
    1. Tambahkan `configs/.env` dan `configs/.env.local` secara eksplisit di `api/.gitignore`.  
    2. Verifikasi dengan `git status` / `git check-ignore` bahwa `api/configs/.env` tidak ter-track.  
    3. Jika file ini pernah ter-commit di history: **rotasi semua secret** (DB password, JWT_SECRET, ADMIN_PASSWORD) dan hapus dari history (mis. `git filter-repo` atau BFG).

- **Root `.gitignore`**  
  - Saat ini tidak mengabaikan `.env`.  
  - **Rekomendasi:** Tambah `.env` dan `.env.local` di root agar tidak ada .env tidak sengaja di-commit di level repo.

- **Kode production**  
  - Config hanya dari env (`config.Load()`); tidak ada hardcode password/secret di handler atau middleware.  
  - Test memakai credential dummy (`secret123`, `test-jwt-secret-...`) — wajar untuk test.

### 2.2 CORS

- **`api/internal/middleware/cors.go`**  
  - Menolak `ALLOW_ORIGIN=*` (log peringatan, header CORS tidak diset).  
  - Hanya mengizinkan origin spesifik dari env.  
  - Sesuai Phase 4 dan aman untuk production.

### 2.3 Security headers (API)

- **`api/internal/middleware/middleware.go`**  
  - X-Content-Type-Options, X-Frame-Options, Referrer-Policy, HSTS (jika HTTPS), CSP, hapus X-Powered-By, Server dikosongkan.  
  - Sesuai `docs/PHASE4-SECURITY.md`.

### 2.4 Frontend (CSP & OG)

- **`web/index.html`**  
  - CSP via meta tag ada; `connect-src 'self'`.  
  - **Production:** Jika API di domain lain (mis. `https://api.example.com`), wajib tambah origin API ke `connect-src` di CSP, mis. `connect-src 'self' https://api.yoursite.com`.  
  - OG/Twitter masih pakai `https://yoursite.com` — harus diganti ke URL production sebelum deploy.

---

## 3. Konfigurasi & Aset

### 3.1 Placeholder domain

| File | Isi saat ini | Tindakan |
|------|--------------|----------|
| `web/index.html` | og:url, og:image, twitter:image → `https://yoursite.com` | Ganti ke domain production. |
| `web/public/sitemap.xml` | Semua `<loc>` pakai `https://yoursite.com` | Ganti ke domain production sebelum deploy. |

### 3.2 Aset yang belum ada

- **`web/public/cv.pdf`**  
  - ToDo: letakkan CV PDF di sini; CI memblok jika > 5 MB.  
  - Saat audit: file tidak ada di `web/public/`.

- **`web/public/og-image.png`**  
  - Dokumen menganjurkan 1200×630 px untuk OG/Twitter.  
  - Saat audit: file tidak ada; OG meta sudah mengarah ke `/og-image.png`.

### 3.3 CI/CD

- **`.github/workflows/ci-cd.yml`**  
  - Build + test API, build web, cek ukuran `cv.pdf` ≤ 5 MB, deploy ke GitHub Pages.  
  - Auto-merge PR memakai `gh pr merge --auto --squash` dengan `GITHUB_TOKEN`.  
  - **Catatan:** Auto-merge bisa memerlukan izin “Allow GitHub Actions to create or approve pull requests” di repo (Settings > Actions > General).

---

## 4. Keselarasan ToDo vs Implementasi

- **Tabel status di ToDo.md**  
  - Backend, DB, config, docs, CI/CD, tests, frontend, admin, graceful degradation, security headers, CORS, kontak mailto/PGP, 404, robots, sitemap, meta OG — semua sesuai dengan kode yang diperiksa.

- **Ceklis phase**  
  - Item yang masih 🔲 (domain, Lighthouse > 90, backup, DNS SPF/DKIM/DMARC, CLI easter egg, analytics) memang belum diimplementasi dan tercatat dengan benar.

- **Langkah berikut di ToDo**  
  - Isi projects & posts, cv.pdf, ganti domain di sitemap, security headers lanjutan — konsisten dengan temuan audit (placeholder domain, cv.pdf, OG image).

---

## 5. Rekomendasi Prioritas

### Segera (sebelum production)

1. **Pastikan `.env` tidak ter-track**  
   - Verifikasi `git status` / `git check-ignore api/configs/.env`.  
   - Tambah aturan eksplisit di `api/.gitignore` dan `.env` di root .gitignore (lihat Section 2.1).  
   - Jika pernah ter-commit: rotasi secret dan bersihkan history.

2. **Ganti placeholder domain**  
   - Di `web/index.html` (og:*, twitter:*) dan `web/public/sitemap.xml` ganti `yoursite.com` ke domain production.

3. **Siapkan aset production**  
   - Tambah `web/public/cv.pdf` (≤ 5 MB).  
   - Tambah `web/public/og-image.png` (1200×630 px).

4. **CSP untuk API beda origin**  
   - Jika frontend dan API beda domain, tambah origin API ke `connect-src` di CSP di `web/index.html`.

### Opsional / berikutnya

- Lighthouse > 90 (ToDo Phase 2).  
- Backup strategy & DNS (SPF, DKIM, DMARC) (Phase 5).  
- Analytics privacy-friendly, CLI easter egg (Phase 4 & 6).  
- Verifikasi izin auto-merge di GitHub Actions jika memakai fitur tersebut.

---

## 6. Checklist Verifikasi Pre-Deploy

- [ ] `api/configs/.env` tidak ter-track di Git; secret di production dari env hosting.
- [ ] Root dan `api/.gitignore` mengabaikan `.env` / `configs/.env`.
- [ ] `web/index.html`: domain production di OG/Twitter; `connect-src` CSP termasuk origin API jika beda domain.
- [ ] `web/public/sitemap.xml`: semua URL pakai domain production.
- [ ] `web/public/cv.pdf` ada dan ≤ 5 MB.
- [ ] `web/public/og-image.png` ada (1200×630 px).
- [ ] Backend: `ALLOW_ORIGIN` set ke origin frontend production (bukan `*`).
- [ ] Env backend di hosting: PORT, DB_DSN (atau DB_*), ADMIN_*, JWT_SECRET, ALLOW_ORIGIN.
