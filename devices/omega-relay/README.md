# Omega Relay
Relay controlled by an Omega v1.  

# Table Of Contents
- [Overview](#overview)
- [Setup](#setup)
- [Development](#development)

# Overview
A control device which switches a relay using the Omega v1. 

# Setup
Build the Omega toolchain, this will take some time:

```
git submodules init --update
cd omega-toolchain
make toolchain/install
```

Add the toolchain to Rust:

```
rustup target add mipsel-unknown-linux-musl
```

# Development
Cross compile the source code for the Omega v1 by running:

```
make build
# or
make
```
