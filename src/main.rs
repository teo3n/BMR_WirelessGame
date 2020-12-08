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

pub mod lcd;
use riscv_rt::entry;
pub mod nunchuk;

pub mod game;


#[entry]
fn main() -> ! {
    let periph = pac::Peripherals::take().unwrap();

    let mut rcu = periph
        .RCU
        .configure()
        .ext_hf_clock(8.mhz())
        .sysclk(108.mhz())
        .freeze();

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
    let scl = gpiob.pb8.into_alternate_open_drain();
    let sda = gpiob.pb9.into_alternate_open_drain();
    let mut nchuck = nunchuk::Nunchuk::new(&mut afio, &mut rcu, i2c0, scl, sda);

    let mut tba_objects: [(usize, f32, f32, char); 8] = [
        (0, 3.536f32, -3.536f32, '*'),
        (0, -2.868f32, -4.096f32, '#'),
        (6, -1.710f32, 4.698f32, '*'),
        (2, -4.728f32, 1.628f32, '#'),
        (2, 0.436f32, 4.981f32, '#'),
        (1, 2.868f32, 4.096f32, '*'),
        (5, -4.193f32, 2.723f32, '#'),
        (5, 4.830f32, -1.294f32, '*'),
    ];

    let mut objects: [Option<game::MovingObject>; game::MAXIMUM_OBJECTS] = [None; game::MAXIMUM_OBJECTS];
    let mut number_of_objects: usize = 0;
    let mut index = 0;

    while tba_objects[index].0 == 0 {
        let (_, x, y, symbol) = tba_objects[index];
        objects[number_of_objects] = Some(
            game::MovingObject::new(game::Vector {
                x,
                y,
            }, symbol));
        index += 1;
        number_of_objects += 1;
    }
	
    delay.delay_ms(10);


    loop {
        game::print_board();

        game::game_tick(&mut objects, number_of_objects);
        game::clear_board();

        while index < tba_objects.len() && tba_objects[index].0 == 0 {
            let (_, x, y, symbol) = tba_objects[index];
            objects[number_of_objects] = Some(
                game::MovingObject::new(game::Vector { x, y }, symbol)
            );
            index += 1;
            number_of_objects += 1;
        }

        let mut moving: bool = false;
        for i in 0..number_of_objects {
            let object = match objects[i] {
                Some(o) => o,
                None => continue,
            };
            unsafe {
                let pos = object.position();
                game::BOARD[pos.0 + pos.1 * game::BOARD_WIDTH] = object.symbol;
            }
            moving |= object.moving();
        }

        if !moving && (index >= tba_objects.len()) {
            break;
        }
        if index < tba_objects.len() {
            tba_objects[index].0 -= 1;
        }
    }

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


        delay.delay_ms(100);
    }
}
