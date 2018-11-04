.PHONY: build scp ssh serial ssh-copy-id

SSH_KEY=${HOME}/.ssh/omega_key

BUILD_TARGET=mips-unknown-linux-musl

OUT_FILE=omega-relay
HOST_SRC_DIR=target/${BUILD_TARGET}/release
GUEST_SRC_DIR=/mnt/omega-storage

SERIAL_PORT=/dev/ttyUSB0
SERIAL_BAUD=115200

# build cross compiles source code for the Omega
build:
	cargo build --release --target="${BUILD_TARGET}"

# scp transfers the built binary using scp to the omega
# args:
#	- ip: omega ip
scp:
	if [ -z "${ip}" ]; then exit 1; fi
	scp -i "${SSH_KEY}" "${HOST_SRC_DIR}/${OUT_FILE}" "root@${ip}:${GUEST_SRC_DIR}"

# ssh opens an ssh connection to the omega
# args:
#	- ip: omega ip
ssh:
	if [ -z "${ip}" ]; then exit 1; fi
	ssh -i "${SSH_KEY}" "root@${ip}"

# serial connects to the Omega over serial
serial:
	screen "${SERIAL_PORT}" "${SERIAL_BAUD}"

# ssh-copy-id copies your SSH public key to the Omega
ssh-copy-id:
	./scripts/ssh-copy-id.sh
