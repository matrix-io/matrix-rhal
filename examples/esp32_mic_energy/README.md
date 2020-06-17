
__NOTE__: Some of the mic-related structs are relatively large.  If you get a lot of `***ERROR*** A stack overflow in task main has been detected`, you may need to increase the size of the stack: `menuconfig > Component config > Common ESP-related > Main task stack size`.

## Installation

First time setup:
```sh
# Setup toolchain for ESP32/xtensa
git clone https://github.com/jeikabu/rust-esp-container
docker build -t rust-esp ./rust-esp-container # Wait an hour or two
```

## Project Setup

```sh
cd <PROJECT_PATH>
docker run --rm -it -v $(pwd):/home/project rust-esp /bin/bash
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