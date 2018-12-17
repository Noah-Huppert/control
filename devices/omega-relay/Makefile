.PHONY: build scp ssh serial ssh-copy-id

# Variables
# ... SSH
SSH_KEY=${HOME}/.ssh/omega_key

# ... C
SRC=src/main.c src/gpio.c
COMPILE_ARGS=-g

HOST_OUT_DIR=build
OUT_FILE=omega-relay

# ... Omega
#GUEST_SRC_DIR=/mnt/omega-storage
GUEST_SRC_DIR=/root

# ... Serial
SERIAL_PORT=/dev/ttyUSB0
SERIAL_BAUD=115200

# build cross compiles source code for the Omega
build: ${SRC}
	HOST_OUT_DIR=${HOST_OUT_DIR} \
	OUT_FILE=${OUT_FILE} \
	SRC="${SRC}" \
	./scripts/build.sh \
	${COMPILE_ARGS}

# scp transfers the built binary using scp to the omega
# args:
#	- ip: omega ip
scp:
	if [ -z "${ip}" ]; then exit 1; fi
	scp -i "${SSH_KEY}" "${HOST_OUT_DIR}/${OUT_FILE}" "root@${ip}:${GUEST_SRC_DIR}"

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
