/*
 *	A correct re-implemnetation of reading from i2c -bus, adapted from
 *	https://docs.rs/gd32vf103xx-hal/0.4.0/src/gd32vf103xx_hal/i2c.rs.html
 *
 * 	Adds functionality for sending an ACK after every succesful
 * 	byte read.
 */

pub use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::i2c::{BlockingI2c, Mode};

use gd32vf103xx_hal::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use gd32vf103xx_hal::gpio::{ Alternate, OpenDrain };
use gd32vf103xx_hal::pac::{ I2C0, I2C1 };
use gd32vf103xx_hal::rcu::{ Rcu, Clocks, Enable, Reset, BaseFrequency };
use gd32vf103xx_hal::time::Hertz;
use gd32vf103xx_hal::afio::{ Afio, Remap };
use riscv::register::mcycle;
use nb::Error::{ Other, WouldBlock };
use nb::{ Error as NbError, Result as NbResult };


macro_rules! busy_wait {
    ($nb_expr:expr, $exit_cond:expr) => {{
        loop {
            let res = $nb_expr;
            if res != Err(WouldBlock) {
                break res;
            }
            if $exit_cond {
                break res;
            }
        }
    }};
}

macro_rules! busy_wait_cycles {
    ($nb_expr:expr, $cycles:expr) => {{
        let started = mcycle::read();
        let cycles = $cycles as usize;
        busy_wait!(
            $nb_expr,
            mcycle::read().wrapping_sub(started) >= cycles
        )
    }};
}

pub trait ReadAck
{
	fn send_start_and_wait_ack(&mut self);
    fn read_with_ack(&mut self,  addr: u8, buffer: &mut [u8]);
}

impl ReadAck for BlockingI2c<pac::I2C0, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>
{
	fn send_start_and_wait_ack(&mut self)
	{
        let mut retries_left = self.start_retries;
        let mut last_ret: NbResult<(), Error> = Err(WouldBlock);
        while retries_left > 0
        {
            self.nb.send_start();
            last_ret = busy_wait_cycles!(self.nb.wait_after_sent_start(), self.start_timeout);
            
            if last_ret.is_err()
            {
                self.nb.reset();
            }
            else
            {
                break;
            }
            retries_left -= 1;
        }
        last_ret
    }

    fn read_with_ack(&mut self,  addr: u8, buffer: &mut [u8])
    {
        self.send_start_and_wait().unwrap();
    }

}