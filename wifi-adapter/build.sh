#!/bin/bash
xargo build --release
esptool -c esp8266 elf2image ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266
cp ./target/xtensa-esp8266-none-elf/release/bmr-wireless-game-esp8266-* .
