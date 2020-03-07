# MATRIX Rust Hardware Abstraction Layer

[![API](https://docs.rs/matrix_rhal/badge.svg)](https://docs.rs/matrix_rhal/)
[![Crate](https://img.shields.io/crates/v/matrix-rhal.svg)](https://crates.io/crates/matrix_rhal)
![crates.io Deployment](https://github.com/matrix-io/matrix-rhal/workflows/crates.io%20Deployment/badge.svg)

MATRIX RHAL is the Rust implementation of [MATRIX HAL](https://github.com/matrix-io/matrix-creator-hal). The goal is to center around calling the MATRIX Kernel Modules and eventually implement all of HAL's features in this layer.

# Usage

> Breaking changes are going to be really common until `v0.1.0`

Add this to your `Cargo.toml`:

```toml
[dependencies]
matrix_rhal = "0.0.5"
```

# Roadmap

Below are the current/planned features in RHAL. While this library is being put together, updates will be pushed to crates.io as `v0.0.*` for users to try.

- [x] Device information
- [x] Sensor Reading
  - [x] UV
  - [x] Pressure
  - [x] Humidity
  - [x] IMU
- [x] Everloop
- [x] GPIO
  - [x] Digital Read/Write
  - [x] PWM Write
- [ ] Microphone
- [ ] Documentation

# Dependencies

Run the following commands individually on your Raspberry Pi.

```bash
# Add matrix repository & key
curl https://apt.matrix.one/doc/apt-key.gpg | sudo apt-key add -
echo "deb https://apt.matrix.one/raspbian $(lsb_release -sc) main" | sudo tee /etc/apt/sources.list.d/matrixlabs.list

# Update packages
sudo apt-get update
sudo apt-get upgrade

# Install MATRIX Init Package
sudo apt-get install matrixio-creator-init

# Restart system
sudo reboot

# Install MATRIX kernel Modules
sudo apt install matrixio-kernel-modules

# Restart a second time
sudo reboot
```

# Building From Source (Raspberry Pi)

Building directly on your Raspberry Pi will lead to slower compilation times, due to the lack processing power.

Install git.

```bash
sudo apt install git
```

Clone the repository.

```bash
git clone https://github.com/Hermitter/matrix-rhal
```

Install Rust on the pi.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Add MATRIX RHAL as a dependency to your Rust project.

- [Cargo Path Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies)

# Building From Source (Cross Compile)

To reduce compilation times, it's recommended to build RHAL on your computer and deploy it to the Pi. This ends up saving time and sanity during development. Below are some guides to help set up this workflow:

- [Docker](https://github.com/rust-embedded/cross)
- [Linux](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi)
- [Windows & macOS](https://dev.to/h_ajsf/cross-compiling-rust-for-raspberry-pi-4iai)
