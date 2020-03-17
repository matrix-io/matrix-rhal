#!/bin/bash

FIRMWARE=build/esp-app.bin

showhelp () {
  echo "---------------------------------------"
  echo "OTA-base installer (only for first use)"
  echo "---------------------------------------"
  echo ""
  echo "usage:"
  echo "./install.sh [RaspberryPi IP]"
  echo ""
  echo "example:"
  echo "./install.sh 192.168.1.10"
  echo ""
} 

if [ "$1" = "" ]; then
  showhelp
else
  if test -f "$FIRMWARE"; then
    echo ""
    #cp $FIRMWARE .
    echo "Loading firmware: $FIRMWARE"
    echo ""
    pushd build
    tar cf - *.bin bootloader/*.bin | ssh pi@$1 'tar xf - -C /tmp;sudo voice_esp32_reset;voice_esptool --chip esp32 --port /dev/ttyS0 --baud 115200 --before default_reset --after hard_reset write_flash -u --flash_mode dio --flash_freq 40m --flash_size detect 0x1000 /tmp/bootloader/bootloader.bin 0x10000 /tmp/esp-app.bin 0x8000 /tmp/partitions_singleapp.bin'
    popd
    echo "done"
    echo ""
    echo "[SUCCESS] Please disconnect your MatrixVoice from the RaspberryPi and reconnect it alone for future OTA updates."
    echo ""
  else
    echo "[ERROR] Please build firmware first!"
    exit 1
  fi

fi

exit 0

