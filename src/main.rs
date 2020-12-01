#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

#![feature(asm)]

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
use gd32vf103xx_hal::gpio::gpiob::{ PB8, PB9, PB5 };

use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

pub mod lcd;
use riscv_rt::entry;
pub mod nunchuk;
pub mod ws2812;
use ws2812::{ Ws2812, RGB };

#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    let mut rcu = periph
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

    let clock_speed = rcu.clocks.sysclk().0;

    let gpioa = periph.GPIOA.split(&mut rcu);
    let gpiob = periph.GPIOB.split(&mut rcu);

    let mut afio = periph.AFIO.constrain(&mut rcu);
    let mut delay = McycleDelay::new(&rcu.clocks);

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(periph.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    // clear the screen
    Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
        .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
        .draw(&mut lcd)
        .unwrap();

    let style = text_style!(
        font = Font6x8,
        text_color = Rgb565::BLACK,
        background_color = Rgb565::GREEN
    );

    delay.delay_ms(2);

    let i2c0 = periph.I2C0;
    let scl = gpiob.pb8.into_alternate_open_drain();use embedded_hal::digital::v2::OutputPin;
    let sda = gpiob.pb9.into_alternate_open_drain();
    let mut nchuck = nunchuk::Nunchuk::new(&mut afio, &mut rcu, i2c0, scl, sda);

    delay.delay_ms(10);


    let mut wspin = gpiob.pb5.into_push_pull_output();
    let mut ws2 = Ws2812::new(clock_speed, &mut wspin);

    loop
    {
        let input: nunchuk::ControllerInput = nchuck.get_input();

        // print out the joystick values
        let mut display_buffer_joy = ArrayString::<[_; 26]>::new();
        write!(&mut display_buffer_joy, "joy_x: {}: joy_y: {}  ", input.joy_x, input.joy_y).expect("failed to create buffer");
        Text::new(&display_buffer_joy, Point::new(10, 10))
            .into_styled(style)
            .draw(&mut lcd).unwrap();

        // print out the button states
        let mut display_buffer_btn = ArrayString::<[_; 26]>::new();
        write!(&mut display_buffer_btn, "btn_z: {}: btn_c: {}  ", input.btn_z, input.btn_c).expect("failed to create buffer");
        Text::new(&display_buffer_btn, Point::new(10, 30))
            .into_styled(style)
            .draw(&mut lcd).unwrap();

        // print out the accelerometer values
        let mut display_buffer_accel = ArrayString::<[_; 26]>::new();
        write!(&mut display_buffer_accel, "az: {}: ay: {} az: {}  ", input.accel_x, input.accel_y, input.accel_z).expect("failed to create buffer");
        Text::new(&display_buffer_accel, Point::new(10, 50))
            .into_styled(style)
            .draw(&mut lcd).unwrap();

        for i in 0..2
        {
            if input.btn_z == 1
            {
                ws2.set_color(RGB { r: 255 as u8, g: 0 as u8, b: 0}, i);
            }
            else
            {
                ws2.set_color(RGB { r: 0 as u8, g: 255 as u8, b: 0}, i);
            }
        }
        

        ws2.write_leds();
        delay.delay_ms(1);
    }
}
