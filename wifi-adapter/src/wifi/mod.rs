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
#[link(name="tkip")]
unsafe extern "C" fn tkip() -> u8 { return 0; }


#[no_mangle]
#[link(name="wep")]
unsafe extern "C" fn wep() -> u8 { return 0; }

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