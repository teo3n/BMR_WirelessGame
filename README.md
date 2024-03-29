# BearMetalRust WirelessGame

This project was made as a part of Tampere University's course, Bare Metal Rust, where Rust code was cross-compiled to some bare metal platform. This is a wireless multiplayer game, where the networking is handled by a separate ESP8266 module. 

</br>

how to run the project, tested on pop-os 20.04: 
1. install dfu-util from source ```git clone git://git.code.sf.net/p/dfu-util/dfu-util``` and follow the instructions
2. connect the longang nano and put it into boot mode (press and hold boot0 -button, click reset -button once)
3. run the automated build-and-flash -script  ```./flash_and_build.sh```
	- note: requires sudo
4. the terminal should now read ```File downloaded successfully``` and ```dfu-util: Error during download get_status.``` This is correct, power-cycle the device (or press reset) to execute the program.

<br/> The example shows a ferris on the lcd screen ands blinks the on-board rgb led (cycles through the colors individually)<br/>

Add your name and email to Cargo.toml's authors -field. <br/>


# IMPORTANT!

requires rust nightly to run, due to inline assembly. <br/>

```
rustup target add riscv32imac-unknown-none-elf --toolchain nightly
cargo +nightly build ...
```

