# API — Backend personal/portfolio

Backend HTTP (Go) untuk situs personal: health, status, auth (JWT), area admin, dan data skills dari database.

## Stack

- **Go 1.21+**
- **MySQL/MariaDB** (opsional; API bisa jalan tanpa DB)
- **JWT** (HS256) untuk login & route protected

## Quick start

```bash
# Konfigurasi (copy dan isi .env)
cp configs/.env.example configs/.env

# Jalankan server (default http://localhost:8080)
go run ./cmd/server
# atau
make run
```

Dengan database: set `DB_DSN` di `.env`, lalu jalankan migrasi:

```bash
go run ./cmd/migrate
```

## Endpoint utama

| Method | Path | Auth | Fungsi |
|--------|------|------|--------|
| GET | `/health` | — | Health check `{"status":"ok"}` |
| GET | `/status` | — | Uptime + status database |
| POST | `/login` | — | Login admin → JWT |
| GET | `/api/skills` | — | Daftar skills (dari DB) |
| GET | `/admin` | Bearer JWT | Area admin |

Detail request/response: **[docs/API.md](docs/API.md)**.

## Konfigurasi

Variabel penting di `.env`: `PORT`, `DB_DSN`, `ADMIN_USERNAME`, `ADMIN_PASSWORD`, `JWT_SECRET`, `ALLOW_ORIGIN` (CORS).  
Penjelasan lengkap: **[docs/CONFIG.md](docs/CONFIG.md)**.

## Perintah

| Perintah | Deskripsi |
|----------|-----------|
| `make run` | Jalankan server |
| `make build` | Build `bin/server` dan `bin/migrate` |
| `make test` | Unit test (`tests/`) |
| `make migrate` | Migrasi DB (up) |
| `make migrate-down` | Rollback migrasi |

## Dokumentasi

Semua dokumentasi API ada di folder **`docs/`**:

- [docs/README.md](docs/README.md) — indeks
- [docs/SETUP.md](docs/SETUP.md) — setup & quick start
- [docs/API.md](docs/API.md) — endpoint
- [docs/CONFIG.md](docs/CONFIG.md) — env
- [docs/DATABASE.md](docs/DATABASE.md) — schema & migrasi
- [docs/TESTS.md](docs/TESTS.md) — unit test
