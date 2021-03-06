#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[path = "../uart/mod.rs"] mod uart;

// Function pointer types used
pub type espconn_connect_callback = unsafe extern "C" fn(arg: *mut u32);
pub type espconn_reconnect_callback = unsafe extern "C" fn(arg: *mut u32, err: i32);
pub type espconn_recv_callback = unsafe extern "C" fn(arg: *mut u32, data: *const u8, len: u16);
pub type espconn_sent_callback = unsafe extern "C" fn(arg: *mut u32);


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct esp_tcp {
    pub remote_port: u32,
    pub local_port: u32,
    pub local_ip: u32,
    pub remote_ip: u32,
    pub connect_callback: Option<espconn_connect_callback>,
    pub reconnect_callback: Option<espconn_reconnect_callback>,
    pub disconnect_callback: Option<espconn_connect_callback>,
	pub write_finish_fn: Option<espconn_connect_callback>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct espconn {
    /** type of the espconn (TCP, UDP) */
    pub conn_type: u32,
    /** current state of the espconn */
    pub state: u32,
    pub tcp: *mut esp_tcp,        
    /** A callback function that is informed about events for this espconn */
    pub recv_callback: Option<espconn_recv_callback>,
    pub sent_callback: Option<espconn_sent_callback>,
    pub ink_cnt: u8,
    pub reverse: *mut u32,
}

extern "C" {
    pub fn espconn_regist_connectcb(espconn: *mut espconn, connect_cb: espconn_connect_callback) -> u8;
    pub fn espconn_regist_disconcb(espconn: *mut espconn, connect_cb: espconn_connect_callback) -> u8;    
    pub fn espconn_regist_recvcb(espconn: *mut espconn, recv_cb: espconn_recv_callback) -> u8;    
    pub fn espconn_connect(espconn: *mut espconn) -> u8;
    pub fn espconn_send(espconn: *mut espconn, psent: *const u8, length: u16) -> u8;
    pub fn dummy_func(arg:*const u32);
    pub fn espconn_set_opt(espconn: *mut espconn,opt: u32);
    pub fn espconn_set_keepalive(espconn: *mut espconn,param: u32, arg:*const u32);
}

#[repr(u32)]
pub enum espconn_state {
    ESPCONN_NONE = 0,
    ESPCONN_WAIT = 1,
    ESPCONN_LISTEN = 2,
    ESPCONN_CONNECT = 3,
    ESPCONN_WRITE = 4,
    ESPCONN_READ = 5,
    ESPCONN_CLOSE = 6
}

#[repr(u32)]
pub enum espconn_type {
    ESPCONN_INVALID    = 0,
    /* ESPCONN_TCP Group */
    ESPCONN_TCP        = 0x10,
    /* ESPCONN_UDP Group */
    ESPCONN_UDP        = 0x20,
}

#[repr(u32)]
pub enum espconn_level{
	ESPCONN_KEEPIDLE = 0,
	ESPCONN_KEEPINTVL = 1,
	ESPCONN_KEEPCNT = 2,
}

const ESPCONN_KEEPALIVE: u32 = 0x08;

static mut TCP1: esp_tcp = esp_tcp {
    remote_port: 0,
    local_port: 0,
    local_ip: 0,
    remote_ip: 0,
    connect_callback: None,
    reconnect_callback: None,
    disconnect_callback: None,
	write_finish_fn: None,
};
static mut CONN: espconn = espconn { 
    conn_type: 0,
    state: 0,
    tcp: unsafe { core::mem::transmute::<u32,*mut esp_tcp>(0) } ,
    recv_callback: None,
    sent_callback: None,
    ink_cnt: 0,
    reverse: unsafe { core::mem::transmute::<u32,*mut u32>(0) } ,
};

static mut SEND_BUFFER: [u8; 100usize] = [0;100];
static mut BUFFER_POS: u16 = 0;
static mut CONNECTED: bool = false;


#[no_mangle]
#[link(name="webclient_recv")]
unsafe extern "C" fn webclient_recv(arg:*mut u32, data: *const u8, len: u16)
{
  for i in 0..len {
    uart::writechr(*data.offset(i as isize));
  }
}

#[no_mangle]
#[link(name="webclient_connect")]
unsafe extern "C" fn webclient_connect(arg:*mut u32)
{
    unsafe { CONNECTED = true; };
    uart::writestring("TCP conn made..\r\n");

    let mut keep_alive:u32 = 1;
    espconn_set_opt(core::mem::transmute::<*mut u32,* mut espconn>(arg), ESPCONN_KEEPALIVE);
    espconn_set_keepalive(core::mem::transmute::<*mut u32,* mut espconn>(arg), espconn_level::ESPCONN_KEEPIDLE as u32, &keep_alive);
    keep_alive = 5; //repeat interval = 5s
    espconn_set_keepalive(core::mem::transmute::<*mut u32,* mut espconn>(arg), espconn_level::ESPCONN_KEEPINTVL as u32, &keep_alive);
    keep_alive = 2;//repeat 2times
    espconn_set_keepalive(core::mem::transmute::<*mut u32,* mut espconn>(arg), espconn_level::ESPCONN_KEEPCNT as u32, &keep_alive);

    espconn_regist_recvcb(core::mem::transmute::<*mut u32,* mut espconn>(arg), webclient_recv);
}

#[no_mangle]
#[link(name="webclient_disconnect")]
unsafe extern "C" fn webclient_disconnect(arg:*mut u32)
{
    unsafe { CONNECTED = false; };
    uart::writestring("TCP conn lost..\r\n");    
    init();
}

pub fn writechr(val: u8) {
    unsafe {
        SEND_BUFFER[BUFFER_POS as usize] = val;
        BUFFER_POS= BUFFER_POS+1;
        // Flush if buffer already full
        if BUFFER_POS == 100 {
            sendbuf();
        }
    };
}

pub fn sendbuf() {
    unsafe {
        if unsafe { CONNECTED } && BUFFER_POS != 0 {
            espconn_send(& mut CONN, &SEND_BUFFER[0], BUFFER_POS);
        }
        BUFFER_POS = 0;
    };
}

pub fn init() {

    unsafe {
        unsafe { CONNECTED = false; } ;
        TCP1.remote_port = 8000;
        TCP1.remote_ip = 0x0104a8c0; // 192.168.4.1
        CONN.conn_type = espconn_type::ESPCONN_TCP as u32;
        CONN.state = espconn_state::ESPCONN_NONE as u32;

        CONN.tcp = & mut TCP1;    
    
        espconn_regist_connectcb(& mut CONN, webclient_connect);
        espconn_regist_disconcb(& mut CONN, webclient_disconnect);
        espconn_connect(& mut CONN);
    };
}