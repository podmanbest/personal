# Audit Proyek Portfolio Stack

**Tanggal audit:** 2026-02-27  
**Ruang lingkup:** Struktur monorepo (portfolio-api, portfolio-admin, portfolio-web), dokumentasi, dependensi, konfigurasi, dan konsistensi dengan standar proyek.

---

## 1. Ringkasan eksekutif

| Aspek | Status | Catatan |
|-------|--------|---------|
| Struktur & layout | Baik | Tiga subproyek jelas; compose.yaml selaras dengan folder. |
| Dokumentasi | Baik | docs/ lengkap (SRS, arsitektur, rancangan, deploy); indeks terpusat. |
| Naming & referensi | Sudah diperbaiki | "api-portfolio" telah diseragamkan ke "portfolio-api" di DEPLOY.md dan README subproyek (2026-02-27). |
| Keamanan & env | Baik | .env tidak di-commit; token di sessionStorage; composer audit bersih. |
| Kualitas kode frontend | Kurang | Tidak ada ESLint; tidak ada script test di admin/web. |
| Deploy & container | Baik | Containerfile ada di ketiga app; compose terdefinisi. |

---

## 2. Struktur proyek

### 2.1 Folder dan peran

| Path | Peran | Build/run |
|-----|--------|-----------|
| `portfolio-api` | Backend Lumen (REST API, auth, CRUD) | Composer, `php artisan serve` |
| `portfolio-admin` | Panel admin React (Vite, Tailwind 4) | npm, port 3001 |
| `portfolio-web` | Situs publik React (Vite) | npm, port 3000 |
| `docs/` | Dokumentasi (SRS, arsitektur, rancangan, changelog, audit) | — |
| Root | Scripts install/build/dev untuk admin & web | `npm run install:all`, `build`, `dev:admin`, `dev:web` |

**Temuan:** Struktur sesuai [README.md](../README.md) dan [ARSITEKTUR.md](ARSITEKTUR.md). `compose.yaml` memakai konteks `./portfolio-api`, `./portfolio-admin`, `./portfolio-web` — konsisten dengan nama folder.

### 2.2 Template-admin

Folder `template-admin` ada di workspace (Shadcn Admin, TypeScript, TanStack Router) dan dirujuk sebagai referensi UI/UX untuk pembangunan ulang admin. Tidak tercantum di README root; sengaja atau tidak — jika dipakai hanya sebagai referensi, bisa ditambahkan satu baris di README (mis. "template-admin: referensi UI/UX untuk admin").

---

## 3. Dokumentasi

### 3.1 Kelengkapan

- **Indeks:** [docs/README.md](README.md) memetakan semua dokumen dan "Mulai dari mana?".
- **SDLC:** [PANDUAN_DOKUMENTASI_RPL.md](PANDUAN_DOKUMENTASI_RPL.md) memetakan dokumen ke tahap analisis, perancangan, implementasi, pengujian, deploy.
- **Kebutuhan:** SRS-PORTFOLIO, RINGKASAN_RANCANGAN, ARSITEKTUR, DIAGRAM_DAN_ERD, PERANCANGAN_ADMIN, PUBLIKASI_WEB, ALUR_KONTEN_POST, RANCANGAN_WEB_UI_UX, RANCANGAN_ADMIN_UI_UX.
- **Deploy:** DEPLOY.md (root); CHANGELOG.md di docs.

**Temuan:** Dokumentasi sangat lengkap dan mengikuti standar yang dideskripsikan di PANDUAN_DOKUMENTASI_RPL.

### 3.2 Ketidakseragaman nama folder API

Beberapa dokumen sebelumnya menyebut folder backend sebagai **api-portfolio**, sedangkan nama folder aktual adalah **portfolio-api**. **Perbaikan telah diterapkan (2026-02-27):** semua referensi "api-portfolio" di dokumen berikut telah diganti menjadi "portfolio-api".

| Dokumen | Status |
|---------|--------|
| [DEPLOY.md](../DEPLOY.md) | Sudah diseragamkan ke `portfolio-api`. |
| [portfolio-admin/README.md](../portfolio-admin/README.md) | Sudah diseragamkan ke `portfolio-api`. |
| [portfolio-web/README.md](../portfolio-web/README.md) | Sudah diseragamkan ke `portfolio-api`. |
| [portfolio-api/README.md](../portfolio-api/README.md) | Path dan URL contoh sudah memakai `portfolio-api`. |

---

## 4. Dependensi dan keamanan

### 4.1 Backend (portfolio-api)

- **Composer:** Lumen 10, PHP ^8.1; PHPUnit untuk testing.
- **composer audit:** Tidak ada advisory kerentanan (dijalankan saat audit).
- **.env:** Terdaftar di .gitignore; .env.example ada dan tidak menyimpan rahasia.

### 4.2 Frontend (portfolio-admin, portfolio-web)

- **portfolio-admin:** React 18, Vite 5, Tailwind 4, react-router-dom, TanStack Table, Headless UI, react-select, @uiw/react-md-editor, mermaid (setelah pembersihan: react-hook-form, zod, @hookform/resolvers, autoprefixer, postcss dihapus).
- **portfolio-web:** React 18, Vite 5, react-router-dom, react-markdown, react-syntax-highlighter, react-hook-form, zod, mermaid.
- **npm audit:** Disarankan dijalankan berkala (`npm audit` di masing-masing folder). Laporan keamanan sebelumnya ([AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md)) mencatat 2 moderate (esbuild/vite) pada dev dependency; tetap pantau dan upgrade saat aman.

### 4.3 Lockfile

