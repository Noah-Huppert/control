# Omega Relay
Relay controlled by an Omega v1.  

# Table Of Contents
- [Overview](#overview)
- [Setup](#setup)
- [Development](#development)

# Overview
A control device which switches a relay using the Omega v1. 

# Setup
The OpenWRT toolchain must be setup in order to cross compile software for
the Omega.

1. Clone OpenWRT repository
   ```
   git submodule init --update
   ```
2. Copy the build configuration
   ```
   cp openwrt-files/.config openwrt
   ```
3. Enter the OpenWRT repository
   ```
   cd openwrt
   ```
   All following commands should be executed in this directory.
4. Add Omega packages to OpenWRT
   ```
   echo 'src-git onion https://github.com/OnionIoT/OpenWRT-Packages.git' >> feeds.conf.default
   scripts/feeds update -a
   ```
5. Build the toolchain
    ```
    make toolchain/install -j4
    ```
	This may take some time.  
	The `-j4` option makes the build tools use 4 cores for compilation. Change 
	this option as needed.
6. Copy toolchain library files to the Omega
   ```
   scp staging_dir/toolchain-mips_24kc_gcc-7.3.0_musl/lib/ld-musl-mips-sf.so.1 root@<ip>:/lib
   ```
7. Add the toolchain to Rust:
   ```
   rustup target add mips-unknown-linux-musl
   ```

# Development
Cross compile the source code for the Omega v1 by running:

```
make build
```

Upload the built executable to the Omega by running:

```
make scp
```
