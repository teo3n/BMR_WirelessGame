#![no_std]
#![no_main]

use panic_halt as _;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
//use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};
//use esp8266_hal::gpio::{Gpio16, Output, PushPull};
//use esp8266_hal::rtccntl::CrystalFrequency;
//use core::str;
mod uart;
mod wifi;


#[entry]
fn main() -> ! {    
    let dp = Peripherals::take().unwrap();
    //dp.RTCCNTL.rtc_control().set_crystal_frequency(CrystalFrequency::Crystal40MHz);

    let pins = dp.GPIO.split();
    let mut led = pins.gpio16.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();
    
    let mut _serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
    timer1.delay_ms(100);

    uart::init();
    wifi::init();

    wifi::connect("", "");

    let mut i = -5;

    loop {
        
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(1000);
        led.toggle().unwrap();


        uart::writestring("Loop..");
        uart::writenum(i);
        uart::writestring("\r\n");

        i = i+1;
    }
}
