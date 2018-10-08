.PHONY: build serial

SERIAL_PORT=/dev/ttyUSB0
SERIAL_BAUD=115200

# build cross compiles source code for the Omega
build:
	cargo build --target=mipsel-unknown-linux-musl

# serial connects to the Omega over serial
serial:
	screen "${SERIAL_PORT}" "${SERIAL_BAUD}"
