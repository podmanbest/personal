package tests

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/personal/api/internal/database"
	"github.com/personal/api/internal/handlers"
)

func TestSkillsList_NoDB(t *testing.T) {
	handler := handlers.SkillsList(nil)
	req := httptest.NewRequest(http.MethodGet, "/api/skills", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusServiceUnavailable {
		t.Errorf("status = %d, want 503", rec.Code)
	}
}

func TestSkillsList_MethodNotAllowed(t *testing.T) {
	handler := handlers.SkillsList(&database.DB{})
	req := httptest.NewRequest(http.MethodPost, "/api/skills", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)
	if rec.Code != http.StatusMethodNotAllowed {
		t.Errorf("status = %d, want 405", rec.Code)
	}
}

func TestSkillsList_WithDB(t *testing.T) {
	handler := handlers.SkillsList(nil)
	req := httptest.NewRequest(http.MethodGet, "/api/skills", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)
	if rec.Code != http.StatusServiceUnavailable {
		t.Errorf("status = %d, want 503 when db nil", rec.Code)
	}
}
