# Indeks Dokumentasi Proyek

Daftar dokumen di folder `docs/` dan dokumen penting di luar folder, beserta ringkasan isi dan untuk siapa dokumen tersebut berguna.

---

## Dokumen di folder docs/

### [SRS-PORTFOLIO.md](SRS-PORTFOLIO.md)

Spesifikasi kebutuhan perangkat lunak untuk aplikasi web portfolio (kebutuhan fungsional, non-fungsional, batasan, dan antarmuka). Berperan sebagai **Product Vision & Product Backlog awal** sekaligus Bab 1–2 laporan RPL. Berguna untuk **stakeholder**, **dosen/penguji**, **tech lead**, dan **developer**.

---

### [RINGKASAN_RANCANGAN.md](RINGKASAN_RANCANGAN.md)

Ringkasan rancangan dan rencana pengembangan (backlog fitur utama) yang menghubungkan kebutuhan di SRS dengan dokumen desain dan implementasi. Berguna untuk **tech lead** dan **developer** yang ingin konteks rencana pengembangan lintas sprint.

---

### [ARSITEKTUR.md](ARSITEKTUR.md)

Gambaran stack (portfolio-api, portfolio-admin, portfolio-web, database), diagram alur klien–API, tabel komponen, serta ringkasan akses (tanpa token vs dengan token). Mewakili bagian utama **Bab Perancangan / Sprint 0 (architecture sprint)**. Berguna untuk **developer baru** dan **tech lead** yang ingin paham sistem secara keseluruhan sebelum menyentuh kode.

---

### [DIAGRAM_DAN_ERD.md](DIAGRAM_DAN_ERD.md)

Diagram dan ERD: (1) arsitektur stack (klien–API–DB), (2) Entity Relationship Diagram database (users, experiences, educations, projects, blog_posts, skills, tags, pivot tables, dll.), (3) alur publikasi (publik vs admin), (4) alur admin login dan auto-fill, (5) alur deploy container. Semua dalam format Mermaid. Menguatkan **Bab Perancangan** dengan representasi visual. Berguna untuk **developer** dan **tech lead** yang perlu gambaran visual model data dan alur sistem.

---

### [PERANCANGAN_ADMIN.md](PERANCANGAN_ADMIN.md)

Perancangan dan implementasi fitur Admin: login dan current user, auto-fill user_id saat buat konten, form relasi sebagai select (nama bukan ID), list tampilan relasi dengan nama, menu sidebar dropdown per kelompok (terbuka saat halaman aktif). Termasuk diagram alur dan referensi file. Mewakili desain detail untuk beberapa item Sprint Backlog. Berguna untuk **developer** yang mengerjakan atau memelihara admin.

---

### [PUBLIKASI_WEB.md](PUBLIKASI_WEB.md)

Perilaku API untuk request publik vs admin: GET blog-posts dan GET projects tanpa token hanya mengembalikan item yang dipublikasi; dengan token admin mendapat semua. Tabel endpoint, referensi controller, dan diagram singkat. Mengikat rancangan API ke kebutuhan publikasi konten. Berguna untuk **developer** dan **QA** yang perlu memastikan siapa melihat data apa.

---

### [ALUR_KONTEN_POST.md](ALUR_KONTEN_POST.md)

Alur konten blog post dari dua sisi: pengunjung (portfolio-web) dan admin (portfolio-admin). Menjelaskan langkah demi langkah bagaimana post dibuat sebagai draft, dipublish, difilter oleh API (is_published, auth()->check()), lalu muncul di daftar dan detail blog publik, lengkap dengan diagram alur. Mewakili dokumentasi fitur hasil satu atau beberapa Sprint. Berguna untuk **developer**, **QA**, dan **penulis konten** yang ingin memahami siklus hidup post end-to-end.

---

### [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md)

Laporan audit keamanan informasi (API & admin) mengacu kontrol Annex A ISO/IEC 27001:2022. Checklist per kontrol, bukti, dan rekomendasi. Menjadi artefak kualitas untuk Sprint Review dan hardening. Berguna untuk **auditor keamanan**, **compliance**, dan **developer** yang menindaklanjuti temuan.

---

### [PANDUAN_DOKUMENTASI_RPL.md](PANDUAN_DOKUMENTASI_RPL.md)

Panduan dokumentasi RPL/SDLC: tahap Analisis Kebutuhan (BRD, SRS, User Story, MoSCoW), Perancangan (SDD, ERD, UML, API), Implementasi (README, Changelog), Pengujian (Test Plan, UAT), Deployment & Maintenance (Deployment Guide, User Manual). Memetakan dokumen proyek ke tiap tahap SDLC dan ke artefak Scrum, serta mencatat celah/rekomendasi. Berguna untuk **mahasiswa RPL**, **tech lead**, **QA**, dan **dosen/penguji** yang ingin struktur dokumen mengacu standar SDLC dan Scrum.

---

## Dokumen di luar folder docs/

| Dokumen | Lokasi | Isi | Untuk siapa |
|---------|--------|-----|-------------|
| **Deploy (container)** | [DEPLOY.md](../DEPLOY.md) | Build dan jalankan stack dengan Podman/Docker Compose; migrasi DB; akses service. | DevOps, developer yang deploy |
| **Root project** | [README.md](../README.md) | Struktur proyek, cara jalankan dari root (install, build, dev), link ke dokumentasi. | Semua |
| **Setup API** | [portfolio-api/README.md](../portfolio-api/README.md) | Persyaratan, env, migrate, token, Swagger, testing. | Developer backend |
| **Setup Admin** | [portfolio-admin/README.md](../portfolio-admin/README.md) | Setup, scripts, fitur admin. | Developer frontend admin |
| **Setup Web** | [portfolio-web/README.md](../portfolio-web/README.md) | Setup, scripts, halaman publik. | Developer frontend web |

---

## Mulai dari mana?

- Ingin **gambaran kebutuhan & ruang lingkup sistem** → baca [SRS-PORTFOLIO.md](SRS-PORTFOLIO.md).
- Ingin **ringkasan rencana dan backlog fitur** → baca [RINGKASAN_RANCANGAN.md](RINGKASAN_RANCANGAN.md).
- Ingin **gambaran sistem secara teknis** → baca [ARSITEKTUR.md](ARSITEKTUR.md).
- Ingin **diagram dan ERD** → baca [DIAGRAM_DAN_ERD.md](DIAGRAM_DAN_ERD.md).
- Ingin **detail fitur admin** → baca [PERANCANGAN_ADMIN.md](PERANCANGAN_ADMIN.md).
- Ingin **publik vs admin API** → baca [PUBLIKASI_WEB.md](PUBLIKASI_WEB.md).
- Ingin **alur konten blog post end-to-end** → baca [ALUR_KONTEN_POST.md](ALUR_KONTEN_POST.md).
- Ingin **audit keamanan** → baca [AUDIT_REPORT_ISO27001.md](AUDIT_REPORT_ISO27001.md).
- Ingin **panduan dokumentasi RPL/SDLC & mapping Scrum** → baca [PANDUAN_DOKUMENTASI_RPL.md](PANDUAN_DOKUMENTASI_RPL.md).
- Ingin **jalankan / deploy** → [README.md](../README.md) dan [DEPLOY.md](../DEPLOY.md); setup per app di README masing-masing subproyek.
