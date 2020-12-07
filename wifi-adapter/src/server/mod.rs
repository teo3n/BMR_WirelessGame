#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

// Function pointer types used
pub type espconn_connect_callback = unsafe extern "C" fn(arg: *const u32);
pub type espconn_reconnect_callback = unsafe extern "C" fn(arg: *const u32, err: i32);
pub type espconn_recv_callback = unsafe extern "C" fn(arg: *const u32, arg: *const u8, len: u16);
pub type espconn_sent_callback = unsafe extern "C" fn(arg: *const u32);


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
    pub fn espconn_regist_recvcb(espconn: *mut espconn, recv_cb: espconn_recv_callback) -> u8;
    pub fn espconn_accept(espconn: *mut espconn) -> u8;
    pub fn espconn_send(espconn: *mut espconn, psent: *mut u8, length: u16) -> u8;
}

const ESPCONN_NONE:u32 = 0;
const ESPCONN_TCP:u32 = 0x10;

static mut tcp1: esp_tcp = esp_tcp {
    remote_port: 0,
    local_port: 0,
    local_ip: 0,
    remote_ip: 0,
    connect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(webserver_listen) },
    reconnect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_reconnect_callback>(webserver_listen) },
    disconnect_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(webserver_listen) },
	write_finish_fn: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_connect_callback>(webserver_listen) },
};
static mut conn: espconn = espconn { 
    conn_type: 0,
    state: 0,
    tcp: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),*mut esp_tcp>(webserver_listen) } ,
    recv_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_recv_callback>(webserver_listen) } ,
    sent_callback: unsafe { core::mem::transmute::<unsafe extern "C" fn(*const u32),espconn_sent_callback>(webserver_listen) } ,
    ink_cnt: 0,
    reverse: unsafe { core::mem::transmute::<u32,*mut u32>(0) } ,
};

#[no_mangle]
#[link(name="webserver_listen")]
unsafe extern "C" fn webserver_listen(arg:*const u32)
{
    //struct espconn *pesp_conn = arg;
    //espconn_regist_recvcb(pesp_conn, webserver_recv);
}

pub fn init() {

    unsafe {
        tcp1.local_port = 8000;
        conn.conn_type = ESPCONN_TCP;
        conn.state = ESPCONN_NONE;

        conn.tcp = & mut tcp1;    
    
        espconn_regist_connectcb(& mut conn, webserver_listen);
        espconn_accept(& mut conn);
    };
}