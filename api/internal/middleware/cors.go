package middleware

import (
	"log"
	"net/http"
	"os"
	"strings"
)

const (
	envAllowOrigin = "ALLOW_ORIGIN"
	headerOrigin   = "Access-Control-Allow-Origin"
	headerMethods  = "Access-Control-Allow-Methods"
	headerHeaders  = "Access-Control-Allow-Headers"
	corsMethods    = "GET, POST, PUT, DELETE, OPTIONS"
	corsHeaders    = "Content-Type, Authorization"
)

// CORS adds Access-Control-Allow-* headers when ALLOW_ORIGIN is set.
// Nilai "*" tidak dipakai: di production wajib set origin spesifik (mis. https://namaanda.com).
// Jika ALLOW_ORIGIN kosong atau "*", header CORS tidak diset (hanya same-origin).
func CORS(next http.Handler) http.Handler {
	allowOrigin := strings.TrimSpace(os.Getenv(envAllowOrigin))
	if allowOrigin == "*" {
		log.Print("cors: ALLOW_ORIGIN=* tidak aman untuk production; header CORS tidak diset. Gunakan origin spesifik (mis. https://namaanda.com).")
		allowOrigin = ""
	}
	origin := allowOrigin
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if origin != "" {
			w.Header().Set(headerOrigin, origin)
			w.Header().Set(headerMethods, corsMethods)
			w.Header().Set(headerHeaders, corsHeaders)
			if r.Method == http.MethodOptions {
				w.WriteHeader(http.StatusNoContent)
				return
			}
		}
		next.ServeHTTP(w, r)
	})
}
