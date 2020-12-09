#![no_std]
#![no_main]

#[allow(unused_mut)]
#[allow(dead_code)]

const SERVER_MODE: u32 = 1;

pub type ETSTimerFunc = unsafe extern "C" fn(timer_arg: *const u32);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct os_timer_t {
    pub timer_next: *mut os_timer_t,
    pub timer_expire: u32,
    pub timer_period: u32,
    pub timer_func: ETSTimerFunc,
    pub timer_arg: *const u32,
}

impl Default for os_timer_t {
    fn default () -> os_timer_t {
        os_timer_t{timer_next: 0 as *mut os_timer_t,
                   timer_expire: 0,
                   timer_period: 0,
                   timer_func: update as ETSTimerFunc,
                   timer_arg: 0 as * const u32
                }
    }
}

const GPIO_BASE : u32 = 0x60000300;
const GPIO_OUT_W1TS_ADDRESS :*mut u32 = (GPIO_BASE+0x04) as *mut u32;
const GPIO_OUT_W1TC_ADDRESS :*mut u32 = (GPIO_BASE+0x08) as *mut u32;

extern "C" {
    pub fn ets_delay_us(time: u32) -> u8;
    pub fn gpio_output_set(set_mask:u32,clear_mask:u32,enable_mask:u32,disable_mask:u32);
    pub fn system_timer_reinit();
    pub fn system_soft_wdt_stop();
    pub fn ets_wdt_disable();
    pub fn wifi_get_opmode() -> u8;
    pub fn wifi_get_phy_mode() -> u32;
    pub fn system_soft_wdt_feed();

    pub fn ets_timer_disarm(timer: *mut os_timer_t);
    pub fn ets_timer_arm_new(timer: *mut os_timer_t, time: u32, repeat: u8, ms: u8);
    pub fn ets_timer_setfn(timer: *mut os_timer_t, function: ETSTimerFunc, arg: *const u32);
    pub fn ets_timer_arm(timer: *mut os_timer_t, time: u32, repeat: u8);
    pub fn ets_timer_init();
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
    write_peri_reg(reg, read_peri_reg(reg)&(!(mask)))
}
fn set_peri_reg_mask(reg:*mut u32, mask: u32)   {
    write_peri_reg(reg, read_peri_reg(reg)|(mask))
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




mod uart;
mod wifi;
mod server;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {
        unsafe { ets_delay_us(500000); };
        uart::writestring("PANIC\r\n");
    }
}


#[no_mangle]
#[link(name="user_pre_init")]
unsafe extern "C" fn user_pre_init() -> u8 { return 0; }


#[no_mangle]
#[link(name="user_rf_cal_sector_set")]
unsafe extern "C" fn user_rf_cal_sector_set() -> u32
{
    return 512 - 5;
}

static mut CONNECTED:bool = false;

#[no_mangle]
#[link(name="update")]
unsafe extern "C" fn update(timer_arg: *const u32) {

    if SERVER_MODE == 0 {
        if CONNECTED == false {
            
            uart::writestring("Check connection..");
            let status = wifi::is_connected();
            if status == 5 { // Status == Connected
                CONNECTED = true;
            } else { // else wait for the next update                
                return;
            }
            uart::writenum(status as i32);
            uart::writestring("..stationmode..");
            unsafe { uart::writenum(wifi_get_opmode() as i32); };
            uart::writestring("..phymode..");
            unsafe { uart::writenum(wifi_get_phy_mode() as i32); };
            uart::writestring("\r\n");

            // Print out IP address for debugging
            let ip = wifi::get_ip();

            uart::writestring("IP: ");
            uart::writenum((ip >> 24) as i32);
            uart::writestring(".");
            uart::writenum((ip >> 16 & 0xff) as i32);
            uart::writestring(".");
            uart::writenum((ip >> 8 & 0xff) as i32);
            uart::writestring(".");
            uart::writenum((ip & 0xff) as i32);
            uart::writestring("\r\n");
        }

    } else {
        let mut byte: u8 = 0;

        // Read chars from the uart and push to the tcp-connection buffer
        while uart::readchr(&mut byte) {
            server::writechr(byte);
        }
        // Send the entire buffer
        server::sendbuf();
    }
}

static mut UPDATE_TIMER:os_timer_t = os_timer_t {
    timer_next: 0 as *mut os_timer_t,
    timer_expire: 0,
    timer_period: 0,
    timer_func: update as ETSTimerFunc,
    timer_arg: 0 as * const u32
 };

//#[entry]
#[no_mangle]
#[link(name="user_init")]
fn user_init() {

    unsafe {
        system_timer_reinit();
    };

    // GPIO16 is a special pin, but the only LED in my ESP-wroom-02 WEMOS
    gpio16_output_conf();
    
    // Conf UART
    pin_pullup_dis(PERIPHS_IO_MUX_U0TXD_U);
    pin_func_select(PERIPHS_IO_MUX_U0TXD_U, FUNC_U0TXD);
    uart::init();
    wifi::init();

    if SERVER_MODE == 0 { // Client mode
        uart::writestring("Connecting Wifi\r\n");
        let con_status = wifi::connect("BMR_wireless", "wire123456");

    } else { // Server mode
        uart::writestring("Setup Wifi server\r\n");
        let con_status = wifi::setup_server("BMR_wireless", "wire123456");
        server::init();
    }    
    
    unsafe {
        let param:u32 = 0;
        ets_timer_disarm(& mut UPDATE_TIMER);
        ets_timer_setfn(& mut UPDATE_TIMER, update, &param);
        ets_timer_arm_new(& mut UPDATE_TIMER, 100, 1, 1);
    };

}
