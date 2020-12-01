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
/*
#define PERIPHS_IO_MUX                  0x60000800
#define PERIPHS_IO_MUX_PULLUP           0x00000080
#define PERIPHS_IO_MUX_FUNC             0x13
#define PERIPHS_IO_MUX_FUNC_S           4
#define READ_PERI_REG(addr) (*((volatile uint32_t *)ETS_UNCACHED_ADDR(addr)))
#define WRITE_PERI_REG(addr, val) (*((volatile uint32_t *)ETS_UNCACHED_ADDR(addr))) = (uint32_t)(val)
#define CLEAR_PERI_REG_MASK(reg, mask) WRITE_PERI_REG((reg), (READ_PERI_REG(reg)&(~(mask))))
#define SET_PERI_REG_MASK(reg, mask)   WRITE_PERI_REG((reg), (READ_PERI_REG(reg)|(mask)))
#define PIN_PULLUP_DIS(PIN_NAME)                 CLEAR_PERI_REG_MASK(PIN_NAME, PERIPHS_IO_MUX_PULLUP)
#define PIN_PULLUP_EN(PIN_NAME)                  SET_PERI_REG_MASK(PIN_NAME, PERIPHS_IO_MUX_PULLUP)

#define PERIPHS_IO_MUX_U0TXD_U          (PERIPHS_IO_MUX + 0x18)
#define FUNC_U0TXD                      0

#define PIN_FUNC_SELECT(PIN_NAME, FUNC)  do { \
    WRITE_PERI_REG(PIN_NAME,   \
                                READ_PERI_REG(PIN_NAME) \
                                     &  (~(PERIPHS_IO_MUX_FUNC<<PERIPHS_IO_MUX_FUNC_S))  \
                                     |( (((FUNC&0x00000004)<<2)|(FUNC&0x3))<<PERIPHS_IO_MUX_FUNC_S) );  \
    } while (0)

    PIN_PULLUP_DIS(PERIPHS_IO_MUX_U0TXD_U);
    PIN_FUNC_SELECT(PERIPHS_IO_MUX_U0TXD_U, FUNC_U0TXD);


PIN_FUNC_SELECT(PERIPHS_IO_MUX_GPIO2_U, FUNC_GPIO2);
GPIO_OUTPUT_SET(2, 0); //GPIO2 as output low

#define GPIO_OUTPUT_SET(gpio_no, bit_value) \
    gpio_output_set((bit_value)<<gpio_no, ((~(bit_value))&0x01)<<gpio_no, 1<<gpio_no,0)

#define PERIPHS_GPIO_BASEADDR               0x60000300
#define GPIO_OUT_W1TS_ADDRESS             0x04
#define GPIO_OUT_W1TC_ADDRESS             0x08
#define GPIO_REG_READ(reg)                         READ_PERI_REG(PERIPHS_GPIO_BASEADDR + reg)
#define GPIO_REG_WRITE(reg, val)                 WRITE_PERI_REG(PERIPHS_GPIO_BASEADDR + reg, val)
#define GPIO2_H         (GPIO_REG_WRITE(GPIO_OUT_W1TS_ADDRESS, 1<<2))
#define GPIO2_L         (GPIO_REG_WRITE(GPIO_OUT_W1TC_ADDRESS, 1<<2))
#define GPIO2(x)        ((x)?GPIO2_H:GPIO2_L)

#define PAD_XPD_DCDC_CONF                       (0x60000700 + 0x0A0)
#define RTC_GPIO_CONF                           (0x60000700 + 0x090)
#define RTC_GPIO_ENABLE                         (0x60000700 + 0x074)
#define RTC_GPIO_OUT                            (0x60000700 + 0x068)
void ICACHE_FLASH_ATTR
gpio16_output_conf(void)
{
    WRITE_PERI_REG(PAD_XPD_DCDC_CONF,
                   (READ_PERI_REG(PAD_XPD_DCDC_CONF) & 0xffffffbc) | (uint32)0x1); 	// mux configuration for XPD_DCDC to output rtc_gpio0

    WRITE_PERI_REG(RTC_GPIO_CONF,
                   (READ_PERI_REG(RTC_GPIO_CONF) & (uint32)0xfffffffe) | (uint32)0x0);	//mux configuration for out enable

    WRITE_PERI_REG(RTC_GPIO_ENABLE,
                   (READ_PERI_REG(RTC_GPIO_ENABLE) & (uint32)0xfffffffe) | (uint32)0x1);	//out enable
}

void ICACHE_FLASH_ATTR
gpio16_output_set(uint8 value)
{
    WRITE_PERI_REG(RTC_GPIO_OUT,
                   (READ_PERI_REG(RTC_GPIO_OUT) & (uint32)0xfffffffe) | (uint32)(value & 1));
}

*/

extern "C" {
    pub fn Uart_init() -> u8;
    pub fn ets_delay_us(time: u32) -> u8;
    pub fn gpio_output_set(set_mask:u32,clear_mask:u32,enable_mask:u32,disable_mask:u32);
}

const PAD_XPD_DCDC_CONF: *mut u32  =  (0x60000700 + 0x0A0) as *mut u32;
const RTC_GPIO_CONF: *mut u32      = (0x60000700 + 0x090) as *mut u32;
const RTC_GPIO_ENABLE: *mut u32    = (0x60000700 + 0x074) as *mut u32;
const RTC_GPIO_OUT: *mut u32       = (0x60000700 + 0x068) as *mut u32;

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
    //let dp = Peripherals::take().unwrap();
    //dp.RTCCNTL.rtc_control().set_crystal_frequency(CrystalFrequency::Crystal40MHz);

    //let pins = dp.GPIO.split();
    //let mut led = pins.gpio16.into_push_pull_output();
    //let (mut timer1, _) = dp.TIMER.timers();
    
    //let mut _serial = dp.UART0.serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());
    //timer1.delay_ms(100);

    gpio16_output_conf();
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
