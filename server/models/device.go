package models

// Device holds information about an IOT devices to be controlled
type Device struct {
	// ID unique database ID
	ID int64

	// State is a value used to indicate how a device should be behaving
	State string
}
