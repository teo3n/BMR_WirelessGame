#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[path = "../uart/mod.rs"] mod uart;

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
pub struct wifi_fast_scan_threshold {
    pub rssi: u8,
    pub authmode:u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct station_config {
    pub ssid: [u8; 32usize],
    pub password: [u8; 64usize],
    pub bssid_set: u8,
    pub bssid: [u8; 6usize],
    pub threshold: wifi_fast_scan_threshold,
    pub open_and_wep_mode_disable: u8,
    pub all_channel_scan: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct softap_config {
    pub ssid: [u8; 32usize],
    pub password: [u8; 64usize],
    pub ssid_len: u8,    // Note: Recommend to set it according to your ssid
    pub channel: u8,    // Note: support 1 ~ 13
    pub authmode: u32,    // Note: Don't support AUTH_WEP in softAP mode.
    pub ssid_hidden: u8,    // Note: default 0
    pub max_connection: u8,    // Note: default 4, max 4
    pub beacon_interval: u16,    // Note: support 100 ~ 60000 ms, default 100
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dhcps_lease {
    enable: u8,
    start_ip:ip_addr,
    end_ip: ip_addr,
}

/*
#[no_mangle]
#[link(name="tkip")]
unsafe extern "C" fn tkip() -> u8 { return 0; }


#[no_mangle]
#[link(name="wep")]
unsafe extern "C" fn wep() -> u8 { return 0; }
*/
extern "C" {
    pub fn wifi_set_opmode(opmode: u8) -> u8;
    pub fn wifi_set_phy_mode(mode: u32) -> u8;
    pub fn wifi_station_set_config_current(config: *mut station_config) -> u8;
    pub fn wifi_station_get_config(config: *mut station_config) -> u8;
    pub fn wifi_softap_set_config(config: *mut softap_config) -> u8;
    pub fn wifi_softap_get_config(config: *mut softap_config) -> u8;
    pub fn wifi_station_connect() -> u8;
    pub fn wifi_get_ip_info(if_index: u8, info: *mut ip_info) -> u8;
    pub fn wifi_set_ip_info(if_index: u8, info: *mut ip_info) -> u8;
    pub fn wifi_station_get_connect_status() -> u8;
    pub fn ets_memcpy(dst:*mut u8, src:*const u8,size:u32);
    pub fn ets_memset(dst:*mut u8, val:u8,size:u32);
    pub fn wifi_station_disconnect() -> u8;
    pub fn wifi_station_set_hostname(name: *mut u8) -> u8;
    pub fn wifi_softap_dhcps_start() -> u8;
    pub fn wifi_softap_dhcps_stop() -> u8;
    pub fn wifi_softap_set_dhcps_lease(please: *mut dhcps_lease) -> u8;
    pub fn wifi_softap_set_dhcps_offer_option(level: u8, arg: *mut u32 ) -> u8;
    pub fn wifi_softap_set_dhcps_lease_time(minutes: u32) -> u8;
    pub fn wifi_set_event_handler_cb();
}

pub fn init() -> bool {
    true
}


pub fn connect(ssid: &str, passwd: &str) -> i32 {

    let client_mode: u8 = 1;
    let mut ssid_arr = [0u8; 32usize];
    let mut password_arr = [0u8; 64usize];
    /*
    for i in 0..ssid.as_bytes().len() {
        ssid_arr[i] = ssid.as_bytes()[i];
    }
    
    for i in 0..passwd.as_bytes().len() {
        password_arr[i] = passwd.as_bytes()[i];
    }
    */
    
    if client_mode == 0 {
        let mut station_conf = station_config {
            ssid: [0;32],
            password: [0;64],
            bssid_set: 0,
            bssid: [0; 6],
            threshold: wifi_fast_scan_threshold { rssi: 0, authmode: 0 },
            open_and_wep_mode_disable: 0,
            all_channel_scan: 0,
        };

        unsafe {
            /*
            ets_memcpy(station_conf.ssid.as_mut_ptr(), ssid.as_ptr(), 32);
            ets_memcpy(station_conf.password.as_mut_ptr(), passwd.as_ptr(), 32);

            station_conf.ssid[ssid.len()] = '\0' as u8;
            station_conf.password[passwd.len()] = '\0' as u8;
            */
            for i in 0..ssid.as_bytes().len() {
                station_conf.ssid[i] = ssid.as_bytes()[i];
            }
            
            for i in 0..passwd.as_bytes().len() {
                station_conf.password[i] = passwd.as_bytes()[i];
            }

            //station_conf.threshold[0] = 84;
            //station_conf.threshold[1] = 4;

            //ets_memset((station_conf.ssid.as_mut_ptr() as u32+ssid.len() as u32) as *mut u8, 0, (32-ssid.len()) as u32);
            //ets_memset((station_conf.password.as_mut_ptr() as u32+passwd.len() as u32) as *mut u8, 0, (64-passwd.len()) as u32);
        };

        unsafe {
            //wifi_station_disconnect();
            if wifi_set_opmode( STATION_MODE ) == 0 {
                return 1;
            }
            if wifi_set_phy_mode(PHY_MODE_11N) == 0 {
                return 2;
            }
            if wifi_station_set_config_current(& mut station_conf) == 0 {
                return 3;
            }
    /*
            wifi_station_get_config(& mut station_conf2);
            
            uart::writestring("\r\n");
            for i in 0..ssid.len() {
                uart::writechr(station_conf2.ssid[i]);
            };
            uart::writestring("\r\n");
            for i in 0..passwd.len() {
                uart::writechr(station_conf2.password[i]);
            };

            uart::writestring("\r\n");
    */
            
            if wifi_station_connect() == 0 {
                return 4;
            }
        }
    } else {

        unsafe {
            if wifi_set_opmode( SOFTAP_MODE ) == 0 {
                return 1;
            }
        };

        let mut station_conf = softap_config {
            ssid: [0;32],
            password: [0;64],
            ssid_len: ssid.len() as u8,    // Note: Recommend to set it according to your ssid
            channel: 7,    // Note: support 1 ~ 13
            authmode: 4,    // Note: Don't support AUTH_WEP in softAP mode.
            ssid_hidden: 0,    // Note: default 0
            max_connection: 4,    // Note: default 4, max 4
            beacon_interval: 100,    // Note: support 100 ~ 60000 ms, default 100
        };

        unsafe {
            wifi_softap_get_config(& mut station_conf);

            ets_memcpy(station_conf.ssid.as_mut_ptr(), ssid.as_ptr(), 32);
            ets_memcpy(station_conf.password.as_mut_ptr(), passwd.as_ptr(), 32);

            station_conf.ssid[ssid.len()] = '\0' as u8;
            station_conf.password[passwd.len()] = '\0' as u8;

            station_conf.channel = 7;
            station_conf.ssid_hidden = 0;

        };

        unsafe {

            


            if wifi_softap_set_config(& mut station_conf) == 0 {
                return 3;
            }

            let mut ipconfig = ip_info {
                ip: ip_addr { addr: 0x0a0a0001 },
                netmask: ip_addr { addr: 0xffffff00 },
                gw: ip_addr { addr: 0x0a0a0001 },
            };

            /*
            if wifi_set_phy_mode(PHY_MODE_11N) == 0 {
                return 2;
            }
            */
            //wifi_softap_dhcps_stop();
            /*
            if wifi_set_ip_info(SOFTAP_IF, & mut ipconfig) == 0 {
                return 2;
            }

            let mut dhcp = dhcps_lease {
                enable: 0,
                start_ip: ip_addr { addr: 0x0a0a0064 },
                end_ip: ip_addr { addr: 0x0a0a00c8 },
            };

            if wifi_softap_set_dhcps_lease(& mut dhcp) == 0 {
                return 5;
            }

            wifi_softap_set_dhcps_lease_time(720);

            let mut mode = 1;
            if wifi_softap_set_dhcps_offer_option(1, & mut mode) == 0 {
                return 6;
            }

            if wifi_softap_dhcps_start() == 0 {
                return 4;
            }
            */
            /*
            if wifi_set_ip_info(SOFTAP_IF, & mut ipconfig) == 0 {
                return 2;
            }
            */
        }
        
    }

    return 0;
}

pub fn is_connected() -> u8 {

    let mut ipconfig = ip_info {
        ip: ip_addr { addr: 0 },
        netmask: ip_addr { addr: 0 },
        gw: ip_addr { addr: 0 },
    };
    unsafe { //wifi_get_ip_info(STATION_IF, & mut ipconfig);
        return wifi_station_get_connect_status();
    };
}

pub fn get_ip() -> u32 {
    let mut ipconfig = ip_info {
        ip: ip_addr { addr: 0 },
        netmask: ip_addr { addr: 0 },
        gw: ip_addr { addr: 0 },
    };
    unsafe { wifi_get_ip_info(SOFTAP_IF, & mut ipconfig); };
    return ipconfig.ip.addr;
}

pub fn send_data(input: i32) -> bool {


    true
}

pub fn recv_data(input: i32) -> bool {


    true
}