#!/usr/bin/env bash
#?
# build.sh - Builds omega relay
#
# USAGE
#
#	build.sh [BUILD_ARGS...]
#
# ARGUMENTS
#
#	BUILD_ARGS    Optional extra gcc arguments
#
# ENVIRONMENT VARIABLES
#
#	HOST_OUT_DIR    Directory to output binary
#	OUT_FILE        Name of binary
#   SRC             Source files
#
# BEHAVIOR
#
#	Cross compiles omega relay binary for OpenWRT.
#?
export STAGING_DIR=openwrt/staging_dir/toolchain-mips_24kc_gcc-7.3.0_musl

if [ -z "$GCC" ]; then
	GCC=${STAGING_DIR}/bin/mips-openwrt-linux-musl-gcc
fi

mkdir -p $HOST_OUT_DIR

$GCC \
	-o $HOST_OUT_DIR/$OUT_FILE \
	$@ \
	$SRC

