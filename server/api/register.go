package api

import (
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"

	"github.com/Noah-Huppert/control/server/models"
	"github.com/Noah-Huppert/control/server/resp"

	"github.com/Noah-Huppert/golog"
	"github.com/jmoiron/sqlx"
	"gopkg.in/validator.v2"
)

// RegisterRequest is the data a client should send in a register request body
type RegisterRequest struct {
	// PhysicalID is the physical ID of the device being registered
	PhysicalID string `json:"physical_id" validate:"nonzero"`

	// DefaultState is the starting state for the device
	DefaultState string `json:"default_state" validate:"nonzero"`
}

// RegisterHandler handles device register requests
type RegisterHandler struct {
	// logger is used output debug information
	logger golog.Logger

	// db is a database connection
	db *sqlx.DB
}

// NewRegisterHandle creates a new http.Handler from a RegisterHandler
func NewRegisterHandler(logger golog.Logger, db *sqlx.DB) http.Handler {
	return resp.WrapResponderHandler(RegisterHandler{
		logger: logger,
		db:     db,
	})
}

// Handle implements ResponderHandler.Handle
func (h RegisterHandler) Handle(r *http.Request) resp.Responder {
	// Read request body
	decoder := json.NewDecoder(r.Body)
	var req RegisterRequest

	err := decoder.Decode(&req)
	if err != nil {
		err = fmt.Errorf("error decoding request body: %s", err.Error())
		pubErr := errors.New("error reading request body")

		return resp.NewErrorResponder(h.logger, http.StatusInternalServerError,
			pubErr, err)
	}

	err = validator.Validate(req)
	if err != nil {
		err = fmt.Errorf("error validating request body: %s", err.Error())

		return resp.NewErrorResponder(h.logger, http.StatusBadRequest,
			err, err)
	}

	// Find device with physical id
	device := &models.Device{
		PhysicalID: req.PhysicalID,
	}

	err = device.QueryByPhysicalID(h.db)
	if err == sql.ErrNoRows { // Insert device if not found
		device.State = req.DefaultState

		err = device.Insert(h.db)

		if err != nil {
			err = fmt.Errorf("error inserting new device: %s", err.Error())
			pubErr := errors.New("error inserting new device model")

			return resp.NewErrorResponder(h.logger,
				http.StatusInternalServerError, pubErr, err)
		}
	} else if err != nil { // Error querying for device
		err = fmt.Errorf("error querying for device: %s", err.Error())
		pubErr := errors.New("error querying for existing device")

		return resp.NewErrorResponder(h.logger,
			http.StatusInternalServerError, pubErr, err)
	}

	return resp.NewJSONResponder(
		map[string]interface{}{
			"device": device,
		},
		http.StatusOK,
	)
}
