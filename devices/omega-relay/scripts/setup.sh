#!/usr/bin/env bash
export ip=$(nmap 192.168.1.100-149 -p 22 --open -n -oG - | sed '/^#/ d' | awk '{ print $2 }' | uniq)
