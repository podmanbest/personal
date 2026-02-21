# CI/CD & Deploy

## GitHub Actions

Workflow: **`.github/workflows/ci-cd.yml`**

### CI (setiap push & pull request ke `main` / `master`)

| Job | Isi |
|-----|-----|
| **API** | Checkout → Set up Go (cache) → `go build ./...` → `go test ./tests/...` |
| **Web** | Checkout → Set up Node (npm cache) → `npm ci` → `npm run build` → upload artifact `web-dist` |

Deploy **tidak** dijalankan pada pull request, hanya pada **push** ke `main`/`master`.

### CD (hanya pada push ke `main` / `master`)

| Job | Isi |
|-----|-----|
| **Deploy** | Setelah API + Web sukses → download artifact `web-dist` → deploy ke **GitHub Pages** |

### Mengaktifkan GitHub Pages

1. Repo → **Settings** → **Pages**.
2. Di **Source** pilih **GitHub Actions** (bukan branch).
3. Setelah workflow jalan sekali (push ke `main`), URL Pages biasanya:  
   `https://<username>.github.io/<repo>/`

### Base URL untuk Vite (opsional)

Jika situs di-host di subpath (mis. `https://user.github.io/personal/`), atur base di `web/vite.config.js`:

```js
export default defineConfig({
  base: '/personal/',  // ganti dengan nama repo
  // ...
})
```

Kalau repo Anda adalah **user.github.io** (situs utama), base bisa tetap `/`.

## Ringkasan

- **CI:** build + test API, build Web; jalan di setiap push dan PR.
- **CD:** deploy hasil build Web ke GitHub Pages; hanya saat push ke `main`/`master` dan setelah Pages di-set ke **GitHub Actions**.
