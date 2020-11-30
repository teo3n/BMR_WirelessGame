#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

pub const NULL_MODE: u8 = 0;
pub const STATION_MODE: u8 = 1;
pub const SOFTAP_MODE: u8 = 2;
pub const STATIONAP_MODE: u8 = 3;

pub const STATION_IF: u8 = 0;
pub const SOFTAP_IF: u8 = 1;

pub const PHY_MODE_11B: u32 = 1;
pub const PHY_MODE_11G: u32 = 2;
pub const PHY_MODE_11N: u32 = 3;

pub const STATION_IDLE: u8 = 0;
pub const STATION_CONNECTING: u8 = 1;
pub const STATION_WRONG_PASSWORD: u8 = 2;
pub const STATION_NO_AP_FOUND: u8 = 3;
pub const STATION_CONNECT_FAIL: u8 = 4;
pub const STATION_GOT_IP: u8 = 5;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_addr {
    pub addr: u32,
}

pub type ip_addr_t = ip_addr;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ip_info {
    pub ip: ip_addr,
    pub netmask: ip_addr,
    pub gw: ip_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct station_config {
    pub ssid: [u8; 32usize],
    pub password: [u8; 64usize],
    pub bssid_set: u8,
    pub bssid: [u8; 6usize],
}

#[no_mangle]
#[link(name="user_init")]
unsafe extern "C" fn user_init() -> u8 { return 0; }

#[no_mangle]
#[link(name="user_pre_init")]
unsafe extern "C" fn user_pre_init() -> u8 { return 0; }

/*
#[no_mangle]
#[link(name="ets_memcpy")]
unsafe extern "C" fn ets_memcpy() -> u8 { return 0; }
#[no_mangle]
#[link(name="ets_printf")]
unsafe extern "C" fn ets_printf() -> u8 { return 0; }
#[no_mangle]
#[link(name="ets_memset")]
unsafe extern "C" fn ets_memset() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_strlen")]
unsafe extern "C" fn ets_strlen() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_set_idle_cb")]
unsafe extern "C" fn ets_set_idle_cb() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_delay_us")]
unsafe extern "C" fn ets_delay_us() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_intr_unlock")]
unsafe extern "C" fn ets_intr_unlock() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_intr_lock")]
unsafe extern "C" fn ets_intr_lock() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_update_cpu_frequency")]
unsafe extern "C" fn ets_update_cpu_frequency() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_bzero")]
unsafe extern "C" fn ets_bzero() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_memcmp")]
unsafe extern "C" fn ets_memcmp() -> u8 {  return 0;}
#[no_mangle]
#[link(name="hmac_sha1_vector")]
unsafe extern "C" fn hmac_sha1_vector() -> u8 {  return 0;}
#[no_mangle]
#[link(name="sha1_prf")]
unsafe extern "C" fn sha1_prf() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_post")]
unsafe extern "C" fn ets_post() -> u8 {  return 0;}
#[no_mangle]
#[link(name="ets_task")]
unsafe extern "C" fn ets_task() -> u8 { return 0; }

#[no_mangle]
#[link(name="Uart_Init")]
unsafe extern "C" fn Uart_Init() -> u8 { return 0; }
#[no_mangle]
#[link(name="uart_buff_switch")]
unsafe extern "C" fn uart_buff_switch() -> u8 { return 0; }

#[no_mangle]
#[link(name="flashchip")]
unsafe extern "C" fn flashchip() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_isr_mask")]
unsafe extern "C" fn ets_isr_mask() -> u8 { return 0; }


#[no_mangle]
#[link(name="ets_get_cpu_frequency")]
unsafe extern "C" fn ets_get_cpu_frequency() -> u8 { return 0; }


#[no_mangle]
#[link(name="gpio_output_set")]
unsafe extern "C" fn gpio_output_set() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_strcpy")]
unsafe extern "C" fn ets_strcpy() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_strncmp")]
unsafe extern "C" fn ets_strncmp() -> u8 { return 0; }

#[no_mangle]
#[link(name="pbkdf2_sha1")]
unsafe extern "C" fn pbkdf2_sha1() -> u8 { return 0; }
    
#[no_mangle]
#[link(name="gpio_pin_wakeup_enable")]
unsafe extern "C" fn gpio_pin_wakeup_enable() -> u8 { return 0; }

#[no_mangle]
#[link(name="gpio_pin_wakeup_disable")]
unsafe extern "C" fn gpio_pin_wakeup_disable() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_write_char")]
unsafe extern "C" fn ets_write_char() -> u8 { return 0; }
#[no_mangle]
#[link(name="ets_vprintf")]
unsafe extern "C" fn ets_vprintf() -> u8 { return 0; }

#[no_mangle]
#[link(name="Wait_SPI_Idle")]
unsafe extern "C" fn Wait_SPI_Idle() -> u8 { return 0; }
    
#[no_mangle]
#[link(name="Cache_Read_Disable")]
unsafe extern "C" fn Cache_Read_Disable() -> u8 { return 0; }
     
#[no_mangle]
#[link(name="SPI_read_status")]
unsafe extern "C" fn SPI_read_status() -> u8 { return 0; }
        
#[no_mangle]
#[link(name="SPI_write_status")]
unsafe extern "C" fn SPI_write_status() -> u8 { return 0; }
        
#[no_mangle]
#[link(name="SPI_write_enable")]
unsafe extern "C" fn SPI_write_enable() -> u8 { return 0; }
       
#[no_mangle]
#[link(name="SPIEraseSector")]
unsafe extern "C" fn SPIEraseSector() -> u8 { return 0; }
          
#[no_mangle]
#[link(name="SPIWrite")]
unsafe extern "C" fn SPIWrite() -> u8 { return 0; }
       
#[no_mangle]
#[link(name="SPIRead")]
unsafe extern "C" fn SPIRead() -> u8 { return 0; }

          
#[no_mangle]
#[link(name="ets_isr_attach")]
unsafe extern "C" fn ets_isr_attach() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_isr_unmask")]
unsafe extern "C" fn ets_isr_unmask() -> u8 { return 0; }

#[no_mangle]
#[link(name="_xtos_set_exception_handler")]
unsafe extern "C" fn _xtos_set_exception_handler() -> u8 { return 0; }

#[no_mangle]
#[link(name="rtc_get_reset_reason")]
unsafe extern "C" fn rtc_get_reset_reason() -> u8 { return 0; }


#[no_mangle]
#[link(name="lldesc_num2link")]
unsafe extern "C" fn lldesc_num2link() -> u8 { return 0; }

#[no_mangle]
#[link(name="lldesc_build_chain")]
unsafe extern "C" fn lldesc_build_chain() -> u8 { return 0; }
    
          
#[no_mangle]
#[link(name="__truncdfsf2")]
unsafe extern "C" fn __truncdfsf2() -> u8 { return 0; }

    
#[no_mangle]
#[link(name="phy_get_romfuncs")]
unsafe extern "C" fn phy_get_romfuncs() -> u8 { return 0; }

#[no_mangle]
#[link(name="roundup2")]
unsafe extern "C" fn roundup2() -> u8 { return 0; }

#[no_mangle]
#[link(name="strncmp")]
unsafe extern "C" fn strncmp() -> u8 { return 0; }

#[no_mangle]
#[link(name="ets_strncpy")]
unsafe extern "C" fn ets_strncpy() -> u8 { return 0; }

 
#[no_mangle]
#[link(name="rc4_skip")]
unsafe extern "C" fn rc4_skip() -> u8 { return 0; }

#[no_mangle]
#[link(name="aes_unwrap")]
unsafe extern "C" fn aes_unwrap() -> u8 { return 0; }
      
#[no_mangle]
#[link(name="hmac_md5")]
unsafe extern "C" fn hmac_md5() -> u8 { return 0; }

#[no_mangle]
#[link(name="hmac_sha1")]
unsafe extern "C" fn hmac_sha1() -> u8 { return 0; }

          
#[no_mangle]
#[link(name="rcons")]
unsafe extern "C" fn rcons() -> u8 { return 0; }

#[no_mangle]
#[link(name="gpio_input_get")]
unsafe extern "C" fn gpio_input_get() -> u8 { return 0; }

#[no_mangle]
#[link(name="_lit4_end")]
unsafe extern "C" fn _lit4_end() -> u8 { return 0; }
#[no_mangle]
#[link(name="rom_chip_v5_enable_cca")]
unsafe extern "C" fn rom_chip_v5_enable_cca() -> u8 { return 0; }
#[no_mangle]
#[link(name="rom_chip_v5_disable_cca")]
unsafe extern "C" fn rom_chip_v5_disable_cca() -> u8 { return 0; }
#[no_mangle]
#[link(name="Te0")]
unsafe extern "C" fn Te0() -> u8 { return 0; }
*/

extern "C" {
    pub fn wifi_set_opmode(opmode: u8) -> u8;
    pub fn wifi_set_phy_mode(mode: u32) -> u8;
    pub fn wifi_station_set_config(config: *mut station_config) -> u8;
    pub fn wifi_station_connect() -> u8;
    pub fn wifi_get_ip_info(if_index: u8, info: *mut ip_info) -> u8;
    pub fn wifi_station_get_connect_status() -> u8;
}

pub fn init() -> bool {
    true
}


pub fn connect(ssid: &str, passwd: &str) -> bool {

    let mut ssid_arr = [0u8; 32usize];
    let mut password_arr = [0u8; 64usize];
    for i in 0..ssid.as_bytes().len() {
        ssid_arr[i] = ssid.as_bytes()[i];
    }
    
    for i in 0..passwd.as_bytes().len() {
        password_arr[i] = ssid.as_bytes()[i];
    }

    let mut station_conf = station_config {
        ssid: ssid_arr,
        password: password_arr,
        bssid_set: 0,
        bssid: [0; 6],
    };

    unsafe {
        wifi_set_opmode( STATION_MODE );
        wifi_set_phy_mode(PHY_MODE_11N);
        wifi_station_set_config(& mut station_conf);
        wifi_station_connect();
    };

    true
}

pub fn is_connected() -> bool {

    let mut ipconfig = ip_info {
        ip: ip_addr { addr: 0 },
        netmask: ip_addr { addr: 0 },
        gw: ip_addr { addr: 0 },
    };
    unsafe { wifi_get_ip_info(STATION_IF, & mut ipconfig);
        if wifi_station_get_connect_status() == STATION_GOT_IP && ipconfig.ip.addr != 0 {
            return true;
        }
    };
    false
}

pub fn get_ip() -> u32 {
    let mut ipconfig = ip_info {
        ip: ip_addr { addr: 0 },
        netmask: ip_addr { addr: 0 },
        gw: ip_addr { addr: 0 },
    };
    unsafe { wifi_get_ip_info(STATION_IF, & mut ipconfig);};
    return ipconfig.ip.addr;
}

pub fn send_data(input: i32) -> bool {


    true
}

pub fn recv_data(input: i32) -> bool {


    true
}