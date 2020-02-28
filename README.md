# MATRIX Rust Hardware Abstraction Layer

[![](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg?longCache=true)](https://crates.io/crates/matrix_rhal)

MATRIX RHAL is the Rust implementation of MATRIX HAL. The goal is to center around calling the MATRIX Kernel Modules and have most/all of HAL rewritten in this layer.

# Roadmap

This will be a bare minimum roadmap on what features are working. It's safe to assume that some of these will require further calibration and testing.

- [x] Device information
- [x] Sensor Reading
  - [x] UV
  - [x] Pressure
  - [x] Humidity
  - [x] IMU
- [x] Everloop
- [x] GPIO
  - [x]Digital Read/Write
  - [ ] PWM Write
- [ ] Microphone

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

Building directly on your Raspberry Pi will cause some delay in compilation times, due to the processing power available.

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

Compiling this project from your personal computer saves a lot of time during development. Below are some guides to help set up this workflow using:

- [Docker](https://github.com/rust-embedded/cross)
- [Linux](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi)
- [Windows & macOS](https://dev.to/h_ajsf/cross-compiling-rust-for-raspberry-pi-4iai)
