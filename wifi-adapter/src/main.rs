#![no_std]
#![no_main]

use panic_halt as _;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;
//use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};
//use esp8266_hal::gpio::{Gpio16, Output, PushPull};
//use esp8266_hal::rtccntl::CrystalFrequency;
use core::str;

const UART_BASE : u32 = 0x60000000;
const UART_FIFO : *mut u8 = UART_BASE as *mut u8;
const UART_INT_CLR: *mut u16 = (UART_BASE + 0x10) as *mut u16;
const UART_CONF0: *mut u32 = (UART_BASE + 0x20) as *mut u32;
const UART_CONF1: *mut u32 = (UART_BASE + 0x24) as *mut u32;

fn uart_init() -> bool {

    unsafe { 
        let val = UART_CONF0.read_volatile();
        UART_CONF0.write_volatile(val | (1<<18) | (1<<17));

        UART_CONF0.write_volatile(val & !((1<<18) | (1<<17)));
    
        UART_CONF1.write_volatile((0x01 & 0x0000007F) | //UART_RXFIFO_FULL_THRHD
                                    ((0x01 & 0x0000007F) << 16) | // UART_RX_FLOW_THRHD
                                    (1<<23)); //UART_RX_FLOW_EN

        // Clear interrupt
        UART_INT_CLR.write_volatile(0xffff);
    };
    true
}


fn uart_writestring(input: &str) -> bool {
    unsafe {
        let _e = input.as_bytes()
            .iter()
            .for_each(|c| UART_FIFO.write_volatile(*c));            
    }
    true
}

fn uart_writenum(input: i32) -> bool {

    if input == 0 {
        unsafe { UART_FIFO.write_volatile('0' as u8); };
        return true;
    }
    // Find num of digits
    let mut divider = 1_000_000_000;
    let mut temp_in = input;
    while input / divider == 0 {
        divider = divider / 10;
    }

    unsafe {
        while divider > 0 {
          let out = '0' as i32 + (temp_in / divider);
          UART_FIFO.write_volatile(out as u8);
          temp_in -= (temp_in / divider)*divider;
          divider = divider / 10;
        }
    }
    true
}


#[entry]
fn main() -> ! {    
    let dp = Peripherals::take().unwrap();
    //dp.RTCCNTL.rtc_control().set_crystal_frequency(CrystalFrequency::Crystal40MHz);

    let pins = dp.GPIO.split();
    let mut led = pins.gpio16.into_push_pull_output();
    let (mut timer1, _) = dp.TIMER.timers();
    
    let mut _serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
    timer1.delay_ms(100);

    uart_init();

    let mut i = 0;

    loop {
        
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(1000);
        led.toggle().unwrap();


        uart_writestring("Loop..");
        uart_writenum(i);
        uart_writestring("\r\n");

        i = i+1;
    }
}
