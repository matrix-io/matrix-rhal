#!/usr/bin/env bash

# Default image name in `Makefile`, found in `build/` after `make menuconfig && make`
FIRMWARE=${FIRMWARE-esp-app.bin}

exit_value=0
if [ "$1" = "" ]; then
  echo ""
  echo "usage:"
  echo "./install.sh [RaspberryPi IP]"
  echo ""
  echo "example:"
  echo "./install.sh 192.168.1.10"
  echo ""
else
  pushd build
  if test -f "$FIRMWARE"; then
    echo ""
    echo "Loading firmware: $FIRMWARE"
    echo ""
    tar cf - *.bin bootloader/*.bin | ssh pi@$1 "tar xf - -C /tmp;sudo voice_esp32_reset;voice_esptool --chip esp32 --port /dev/ttyS0 --baud 115200 --before default_reset --after hard_reset write_flash -u --flash_mode dio --flash_freq 40m --flash_size detect 0x1000 /tmp/bootloader/bootloader.bin 0x10000 /tmp/$FIRMWARE 0x8000 /tmp/partitions_singleapp.bin"
    echo "done"
    echo ""
    echo "[SUCCESS] Please disconnect your MatrixVoice from the RaspberryPi and reconnect it alone for future OTA updates."
    echo ""
  else
    echo "[ERROR] Please build firmware first!"
    exit_value=1
  fi
  popd
fi

exit $exit_value
