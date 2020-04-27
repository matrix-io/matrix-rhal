
```sh
# Pull docker image containing LLVM and Rust with xtensa support
docker run --rm -it -v $(pwd):/home/matrix-io quay.io/ctron/rust-esp /bin/bash

# >>>Inside docker container

cd /home/matrix-io/examples/esp32_everloop

# Create dummy app_main stub
mkdir main
echo "void app_main() {}" > main/esp_app_main.c
touch main/component.mk

# Build ESP-IDF
make menuconfig
make -j4

# Build Rust binary: `target/xtensa-esp32-none-elf/release/esp32_sample`
# Using `[build].target` from `.cargo/config`:
cargo +xtensa xbuild --release
# OR, the more explicit (and longer):
cargo +xtensa xbuild --target "${XARGO_TARGET:-xtensa-esp32-none-elf}" --release


# Replace `build/esp-app.bin` with Rust binary
"${IDF_PATH}/components/esptool_py/esptool/esptool.py" \
    --chip esp32 \
    elf2image \
    -o build/esp-app.bin \
    target/xtensa-esp32-none-elf/release/esp32_everloop

# Push to Matrix Voice via ssh to Raspberry Pi `pi@pi.local`
./install.sh pi.local
```