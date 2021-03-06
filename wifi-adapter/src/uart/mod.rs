
#[allow(dead_code)]

const UART_BASE : u32 = 0x60000000;
const UART_FIFO : *mut u8 = UART_BASE as *mut u8;
const UART_INT_CLR: *mut u16 = (UART_BASE + 0x10) as *mut u16;
const UART_STATUS: *mut u32 = (UART_BASE + 0x1C) as *mut u32;
const UART_CONF0: *mut u32 = (UART_BASE + 0x20) as *mut u32;
const UART_CONF1: *mut u32 = (UART_BASE + 0x24) as *mut u32;


// Bit-bang initialize the UART
pub fn init() -> bool {

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

// Iterate through a string and push each character to the UART FIFO buffer
pub fn writestring(input: &str) -> bool {
    unsafe {
        let _e = input.as_bytes()
            .iter()
            .for_each(|c| UART_FIFO.write_volatile(*c));            
    }
    true
}

// Write a single character to UART FIFO
pub fn writechr(input: u8) -> bool {
    unsafe {
        UART_FIFO.write_volatile(input);
    }
    true
}

// Convert number to a string and write the UART FIFO, supports whole 32-bit integer range
pub fn writenum(input: i32) -> bool {

    if input == 0 {
        unsafe { UART_FIFO.write_volatile('0' as u8); };
        return true;
    }

    // Find num of digits
    let mut divider = 1_000_000_000;
    let mut temp_in = input;

    // Handle negative numbers
    if temp_in < 0 {
        unsafe { UART_FIFO.write_volatile('-' as u8); };
        temp_in = -temp_in;
    }

    while temp_in / divider == 0 {
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

// Read one character from the UART FIFO
pub fn readchr(byte: &mut u8) -> bool {
    unsafe {
        if UART_STATUS.read_volatile() & 0xff == 0 {
            return false;
        }
        *byte = UART_FIFO.read_volatile();
    };
    return true;
}