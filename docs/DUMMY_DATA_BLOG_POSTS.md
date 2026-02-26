# Dummy Data Blog Posts

Dokumen ini berisi contoh **data dummy** untuk resource `blog-posts`.  
Data bisa digunakan sebagai referensi saat seeding database atau pengujian tampilan blog.

---

## Skema Sederhana

Field utama yang dipakai di API dan admin:

- `user_id` — ID user pemilik post.
- `title` — Judul post.
- `slug` — Slug URL unik (opsional, diisi otomatis jika kosong).
- `excerpt` — Ringkasan singkat.
- `content` — Konten utama dalam format Markdown.
- `published_at` — Tanggal publikasi (opsional).
- `is_published` — Status publikasi (0 = draft, 1 = published).

---

## Contoh 1 — Published: “Membangun Portfolio Modern dengan React dan Lumen”

```json
{
  "user_id": 1,
  "title": "Membangun Portfolio Modern dengan React dan Lumen",
  "slug": "portfolio-modern-react-lumen",
  "excerpt": "Ringkasan langkah membangun portfolio modern dengan stack React (Vite) untuk frontend dan Lumen untuk API.",
  "content": "# Membangun Portfolio Modern dengan React dan Lumen\n\nPortfolio modern biasanya terdiri dari tiga bagian utama:\n\n1. **Frontend publik** untuk pengunjung (portfolio-web).\n2. **Admin panel** untuk mengelola konten (portfolio-admin).\n3. **REST API** yang menjadi jembatan antara frontend dan database (portfolio-api).\n\nPada stack ini, frontend menggunakan **React + Vite**, sedangkan backend memakai **Lumen**. Semua service dijalankan sebagai container sehingga mudah di-deploy ke Podman atau Docker.\n\n## Fitur yang dibahas\n\n- Routing halaman publik (home, about, projects, blog, contact).\n- Admin panel bergaya CMS untuk mengelola blog posts dan projects.\n- Pemisahan data publik vs admin menggunakan flag `is_published` di API.\n",
  "published_at": "2026-02-01T09:00:00",
  "is_published": 1
}
```

---

## Contoh 2 — Draft: “Catatan Belakang Layar: Desain Database”

```json
{
  "user_id": 1,
  "title": "Catatan Belakang Layar: Desain Database",
  "slug": "catatan-desain-database",
  "excerpt": "Tulisan internal tentang pertimbangan desain ERD untuk proyek portfolio ini.",
  "content": "# Catatan Belakang Layar: Desain Database\n\nTulisan ini lebih bersifat teknis dan ditujukan untuk dokumentasi internal.\n\nBeberapa entitas utama:\n\n- `users`\n- `experiences`\n- `educations`\n- `projects`\n- `blog_posts`\n- `skills`, `tags`, dan tabel pivot terkait.\n\nPada tahap awal, beberapa field dibuat fleksibel (nullable) untuk memudahkan iterasi. Setelah kebutuhan stabil, constraint bisa diperketat.\n",
  "published_at": null,
  "is_published": 0
}
```

---

## Contoh 3 — Published: “Menulis Konten Teknis yang Nyaman Dibaca”

```json
{
  "user_id": 1,
  "title": "Menulis Konten Teknis yang Nyaman Dibaca",
  "slug": "menulis-konten-teknis-nyaman-dibaca",
  "excerpt": "Beberapa tips praktis untuk menulis konten teknis yang tetap ramah pembaca.",
  "content": "# Menulis Konten Teknis yang Nyaman Dibaca\n\nMenulis konten teknis untuk portfolio bukan hanya soal kode, tetapi juga **pengalaman membaca**.\n\nBeberapa tips singkat:\n\n- Bagi tulisan menjadi beberapa bagian dengan heading yang jelas.\n- Gunakan list bernomor untuk langkah-langkah.\n- Sertakan _code block_ seperlunya, misalnya:\n\n```bash\npodman compose up -d\n```\n\n- Tutup dengan ringkasan singkat tentang apa yang pembaca pelajari.\n",
  "published_at": "2026-02-10T14:30:00",
  "is_published": 1
}
```

---

## Contoh 4 — Published: “Checklist Sebelum Deploy ke Podman”

```json
{
  "user_id": 1,
  "title": "Checklist Sebelum Deploy ke Podman",
  "slug": "checklist-deploy-podman",
  "excerpt": "Daftar singkat hal yang perlu dicek sebelum menjalankan stack portfolio di Podman.",
  "content": "# Checklist Sebelum Deploy ke Podman\n\nSebelum menjalankan perintah:\n\n```bash\npodman compose up -d --build\n```\n\npastikan beberapa hal berikut sudah siap:\n\n1. File `compose.yaml` sudah mengarah ke folder yang benar (`portfolio-api`, `portfolio-web`, `portfolio-admin`).\n2. File `.env.podman` untuk API sudah dibuat dan **tidak di-commit** (masuk `.gitignore`).\n3. Healthcheck database sudah sesuai dengan environment (contoh: menggunakan `mysqladmin ping`).\n4. Port yang dipakai (8000, 3000, 3001, 3306) tidak bentrok dengan service lain.\n\nChecklist ini bisa kamu sesuaikan dengan kebutuhan lingkungan deployment masing-masing.\n",
  "published_at": "2026-02-15T08:15:00",
  "is_published": 1
}
```

---

## Cara Menggunakan

- **Sebagai referensi seeding**: salin objek JSON di atas ke seeder atau script import sesuai kebutuhan (misalnya di `database/seeders` untuk Lumen/Laravel).
- **Sebagai dokumentasi**: gunakan contoh ini untuk menjelaskan ke tim bagaimana bentuk data `blog-posts` yang ideal (baik untuk draft maupun published).

