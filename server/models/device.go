package models

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
