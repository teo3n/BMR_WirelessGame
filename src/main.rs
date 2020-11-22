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

use gd32vf103xx_hal::gpio::gpiob::{ PB6, PB7 };

use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

pub mod lcd;
// pub mod led;
use riscv_rt::entry;
pub mod nunchuk;

#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    // Configure clocks
    let mut rcu = periph
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();
    let mut afio = periph.AFIO.constrain(&mut rcu);

    let gpioa = periph.GPIOA.split(&mut rcu);
    let gpiob = periph.GPIOB.split(&mut rcu);

    let i2c_0 = periph.I2C0;


    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(periph.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // Clear screen
    Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::BLACK,
        background_color = Rgb565::GREEN
    );

    let mut delay = McycleDelay::new(&rcu.clocks);
    delay.delay_ms(2);


    let scl = gpiob.pb6.into_alternate_open_drain();
    let sda = gpiob.pb7.into_alternate_open_drain();
    
    let mut i2c_handle = BlockingI2c::i2c0 (
        i2c_0,
        (scl, sda),
        &mut afio,
        Mode::standard(99500.hz()),
        &mut rcu,
        50,
        1,
        10000,
        200
    );

    // i2c_handle.write(0x52, &[0xF0, 0x55, 0xFB, 0x00]).unwrap();
    
    let mut write_res = i2c_handle.write(0x52, &[0x40, 0x00]);


    let mut i: u32 = 0;
    let mut f_val: u8 = 0;

    let mut read_ok = 0;
    let mut write_ok = 0;
    let mut init_write = 0;

    match write_res
    {
        Ok(x) => init_write = 1,
        _ => init_write = 0,
    }


    loop
    {
        let mut print_buf = ArrayString::<[_; 24]>::new();

        write!(&mut print_buf, "{}: iw: {}, r: {}, w: {}", i, init_write, read_ok, write_ok).expect("failed to create buffer");
        // write!(&mut print_buf, "{}: {} {} {} {}", i, read_buf[0], read_buf[1], read_buf[2], read_buf[3]).expect("failed to create buffer");


        Text::new(&print_buf, Point::new(10, 35))
            .into_styled(style)
            .draw(&mut lcd).unwrap();


        let mut read_buf: [u8; 5] = [0; 5];
        let read_res = i2c_handle.read(0x52, &mut read_buf);
        f_val = read_buf[0];

        delay.delay_us(100);

        write_res = i2c_handle.write(0x52, &[0x00 as u8]);


        match read_res
        {
            Ok(x) => read_ok = 1,
            _ => read_ok = 0,
        }
        
        match write_res
        {
            Ok(x) => write_ok = 1,
            _ => write_ok = 0,
        }

        i += 1;
        delay.delay_ms(100);
    }
}
