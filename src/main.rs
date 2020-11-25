#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use gd32vf103xx_hal as hal;

use panic_halt as _;
use arrayvec::ArrayString;
use core::fmt::Write;

// use embedded_graphics::image::{Image, ImageRaw};
use embedded_graphics::pixelcolor::raw::LittleEndian;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{primitive_style, text_style};
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::pac::I2C0;
// use gd32vf103xx_hal::spi::{Spi, MODE_0};
use gd32vf103xx_hal::i2c::{BlockingI2c, Mode};
use gd32vf103xx_hal::prelude::*;

use gd32vf103xx_hal::gpio::{Alternate, OpenDrain};
use gd32vf103xx_hal::gpio::gpiob::{ PB8, PB9 };

use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

use riscv_rt::entry;
pub mod nunchuk;


#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    let mut rcu = periph
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let mut afio = periph.AFIO.constrain(&mut rcu);
    let gpiob = periph.GPIOB.split(&mut rcu);
    let i2c_0 = periph.I2C0;
    let mut delay = McycleDelay::new(&rcu.clocks);
    delay.delay_ms(2);

    let scl = gpiob.pb8.into_alternate_open_drain();
    let sda = gpiob.pb9.into_alternate_open_drain();
    
    let mut i2c_handle = BlockingI2c::i2c0 (
        i2c_0,
        (scl, sda),
        &mut afio,
        Mode::standard(100.khz()),
        &mut rcu,
        998,
        1,
        998,
        998
    );

    // TODO: submit bugreport on incorrect master read ACK

    // i2c_handle.write(0x52, &[0xF0, 0x55, 0xFB, 0x00]);
    let shit = i2c_handle.write(0x52, &[0x40, 0x00]);

    delay.delay_ms(10);

    // let shit = i2c_handle.write(0x52, &[0x00]);


    loop
    {
        let mut read_buf: [u8; 6] = [0; 6];
        let read_shit = i2c_handle.read(0x52, &mut read_buf);

        delay.delay_us(100);
        let write_white = i2c_handle.write(0x52, &[0x00]);

        delay.delay_ms(100);
    }
}
