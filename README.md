# Raspberry Pi Setup

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Reload Terminal Session

```
source ~/.profile
```

## (Optional) Target Pi From Computer

```bash
rustup target add armv7-unknown-linux-gnueabihf

sudo dnf install arm-none-eabi-gcc-cs arm-none-eabi-gcc-cs-c++ arm-none-eabi-binutils-cs
sudo dnf install qemu-user gcc-c++-arm-linux-gnu
```
