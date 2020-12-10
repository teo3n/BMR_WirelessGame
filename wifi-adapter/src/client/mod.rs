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
    pub connect_callback: espconn_connect_callback,
    pub reconnect_callback: espconn_reconnect_callback,
    pub disconnect_callback: espconn_connect_callback,
	pub write_finish_fn: espconn_connect_callback,
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
    pub recv_callback: espconn_recv_callback,
    pub sent_callback: espconn_sent_callback,
    pub ink_cnt: u8,
    pub reverse: *mut u32,
}

extern "C" {
    pub fn espconn_regist_connectcb(espconn: *mut espconn, connect_cb: espconn_connect_callback) -> u8;
    pub fn espconn_regist_disconcb(espconn: *mut espconn, connect_cb: espconn_connect_callback) -> u8;    
    pub fn espconn_regist_recvcb(espconn: *mut espconn, recv_cb: espconn_recv_callback) -> u8;    
    pub fn espconn_connect(espconn: *mut espconn) -> u8;
    pub fn espconn_send(espconn: *mut espconn, psent: *const u8, length: u16) -> u8;
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

static mut TCP1: esp_tcp = esp_tcp {
    remote_port: 0,
    local_port: 0,
    local_ip: 0,
    remote_ip: 0,
    connect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(dummy_func_client) },
    reconnect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_reconnect_callback>(dummy_func_client) },
    disconnect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(dummy_func_client) },
	write_finish_fn: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(dummy_func_client) },
};
static mut CONN: espconn = espconn { 
    conn_type: 0,
    state: 0,
    tcp: unsafe { core::mem::transmute::<u32,*mut esp_tcp>(0) } ,
    recv_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_recv_callback>(dummy_func_client) } ,
    sent_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_sent_callback>(dummy_func_client) } ,
    ink_cnt: 0,
    reverse: unsafe { core::mem::transmute::<u32,*mut u32>(0) } ,
};

static mut SEND_BUFFER: [u8; 100usize] = [0;100];
static mut BUFFER_POS: u16 = 0;
static mut IN_CONN: * mut espconn = unsafe { core::mem::transmute::<u32,* mut espconn>(0) } ;

// Just so that the rust compiler doesn't complain about null pointers
// "type validation failed: encountered 0x00000000 at xxx, but expected a function pointer"
#[no_mangle]
#[link(name="dummy_func")]
unsafe extern "C" fn dummy_func_client(arg:*const u32)
{
}

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
    espconn_regist_recvcb(core::mem::transmute::<*mut u32,* mut espconn>(arg), webclient_recv);
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
        if unsafe { core::mem::transmute::<* mut espconn, u32>(IN_CONN) } != 0 {
            espconn_send(IN_CONN, &SEND_BUFFER[0], BUFFER_POS);
            BUFFER_POS = 0;
        }
    };
}

pub fn init() {

    unsafe {
        TCP1.remote_port = 8000;
        TCP1.remote_ip = 0xc0a80401;
        CONN.conn_type = espconn_type::ESPCONN_TCP as u32;
        CONN.state = espconn_state::ESPCONN_NONE as u32;

        CONN.tcp = & mut TCP1;    
    
        espconn_regist_connectcb(& mut CONN, webclient_connect);
        espconn_connect(& mut CONN);
    };
}