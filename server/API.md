# Server API
Control API.

# Table Of Contents
- [Overview](#overview)
- [Health Check](#health-check)
- [Register](#register)

# Overview
API endpoint documentation.  

All request and response data is JSON encoded.

# Health Check
`GET /healthz`

Request: None

Response:

- `ok` (boolean)

# Register
`POST /api/v0/register`  

Register a device to control.

Request body:

- `physical_id` (String): Unique human readable name of device
- `default_state` (String): State in which device starts

Response:

- `device` ([Device](https://godoc.org/github.com/Noah-Huppert/control/server/models#Device))
