# Server API
Control API.

# Table Of Contents
- [Register](#register)

# Register
`POST /api/v0/register`  

Register a device to control.

Request body:

- `physical_id` (String): Unique human readable name of device
- `default_state` (String): State in which device starts

Response:

- `device` ([Device](https://godoc.org/github.com/Noah-Huppert/control/server/models#Device))
