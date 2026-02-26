# Audit API (portfolio-api)

**Tanggal audit:** 2026-02-27  
**Ruang lingkup:** portfolio-api (Lumen 10) — routes, controllers, models, middleware, validasi, keamanan, konfigurasi, dan testing.

---

## 1. Ringkasan eksekutif

| Aspek | Status | Catatan |
|-------|--------|---------|
| Struktur & routing | Baik | Route publik vs terproteksi jelas; prefix `api`, throttle per grup. |
| Autentikasi & otorisasi | Baik | Bearer token via AuthServiceProvider; 401 untuk mutasi tanpa token; pesan login generik. |
| Validasi input | Baik | Semua controller mutasi dan input publik memakai Validator; aturan max length, exists, email, url. |
| Respons API | Baik | Bentuk seragam (data, message, errors); trait ApiResponses; Handler untuk 404/422/500. |
| Keamanan | Baik | Password hash, token tidak di response user, CORS dari env, rate limit; sesuai AUDIT_REPORT_ISO27001. |
| Testing | Baik | PHPUnit; ApiTest mencakup 401, 201, 422, 429, struktur respons. |
| Dokumentasi | Baik | OpenAPI/Swagger di `/docs`; README dan .env.example lengkap. |

**Temuan minor:** Throttle login memakai parameter `throttle:10,1` tetapi middleware hanya memakai parameter pertama (10); decay tetap dari env — perilaku sudah benar (10 req/menit untuk login). Tidak ada log terstruktur untuk kejadian keamanan (login sukses/gagal); rekomendasi tetap seperti di audit ISO 27001.

---

## 2. Struktur aplikasi

### 2.1 Stack

- **Lumen** 10.x, **PHP** ^8.1
- **Eloquent** (MySQL/MariaDB)
- **Dependensi dev:** PHPUnit, Faker, Mockery

### 2.2 Direktori utama

| Path | Isi |
|------|-----|
| `app/Http/Controllers/` | 16 controller (User, Experience, Education, SkillCategory, Skill, UserSkill, Project, ProjectSkill, BlogPost, Tag, PostTag, Certification, ContactMessage, Login, PublicContact) |
| `app/Http/Middleware/` | CorsMiddleware, Authenticate, ThrottleRequests |
| `app/Http/Traits/` | ApiResponses (successResponse, errorResponse) |
| `app/Models/` | 13 model Eloquent (relasi, fillable, hidden) |
| `app/Providers/` | AuthServiceProvider, AppServiceProvider, EventServiceProvider |
| `app/Exceptions/` | Handler (JSON ApiError, APP_DEBUG mengontrol pesan 500) |
| `routes/web.php` | Semua route API + docs |
| `database/migrations/` | 21 migration (users, experiences, educations, skills, projects, blog_posts, tags, contact_messages, dll.) |

---

## 3. Routing

### 3.1 Route publik (tanpa auth)

- **GET** `/` — versi aplikasi
- **GET** `/docs`, `/docs/`, `/docs/openapi.yaml` — dokumentasi OpenAPI/Swagger
- **GET** `api/*` — users, experiences, educations, skill-categories, skills, user-skills, projects, project-skills, blog-posts, tags, post-tags, certifications (index + show). **Throttle:** 60/menit per IP.
- **POST** `api/contact` — form kontak publik. **Throttle:** 5/menit per IP.
- **POST** `api/login` — login username/password, kembalikan token + user. **Throttle:** 10/menit per IP.

### 3.2 Route terproteksi (middleware `auth`)

- **POST/PUT/PATCH/DELETE** untuk semua resource di atas (users … certifications).
- **GET/POST/PUT/PATCH/DELETE** `api/contact-messages` — hanya admin.

**Temuan:** Pemisahan publik vs admin konsisten dengan [PUBLIKASI_WEB.md](PUBLIKASI_WEB.md) dan [ARSITEKTUR.md](ARSITEKTUR.md). Blog posts dan projects memfilter `is_published` saat tidak ada auth.

---

## 4. Autentikasi dan otorisasi

### 4.1 Mekanisme

- **AuthServiceProvider:** Token dari header `Authorization: Bearer <token>` atau query `api_token`. User diload dari `User::where('api_token', $token)`.
- **LoginController:** Validasi username/password; `Hash::check()`; token baru `Str::random(60)` disimpan di `users.api_token`; response berisi `token` dan `user` (id, full_name, username — tanpa password/api_token di payload).
- **Authenticate middleware:** Jika guest → response JSON 401 `Unauthorized.`

### 4.2 Keamanan kredensial

- **User model:** `password` dan `api_token` di `$hidden` (tidak muncul di JSON).
- **UserController:** Password tidak di-mass-assign; di-hash dengan `Hash::make()` lalu di-set langsung pada model.
- **Pesan error login:** "Kredensial tidak valid" (tidak membocorkan username vs password).

**Temuan:** Conform dengan praktik aman; selaras dengan [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md) A.8.5.

---

## 5. Validasi input

### 5.1 Pola umum

- Semua controller yang menerima input memakai `Validator::make($request->all(), [...])`; pada gagal mengembalikan `errorResponse('Validation failed', $validator->errors(), 422)`.
- Data yang dipakai: `$validator->validated()` (store/update).

### 5.2 Contoh aturan

