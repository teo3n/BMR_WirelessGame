# BearMetalRust WirelessGame

how to run the project, tested on pop-os 20.04: 
1. install dfu-util from source ```git clone git://git.code.sf.net/p/dfu-util/dfu-util``` and follow the instructions
2. connect the longang nano and put it into boot mode (press and hold boot0 -button, click reset -button once)
3. run the automated build-and-flash -script  ```./flash_and_build.sh```
	- note: requires sudo
4. the terminal should now read ```File downloaded successfully``` and ```dfu-util: Error during download get_status.``` This is correct, power-cycle the device (or press reset) to execute the program.

<br/> The example shows a ferris on the lcd screen ands blinks the on-board rgb led (cycles through the colors individually)<br/>

Add your name and email to Cargo.toml's authors -field. <br/>

