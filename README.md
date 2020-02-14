> TODO: polish README.md

# MATRIX Rust Hardware Abstraction Layer

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
- GPIO [ ]
- Microphone [ ]

# Dependencies

All of this is expected to run on a Raspberry pi with the following installed:

- MATRIX Init Package
- MATRIX Kernel Modules
- Rust installed

# Installation

```bash
git clone https://github.com/Hermitter/matrix-rhal
cd matrix-rhal
cargo run
```