| Endpoint / field | Aturan utama |
|------------------|--------------|
| Login | username required\|string\|max:255; password required\|string |
| Public contact | name, email, subject, message required; email email; message max:65535 |
| User (store/update) | full_name required; email_public email; profile_image_url url; password nullable\|min:6 |
| BlogPost | title required; slug unique; excerpt nullable\|string\|max:65535; content required\|max:16777215 |
| Project | title required; url/repository_url url; end_date after_or_equal:start_date |

**Temuan:** Validasi konsisten; batas panjang (excerpt, content) sesuai migrasi dan dokumentasi; tidak ada input kritis yang tidak divalidasi.

---

## 6. Respons dan penanganan error

### 6.1 Bentuk respons

- **Sukses:** `{ "data": ..., "message": "...", "errors": null }` (status 200/201).
- **Error:** `{ "data": null, "message": "...", "errors": ... }` (errors berisi object untuk validasi 422).
- **Trait ApiResponses:** `successResponse()`, `errorResponse()` dipakai oleh semua controller.

### 6.2 Exception Handler

- **ValidationException** → 422, errors dari validator.
- **ModelNotFoundException** → 404 "Resource not found".
- **HttpException** → status code sesuai, message dari exception.
- **Lainnya** → 500; message = `$exception->getMessage()` jika `APP_DEBUG=true`,否则 "Internal server error".

**Temuan:** Konform dengan skema OpenAPI ApiResponse/ApiError; production tidak membocorkan stack trace.

---

## 7. CORS dan rate limit

### 7.1 CORS

- **CorsMiddleware** (global): Origin dari env `CORS_ORIGINS` (comma-separated); method GET, POST, PUT, PATCH, DELETE, OPTIONS; header Content-Type, Authorization, X-Requested-With, Accept. Jika `*` atau kosong → allow origin `*`.

### 7.2 Rate limit

- **ThrottleRequests:** Berbasis Cache; key per IP dan window waktu (THROTTLE_DECAY_MINUTES). Limit per route: 60 (GET api), 5 (POST contact), 10 (POST login). Response 429: `{ "data": null, "message": "Too many requests.", "errors": null }`.

**Temuan:** Sesuai kebutuhan; production disarankan CORS_ORIGINS spesifik (bukan *).

---

## 8. Konfigurasi dan environment

### 8.1 .env.example

- **App:** APP_NAME, APP_ENV, APP_KEY, APP_DEBUG, APP_URL, APP_TIMEZONE
- **Log:** LOG_CHANNEL, LOG_SLACK_WEBHOOK_URL
- **DB:** DB_CONNECTION, DB_HOST, DB_PORT, DB_DATABASE, DB_USERNAME, DB_PASSWORD
- **Lain:** CACHE_DRIVER, QUEUE_CONNECTION, CONTACT_OWNER_USER_ID, CORS_ORIGINS, THROTTLE_MAX_ATTEMPTS, THROTTLE_DECAY_MINUTES

### 8.2 Rahasia

- `.env` dan `.env.podman` terdaftar di .gitignore; tidak ada rahasia hardcoded di kode.

---

## 9. Testing

### 9.1 PHPUnit

- **Script:** `composer test` (phpunit).
- **ApiTest:** DatabaseMigrations; helper `userWithToken()`, `authHeader()`.

### 9.2 Skenario yang dites

- GET `/` → 200, konten versi.
- GET `api/users`, `api/experiences`, `api/skills`, `api/projects`, `api/blog-posts` → 200, struktur (data, message, errors).
- GET `api/users/99999` → 404.
- POST `api/users` tanpa token → 401.
- POST `api/users` dengan token + payload valid → 201, data user.
- POST `api/users` dengan full_name kosong → 422, errors.
- PUT/DELETE `api/users/99999` dengan token → 404.
- POST `api/contact-messages` tanpa token → 401.
- POST `api/contact-messages` dengan token + payload valid → 201.
- POST `api/contact` (publik) payload valid → 201, message sukses.
- POST `api/contact` validasi gagal → 422.
- POST `api/contact` 6x → 429 (rate limit).

**Temuan:** Cakupan memadai untuk endpoint kritis (auth, kontak publik, rate limit, validasi, 401/404).

---

## 10. Rekomendasi

| Prioritas | Rekomendasi |
|-----------|-------------|
| Tinggi (production) | Set APP_DEBUG=false, APP_KEY, CORS_ORIGINS spesifik; ganti password default seeder. |
| Sedang | Pertimbangkan log kejadian keamanan (login sukses/gagal, akses contact-messages) tanpa log kredensial/token; dokumentasikan di prosedur. |
| Rendah | Tambah tes eksplisit untuk login sukses dan login gagal (401); opsional tes filter is_published untuk GET blog-posts/projects tanpa auth. |

---

## 11. Referensi

- [README portfolio-api](../portfolio-api/README.md) — Setup, migrate, token, Swagger, testing.
- [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md) — Kontrol keamanan informasi (API & admin).
- [PUBLIKASI_WEB.md](PUBLIKASI_WEB.md) — Perilaku endpoint blog-posts dan projects (publik vs admin).
- [ARSITEKTUR.md](ARSITEKTUR.md) — Posisi API dalam stack.

---

*Audit API selesai 2026-02-27. Untuk checklist production dan keamanan mendalam, gunakan [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md).*
