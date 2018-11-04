#!/usr/bin/env bash
#?
# get-ip.sh - Scans ethernet for another device with port 22 open
#
# USAGE
#	./get-ip.sh
#
# BEHAVIOR
#
#	Searches the ethernet space for a device with port 22 open. Assumes the 
#	space only has one device connected to it.
#?
export ip=$(nmap 192.168.1.100-149 -p 22 --open -n -oG - | sed '/^#/ d' | awk '{ print $2 }' | uniq)

if [ -z "$ip" ]; then
	echo "Failed to find device" >&2
	exit 1
fi

echo "Found device with ip: $ip"
