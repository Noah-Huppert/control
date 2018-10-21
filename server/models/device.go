package models

import (
	"fmt"

	"github.com/jmoiron/sqlx"
)

// Device holds information about an IOT devices to be controlled
type Device struct {
	// ID unique database ID
	ID int64 `db:"id"`

	// PhysicalID identifies a specific device in the real world. Sent to the
	// server when a device registers.
	PhysicalID string `db:"physical_id"`

	// State is a value used to indicate how a device should be behaving
	State string `db:"state"`
}

// QueryByPhysicalID searches the database for a row with a matching
// physical id
func (d *Device) QueryByPhysicalID(db *sqlx.DB) error {
	return db.Get(d, "SELECT id FROM devices WHERE physical_id = $1",
		d.PhysicalID)
}

// Insert adds a database row using the Device struct fields. The inserted
// row's ID is saved in the struct
func (d *Device) Insert(db *sqlx.DB) error {
	// Begin transaction
	tx, err := db.Beginx()
	if err != nil {
		return fmt.Errorf("error beginning transaction: %s", err.Error())
	}

	// Insert
	err = tx.QueryRowx("INSERT INTO devices (physical_id, state) VALUES ($1, "+
		"$2) RETURNING id", d.PhysicalID, d.State).StructScan(d)
	if err != nil {
		return fmt.Errorf("error inserting row: %s", err.Error())
	}

	// Commit transaction
	err = tx.Commit()
	if err != nil {
		return fmt.Errorf("error committing transaction: %s", err.Error())
	}

	return nil
}

// SelectAllDevices queries the database for all the Devices
func SelectAllDevices(db *sqlx.DB) ([]Device, error) {
	devices := []Device{}

	err := db.Select(&devices, "SELECT * FROM devices")
	if err != nil {
		return nil, fmt.Errorf("error querying database: %s", err.Error())
	}

	return devices, nil
}
