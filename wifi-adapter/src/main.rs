#![no_std]
#![no_main]

#[link(name = "c")]
#[link(name = "gcc")]
#[link(name = "pp")]
#[link(name = "phy")]
#[link(name = "net80211")]
#[link(name = "lwip")]
#[link(name = "wpa")]
#[link(name = "wpa2")]
#[link(name = "main")]
#[link(name = "crypto")]

#[allow(unused_mut)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}

const GPIO_BASE : u32 = 0x60000300;
const GPIO_OUT_W1TS_ADDRESS :*mut u32 = (GPIO_BASE+0x04) as *mut u32;
const GPIO_OUT_W1TC_ADDRESS :*mut u32 = (GPIO_BASE+0x08) as *mut u32;


extern "C" {
    pub fn Uart_init() -> u8;
    pub fn ets_delay_us(time: u32) -> u8;
    pub fn gpio_output_set(set_mask:u32,clear_mask:u32,enable_mask:u32,disable_mask:u32);
}

const PAD_XPD_DCDC_CONF: *mut u32  =  (0x60000700 + 0x0A0) as *mut u32;
const RTC_GPIO_CONF: *mut u32      = (0x60000700 + 0x090) as *mut u32;
const RTC_GPIO_ENABLE: *mut u32    = (0x60000700 + 0x074) as *mut u32;
const RTC_GPIO_OUT: *mut u32       = (0x60000700 + 0x068) as *mut u32;


fn read_peri_reg(addr:*mut u32) -> u32 {
    unsafe {
    return addr.read_volatile();
    };
}
fn write_peri_reg(addr:*mut u32, val: u32) { 
    unsafe { addr.write_volatile(val); };
}
fn clear_peri_reg_mask(reg:*mut u32, mask: u32) {
    write_peri_reg((reg), read_peri_reg(reg)&(!(mask)))
}
fn set_peri_reg_mask(reg:*mut u32, mask: u32)   {
    write_peri_reg((reg), read_peri_reg(reg)|(mask))
}

const PERIPHS_IO_MUX_PULLUP:u32 =  0x00000080;

fn pin_pullup_dis(pin:*mut u32) {
    clear_peri_reg_mask(pin, PERIPHS_IO_MUX_PULLUP)
}  
fn pin_pullup_en(pin:*mut u32) {
    set_peri_reg_mask(pin, PERIPHS_IO_MUX_PULLUP);
}
const PERIPHS_IO_MUX:*mut u32 = 0x60000800 as *mut u32;
const PERIPHS_IO_MUX_U0TXD_U:*mut u32 = (0x60000800 + 0x18) as *mut u32;
const PERIPHS_IO_MUX_FUNC: u32 =  0x13;
const PERIPHS_IO_MUX_FUNC_S: u32 = 4;
const FUNC_U0TXD: u32 = 0;

fn pin_func_select(pin:*mut u32, func: u32) {     
    write_peri_reg(pin, 
        read_peri_reg(pin) & 
         (!(PERIPHS_IO_MUX_FUNC<<PERIPHS_IO_MUX_FUNC_S))
         |( (((func&0x00000004)<<2)|(func&0x3))<<PERIPHS_IO_MUX_FUNC_S) );    
}


fn gpio16_output_conf() {

    unsafe {
        // mux configuration for XPD_DCDC to output rtc_gpio0
        let dcdc_conf = (PAD_XPD_DCDC_CONF.read_volatile() & 0xffffffbc) | 0x1;
        PAD_XPD_DCDC_CONF.write_volatile(dcdc_conf);
        //mux configuration for out enable
        let gpio_conf = (RTC_GPIO_CONF.read_volatile() & 0xfffffffe) | 0x0;
        RTC_GPIO_CONF.write_volatile(gpio_conf);
        //out enable
        let gpio_en = (RTC_GPIO_ENABLE.read_volatile() & 0xfffffffe) | 0x1;
        RTC_GPIO_ENABLE.write_volatile(gpio_en);
    };

}

fn gpio16_output_set(value: u32)
{
    unsafe {
        let val = (RTC_GPIO_OUT.read_volatile() & 0xfffffffe) | (value & 1);
        RTC_GPIO_OUT.write_volatile(val);
    };
}

fn gpio_set(pin:u32, val:u32) -> bool {
    unsafe { gpio_output_set((val)<<pin, ((!(val))&0x01)<<pin, 1<<pin,0); };
    true
}

//use panic_halt as _;
//use esp8266_hal::prelude::*;
//use esp8266_hal::target::Peripherals;
//use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};
//use esp8266_hal::gpio::{Gpio16, Output, PushPull};
//use esp8266_hal::rtccntl::CrystalFrequency;
//use core::str;
mod uart;
//mod wifi;



/*
#[no_mangle]
#[link(name="user_init")]
unsafe extern "C" fn user_init() -> u8 { return 0; }
*/
#[no_mangle]
#[link(name="user_pre_init")]
unsafe extern "C" fn user_pre_init() -> u8 { return 0; }

//#[entry]
#[no_mangle]
#[link(name="user_init")]
fn user_init() -> ! {    

    gpio16_output_conf();
    
    // Conf UART
    pin_pullup_dis(PERIPHS_IO_MUX_U0TXD_U);
    pin_func_select(PERIPHS_IO_MUX_U0TXD_U, FUNC_U0TXD);
    //uart::init();
    //wifi::init();

    uart::writestring("Connecting Wifi\r\n");
    //wifi::connect("", "");

    let mut i = -5;


    let mut connected = false;

    while !connected {
        uart::writestring("Check connection\r\n");
        //connected = wifi::is_connected();
        //timer1.delay_ms(500);
        unsafe { ets_delay_us(500000); };
        //unsafe { GPIO_OUT_W1TS_ADDRESS.write_volatile(1<<16); };
        gpio16_output_set(1);
        unsafe { ets_delay_us(100000); };
        //unsafe { GPIO_OUT_W1TC_ADDRESS.write_volatile(1<<16); };
        gpio16_output_set(0);

    }
    /*
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
*/
    loop {
        /*
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(100);
        led.toggle().unwrap();
        timer1.delay_ms(1000);
        led.toggle().unwrap();
        */

        uart::writestring("Loop..");
        uart::writenum(i);
        uart::writestring("\r\n");

        i = i+1;
    }
}
