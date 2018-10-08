package models

import (
	"github.com/jmoiron/sqlx"
)

// Device holds information about an IOT devices to be controlled
type Device struct {
	// ID unique database ID
	ID int64

	// PhysicalID identifies a specific device in the real world. Sent to the
	// server when a device registers.
	PhysicalID string

	// State is a value used to indicate how a device should be behaving
	State string
}

// QueryByPhysicalID searches the database for a row with a matching
// physical id
func (d *Device) QueryByPhysicalID(db *sqlx.DB) error {
	return db.Select(d, "SELECT id FROM devices WHERE physical_id = $1",
		d.PhysicalID)
}
