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

pub fn ets_printf() -> bool  {
    true
}

#[entry]
//#[no_mangle]
//#[link(name="user_init")]
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

    uart::writestring("Connecting Wifi\r\n");
    wifi::connect("", "");

    let mut i = -5;


    let mut connected = false;

    while !connected {
        uart::writestring("Check connection\r\n");
        connected = wifi::is_connected();
        timer1.delay_ms(500);
    }
    let ip = wifi::get_ip();

    uart::writestring("Wifi connected!\r\n");
    uart::writenum((ip >> 24) as i32);
    uart::writestring(".");
    uart::writenum((ip >> 16 & 0xff) as i32);
    uart::writestring(".");
    uart::writenum((ip >> 8 & 0xff) as i32);
    uart::writestring(".");
    uart::writenum((ip & 0xff) as i32);
    uart::writestring("\r\n");

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