- portfolio-api: composer.lock (implied by Composer workflow).
- portfolio-admin & portfolio-web: package-lock.json (ada di repo) — baik untuk reproducible build.

---

## 5. Konfigurasi

### 5.1 Environment

- **portfolio-api:** .env.example lengkap (APP_KEY, DB_*, CORS_ORIGINS, CONTACT_OWNER_USER_ID, throttle). .env dan .env.podman di .gitignore.
- **portfolio-admin:** .env.example hanya VITE_API_URL (opsional saat pakai proxy). .env* di .gitignore.
- **portfolio-web:** .env.example ada; sama, proxy dev ke API.

### 5.2 Vite proxy

- Admin (port 3001) dan web (port 3000) mem-proxy `/api` ke `http://localhost:8000` — konsisten dengan arsitektur "frontend memanggil API".

### 5.3 Root package.json

- Scripts: `install:all`, `build`, `build:admin`, `build:web`, `dev:admin`, `dev:web`, `preview:admin`, `preview:web`. Tidak ada script untuk API (API dijalankan terpisah dengan Composer/artisan) — sesuai README.

---

## 6. Kualitas kode dan tooling

### 6.1 Linting (ESLint)

- **Temuan:** Tidak ada konfigurasi ESLint di portfolio-admin maupun portfolio-web (tidak ada file eslint.config.js / .eslintrc*).
- **Rekomendasi:** Tambah ESLint (mis. eslint-plugin-react, eslint-plugin-react-hooks) dan script `"lint": "eslint src"` agar gaya kode dan bug umum terdeteksi.

### 6.2 Testing

- **portfolio-api:** PHPUnit; script `composer test` (phpunit). Sesuai dokumentasi.
- **portfolio-admin:** Tidak ada script test (tidak ada Jest/Vitest).
- **portfolio-web:** Tidak ada script test.

**Rekomendasi:** Untuk peningkatan kualitas jangka panjang, pertimbangkan menambah Vitest (atau Jest) di admin/web dengan minimal smoke test atau unit test untuk modul kritis (mis. auth, api client).

### 6.3 Pre-commit / CI

- Tidak ada pre-commit hook atau CI workflow (GitHub Actions dll.) yang terlihat di root atau subproyek (kecuali template-admin yang punya .github/workflows).
- **Rekomendasi:** Opsional: tambah workflow CI (install, lint, test, build) untuk portfolio-api, portfolio-admin, portfolio-web.

---

## 7. Keamanan (singkat)

- Token admin disimpan di sessionStorage (auth.js); tidak hardcode di kode.
- API: kredensial dan CORS dikontrol via env; login throttle; pesan error generik — sesuai [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md).
- .env dan .env.podman tidak di-commit.

---

## 8. Git dan ignore

- Root .gitignore: .cursor/, data/, dll. Duplikat `.cursorignorerules` telah dibersihkan (2026-02-27).
- portfolio-admin / portfolio-web: node_modules, dist, .env*, log, editor — standar.
- portfolio-api: vendor, .env, .env.podman, .phpunit.result.cache — standar.

---

## 9. Rekomendasi prioritas

| Prioritas | Aksi |
|-----------|------|
| — | ~~Tinggi~~ Done | Penulisan nama folder API telah diseragamkan ke "portfolio-api" di DEPLOY.md dan README admin/web/API. |
| Sedang | Tambah ESLint di portfolio-admin dan portfolio-web; tambah script `lint` di package.json. |
| — | ~~Sedang~~ Done | Duplikat `.cursorignorerules` di root .gitignore telah dibersihkan. |
| Rendah | Tambah satu baris di README root tentang template-admin jika dipakai sebagai referensi. |
| Rendah | Pertimbangkan Vitest/Jest dan script test untuk admin/web; CI opsional. |

---

## 10. Audit sintaks dan dependensi (2026-02-27)

### 10.1 portfolio-admin

- **Dependensi yang dihapus (tidak terpakai di kode):**
  - `@hookform/resolvers` — tidak ada `zodResolver` atau `useForm` di src.
  - `react-hook-form` — form CRUD memakai state manual (`formData`/`setFormData`), bukan useForm.
  - `zod` — tidak ada impor atau validasi schema Zod di src.
- **DevDependencies yang dihapus:** `autoprefixer`, `postcss` — proyek memakai Tailwind 4 via `@tailwindcss/vite` tanpa PostCSS terpisah (tidak ada postcss.config.js).
- **Sintaks:** Build Vite (`npm run build`) berhasil tanpa error.

### 10.2 portfolio-web

- Semua dependency terpakai: `react-hook-form`, `zod`, `@hookform/resolvers` di Contact.jsx; `react-markdown`, `react-syntax-highlighter`, `mermaid` di MarkdownContent.jsx. Tidak ada yang dihapus.
- **Sintaks:** Build Vite (`npm run build`) berhasil tanpa error.

### 10.3 portfolio-api

- `fakerphp/faker` dipakai di `database/factories/UserFactory.php`. `mockery/mockery` dan `phpunit/phpunit` dipakai untuk testing. Tidak ada dependensi yang dihapus.

---

## 11. Referensi

- [README (root)](../README.md) — Struktur dan cara jalankan.
- [docs/README.md](README.md) — Indeks dokumentasi.
- [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md) — Audit keamanan API & admin.
- [PANDUAN_DOKUMENTASI_RPL.md](PANDUAN_DOKUMENTASI_RPL.md) — Standar dokumentasi proyek.

---

*Terakhir diperbarui: 2026-02-27 (audit sintaks dan penghapusan dependensi tidak terpakai).*
