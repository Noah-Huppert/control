package api

import (
	"context"
	"fmt"
	"net/http"

	"github.com/Noah-Huppert/control/server/config"

	"github.com/Noah-Huppert/golog"
	"github.com/gorilla/mux"
	"github.com/jmoiron/sqlx"
)

// NewServer creates a new http server which serves API requests
func NewServer(cfg config.Config, logger golog.Logger,
	db *sqlx.DB) http.Server {

	// Register handlers
	router := mux.NewRouter()

	routesLogger := logger.GetChild("routes")

	// ... Health check endpoint
	healthCheckHandler := NewHealthCheckHandler()
	router.Handle("/healthz", healthCheckHandler)

	// ... Register endpoint
	registerLogger := routesLogger.GetChild("register")
	registerHandler := NewRegisterHandler(registerLogger, db)

	router.Handle("/api/v0/register", registerHandler).Methods("POST")

	// Setup http server
	server := http.Server{
		Addr:    fmt.Sprintf(":%d", cfg.Port),
		Handler: router,
	}

	return server
}

// StartServer runs the http server
func StartServer(ctx context.Context, server http.Server) error {
	errChan := make(chan error)

	// Setup graceful shutdown
	go func() {
		<-ctx.Done()

		err := server.Shutdown(context.Background())
		if err != nil {
			errChan <- fmt.Errorf("error shutting down http server: %s",
				err.Error())
		} else {
			close(errChan)
		}
	}()

	// Start server
	go func() {
		err := server.ListenAndServe()
		if err != nil && err != http.ErrServerClosed {
			errChan <- fmt.Errorf("error running http server: %s", err.Error())
		}
	}()

	return <-errChan
}
