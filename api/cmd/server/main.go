// Package main runs the API HTTP server.
package main

import (
	"log"
	"net/http"
	"strconv"
	"time"

	"github.com/joho/godotenv"
	"github.com/personal/api/internal/config"
	"github.com/personal/api/internal/database"
	"github.com/personal/api/internal/handlers"
	"github.com/personal/api/internal/middleware"
)

func main() {
	// Load .env dari folder api/ atau api/configs/
	_ = godotenv.Load()
	_ = godotenv.Load("configs/.env")
	startTime := time.Now().Unix()
	cfg := config.Load(startTime)

	var db *database.DB
	if cfg.DBDSN != "" {
		var err error
		db, err = database.Open(cfg.DBDSN)
		if err != nil {
			log.Printf("database: koneksi gagal (server jalan tanpa DB): %v", err)
			log.Print("database: pastikan MySQL/MariaDB jalan, DB_DSN benar di configs/.env")
			db = nil
		} else {
			log.Print("database: terhubung")
			defer db.Close()
		}
	}

	mux := http.NewServeMux()
	mux.HandleFunc("/health", handlers.Health)
	mux.HandleFunc("/status", handlers.Status(cfg.StartTime, db))
	mux.HandleFunc("/api/skills", handlers.SkillsList(db))
	mux.HandleFunc("/login", handlers.Login(cfg))
	mux.Handle("/admin", middleware.RequireAuth(cfg, handlers.Admin))

	addr := ":" + strconv.Itoa(cfg.Port)

	handler := middleware.CORS(middleware.SecurityHeaders(mux))
	log.Printf("listening on %s", addr)
	log.Fatal(http.ListenAndServe(addr, handler))
}
