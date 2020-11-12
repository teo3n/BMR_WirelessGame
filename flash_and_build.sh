cargo build --release --all-features
rm bmr_wiregame.bin
./riscv-nuclei-elf-objcopy -O binary target/riscv32imac-unknown-none-elf/release/bmr_wiregame bmr_wiregame.bin
sudo dfu-util -a 0 -s 0x08000000:leave -D bmr_wiregame.bin
