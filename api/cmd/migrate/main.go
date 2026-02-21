// Program migrasi database. Jalankan: go run ./cmd/migrate [--down]
package main

import (
	"context"
	"flag"
	"log"
	"os"
	"time"

	"github.com/joho/godotenv"
	"github.com/personal/api/internal/config"
	"github.com/personal/api/internal/database"
)

func main() {
	down := flag.Bool("down", false, "rollback satu migrasi terakhir")
	flag.Parse()

	_ = godotenv.Load()
	cfg := config.Load(time.Now().Unix())
	if cfg.DBDSN == "" {
		log.Fatal("DB_DSN tidak di-set. Isi di .env")
	}

	db, err := database.Open(cfg.DBDSN)
	if err != nil {
		log.Fatalf("Koneksi database gagal: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Minute)
	defer cancel()

	if *down {
		if err := database.MigrateDownSQL(ctx, db.DB); err != nil {
			log.Fatalf("Migrate down gagal: %v", err)
		}
		log.Print("Migrate down berhasil.")
		os.Exit(0)
	}

	if err := database.MigrateUpSQL(ctx, db.DB); err != nil {
		log.Fatalf("Migrate up gagal: %v", err)
	}
	log.Print("Migrate up berhasil.")
}
