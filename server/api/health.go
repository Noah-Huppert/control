package api

import (
	"net/http"

	"github.com/Noah-Huppert/control/server/resp"
)

// HealthCheckHandler handles health check requests
type HealthCheckHandler struct{}

// NewHealthCheckHandler creates a new http handler using a HealthCheckHandler
func NewHealthCheckHandler() http.Handler {
	return resp.WrapResponderHandler(HealthCheckHandler{})
}

// Handle implements ResponderHandler.Handle
func (h HealthCheckHandler) Handle(r *http.Request) resp.Responder {
	return resp.NewJSONResponder(
		map[string]bool{
			"ok": true,
		},
		http.StatusOK,
	)
}
