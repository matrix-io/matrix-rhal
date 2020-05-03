[Cargo generate](https://github.com/ashleygwilliams/cargo-generate) template to simplify developing Rust ESP32 binaries using ESP-IDF via [esp-idf-sys](https://github.com/sapir/esp-idf-sys).

## Installation

First time setup:
```sh
cargo install cargo-generate

# Setup toolchain for ESP32/xtensa
git clone https://github.com/jeikabu/rust-esp-container
docker build -t rust-esp ./rust-esp-container # Wait an hour or two
```

## Project Setup

```sh
# Create new Rust project from template
cargo generate -f --name PROJECT_NAME --git https://github.com/jeikabu/esp_idf_template.git --branch v4.0
# OR, from local repo:
cargo generate --name PROJECT_NAME --git esp_idf_template --branch v4.0

docker run --rm -it -v $(pwd)/PROJECT_NAME:/home/project rust-esp /bin/bash
```

```sh
source $IDF_PATH/export.sh

# Generate `build/` binaries
idf.py build
```

Easy way:
```sh
quick-build
```

Hard way:
```sh
# Build Rust binary: `target/xtensa-esp32-none-elf/`
# Using `[build].target` from `.cargo/config`:
cargo +xtensa xbuild
# OR, the more explicit (and longer):
cargo +xtensa xbuild --target "${XARGO_TARGET:-xtensa-esp32-none-elf}"


# Replace `build/esp-app.bin` with Rust binary
"${IDF_PATH}/components/esptool_py/esptool/esptool.py" \
    --chip esp32 \
    elf2image \
    -o build/esp-app.bin \
    target/xtensa-esp32-none-elf/CONFIG/PROJECT_NAME
```

```sh
# Push to Matrix Voice via ssh to Raspberry Pi `pi@pi.local`
./install.sh pi.local
```