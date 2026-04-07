package main

import (
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

func isIndonesia(r *http.Request) bool {
	country := strings.ToUpper(strings.TrimSpace(
		firstNonEmpty(
			r.Header.Get("x-vercel-ip-country"),
			r.Header.Get("cf-ipcountry"),
			r.Header.Get("x-country-code"),
		),
	))
	if country == "ID" {
		return true
	}

	acceptLanguage := strings.ToLower(r.Header.Get("accept-language"))
	for _, part := range strings.Split(acceptLanguage, ",") {
		lang := strings.TrimSpace(strings.Split(part, ";")[0])
		if lang == "id" || strings.HasPrefix(lang, "id-") {
			return true
		}
	}
	return false
}

func firstNonEmpty(values ...string) string {
	for _, v := range values {
		if strings.TrimSpace(v) != "" {
			return v
		}
	}
	return ""
}

func rootHandler(w http.ResponseWriter, r *http.Request) {
	if isIndonesia(r) {
		http.Redirect(w, r, "/id", http.StatusTemporaryRedirect)
		return
	}

	indexPath := filepath.Join("dist", "index.html")
	http.ServeFile(w, r, indexPath)
}

func main() {
	port := os.Getenv("PORT")
	if port == "" {
		port = "8081"
	}

	fileServer := http.FileServer(http.Dir("dist"))

	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/" {
			rootHandler(w, r)
			return
		}
		fileServer.ServeHTTP(w, r)
	})

	addr := "0.0.0.0:" + port
	log.Printf("listening on %s", addr)
	if err := http.ListenAndServe(addr, handler); err != nil {
		log.Fatalf("server exited with error: %v", err)
	}
}
