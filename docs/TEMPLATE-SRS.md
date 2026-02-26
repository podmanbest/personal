## Software Requirements Specification (SRS)

### Kerangka Dokumen Spesifikasi Kebutuhan Perangkat Lunak (SRS) yang mengacu pada standar IEEE 830 / ISO/IEC/IEEE 29148. Ini adalah standar emas untuk dokumentasi RPL.

Proyek: [Nama Aplikasi Web/Mobile Anda]

Versi: 1.0

Tanggal: 26 Februari 2026

---

1.  Pendahuluan

    1.1 Tujuan: Menjelaskan maksud pembuatan dokumen ini dan target audiensnya (developer, tester, atau client).

    1.2 Cakupan Produk: Penjelasan singkat apakah ini aplikasi Web, Mobile (Android/iOS), atau Hybrid, serta manfaat utamanya bagi pengguna.

    1.3 Definisi dan Akronim: Daftar istilah teknis (misal: API, Firebase, UI/UX, CRUD).

2.  Deskripsi Umum

    2.1 Perspektif Produk: Apakah aplikasi ini berdiri sendiri atau merupakan bagian dari sistem yang lebih besar (misalnya: modul e-commerce dari sistem ERP)?

    2.2 Fitur Produk: Ringkasan fungsi utama (misal: Login, Pembayaran, Chat Real-time).

    2.3 Karakteristik Pengguna: Siapa penggunanya? (Admin, User Umum, Merchant).

    2.4 Batasan-batasan: Kendala teknis, misalnya: "Hanya berjalan di Android versi 10 ke atas" atau "Membutuhkan koneksi internet stabil".

3.  Kebutuhan Sistem (System Requirements)

    Ini adalah bagian paling teknis dan krusial dalam dokumen RPL.

    3.1 Kebutuhan Fungsional (Functional Requirements)

    Menjelaskan apa yang dilakukan sistem. Biasanya ditulis dengan format: "Sistem harus dapat..."

        - FR-01: Sistem harus memungkinkan pengguna mendaftar menggunakan email.
        - FR-02: Sistem harus dapat mengirim notifikasi push saat ada pesan masuk.
        - FR-03: (Web) Sistem harus memiliki dashboard analitik untuk Admin.

    3.2 Kebutuhan Non-Fungsional (Non-Functional Requirements)

    Menjelaskan bagaimana sistem bekerja (Kualitas berdasarkan ISO 2510).

        - Performa: Waktu pemuatan halaman maksimal 3 detik.
        - Keamanan: Password harus dienkripsi menggunakan algoritma AES/bcrypt.
        - Ketersediaan: Sistem harus aktif 99.9% (High Availability).
        - Scalability: Aplikasi mampu menangani hingga 1.000 pengguna bersamaan.

4.  Arsitektur dan Desain Teknis

    4.1 Stack Teknologi: \* Frontend: React.js (Web) atau Flutter (Mobile).

        - Backend: Node.js / Go.
        - Database: PostgreSQL / MongoDB.

    4.2 Diagram Arsitektur: (Biasanya berupa bagan alur data/DFD atau Unified Modeling Language/UML).

    Tabel Matriks Kebutuhan
    | ID | Kebutuhan | Proritas | Status |
    | - | - | - | - |
    | FR-01 | Fitur Login & Autentikasi | Tinggi | Wajib |
    | FR-02 | Integrasi Payment Gateway | Tinggi | Wajib |
    | NFR-01 | Enkripsi Data End-to-End | Sedang | Penting|
