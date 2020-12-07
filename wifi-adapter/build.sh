#!/bin/bash
xargo build --release
#xtensa-lx106-elf-strip --strip-all ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266
esptool -c esp8266 elf2image ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266
#xtensa-lx106-elf-nm -av ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266 | uniq -u | grep "^4010"
#xtensa-lx106-elf-objdump -d /mnt/g/projektit/ESP8266/Espressif_toolchain/ESP8266_NONOS_SDK/lib/libmain.a | awk -v RS= '/^[[:xdigit:]]+ <wifi_set_opmode>/'
cp ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266-* .
