#![no_std]
#![no_main]
#![allow(unused_imports)]
#![allow(unused_variables)]

#![feature(asm)]
#![feature(min_const_generics)]

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

use gd32vf103xx_hal::gpio::{ Alternate, OpenDrain, PushPull};
use gd32vf103xx_hal::gpio::gpiob::{ PB8, PB9, PB5 };

use gd32vf103xx_hal::rcu::RcuExt;
use gd32vf103xx_hal::delay::McycleDelay;
use gd32vf103xx_hal::serial::{Serial, Config};
use embedded_hal::blocking::delay::DelayMs;

pub mod lcd;
use riscv_rt::entry;

pub mod nunchuk;
pub mod ws2812;
pub mod gameboard;
pub mod colors;

pub mod game;
use gameboard::Gameboard;
use ws2812::{ Ws2812, RGB };


pub mod scoreboard;
use scoreboard::ScoreBoard;

// Configuration
const PIXEL_TOTAL_AMOUNT: usize = 256;
const X_LIMIT: usize = 16;
const Y_LIMIT: usize = 16;

const OLED_DEBUG_SCREEN: bool = false;
const SERIAL_DEBUG: bool = true;
const MASTER_DEVICE: bool = true;
const DEFAULT_TIMEOUT: u8 = 10;

#[derive(Copy, Clone)]
struct Player {
    x: f32,
    y: f32,
    color: RGB,
    shoot_timeout: u8,
    shoot_btn: bool
}

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

    let serial = Serial::new(
        periph.USART0,
        (gpioa.pa9, gpioa.pa10),
        Config::default().baudrate(115200.bps()),
        &mut afio,
        &mut rcu,
    );

    let (mut tx, mut rx) = serial.split();

    if SERIAL_DEBUG == true
    {
        write!(tx,"Starting..\r\n").expect("failed to create buffer");
    }

    let lcd_pins = lcd_pins!(gpioa, gpiob);
    let mut lcd = lcd::configure(periph.SPI0, lcd_pins, &mut afio, &mut rcu);
    let (width, height) = (lcd.size().width as i32, lcd.size().height as i32);

    if OLED_DEBUG_SCREEN == true
    {
        // clear the screen
        Rectangle::new(Point::new(0, 0), Point::new(width - 1, height - 1))
            .into_styled(primitive_style!(fill_color = Rgb565::BLACK))
            .draw(&mut lcd)
            .unwrap();
    }
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

    let temp_player = Player { x: 7.5f32, y:1.0f32, color:colors::WHITE, shoot_timeout: 0, shoot_btn: false,};
    let mut players = [temp_player, temp_player];
    players[1].y = 14.0f32;

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

    let mut wspin = gpiob.pb5.into_push_pull_output();
    let mut ws2 = Ws2812::<_, PIXEL_TOTAL_AMOUNT>::new(clock_speed, &mut wspin);
    let mut board = gameboard::Gameboard::<_>::new(&mut ws2);

    // Add borders
    for x in 0..X_LIMIT
    {
        board.set_color(x, 0, colors::GREEN);
        board.set_color(x, Y_LIMIT-1, colors::GREEN);
    }
    for y in 0..Y_LIMIT
    {
        board.set_color(0, y, colors::GREEN);
        board.set_color(X_LIMIT-1, y, colors::GREEN);
    }
    //flush board 
    board.flush();

    let mut sboard_pin = gpiob.pb6.into_push_pull_output();

    // second argument is the maximum score
    let mut sboard = ScoreBoard::new(&mut sboard_pin, 5);

    delay.delay_ms(100);

    loop
    {
        let input: nunchuk::ControllerInput = nchuck.get_input();

        if OLED_DEBUG_SCREEN == true
        {
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
        }
        if SERIAL_DEBUG == true
        {
            write!(tx,"joy_x: {}: joy_y: {}  \r\n", input.joy_x, input.joy_y).expect("failed to create buffer");
            write!(tx,"btn_z: {}: btn_c: {}  \r\n", input.btn_z, input.btn_c).expect("failed to create buffer");
            //write!(tx,"az: {}: ay: {} az: {}  \r\n", input.accel_x, input.accel_y, input.accel_z).expect("failed to create buffer");
        }
        // for i in 0..ws2.get_led_count()
        // {
        //     if input.btn_z == 1
        //     {
        //         ws2.set_color(RGB { r: 255 as u8, g: 0 as u8, b: 0}, i);
        //     }
        //     else
        //     {
        //     }
        // }

        //game::print_board();

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
            //break;
        }
        if index < tba_objects.len() {
            tba_objects[index].0 -= 1;
        }


        for y in 1..(game::BOARD_WIDTH - 1) {
            for x in 1..(game::BOARD_WIDTH - 1) {
                unsafe {
                    if game::BOARD[x + y * game::BOARD_WIDTH] != '.' 
                    {
                        board.set_color(x, y, colors::NAVY);
                    } else {
                        board.set_color(x, y, colors::BLACK);
                    }
                }
            }
        }
        
        /*
        for x in 1..X_LIMIT
        {            
            for y in 0..3
            {
                board.swap(x,y,x-1,y);
            }
        }*/
        let mut target_x:usize = 0;
        let mut target_y:usize = 0;
        let mut use_target = false;
        let mut original_color: RGB = colors::BLACK;

        if players[0].shoot_timeout > 0
        {
            players[0].shoot_timeout = players[0].shoot_timeout-1;
        } else if players[0].shoot_btn == true { 

            // Shoot on release
            if input.btn_z == 0 {
                players[0].shoot_timeout = DEFAULT_TIMEOUT;
                players[0].shoot_btn = false;
            }
            else {
                // Draw target vector
                let x_float:f32 = input.joy_x as f32 / 64.0f32;
                let y_float:f32 = input.joy_y as f32 / 64.0f32;
                let xpos = (players[0].x + x_float) as i32;
                let ypos = (players[0].y + y_float) as i32;
                if xpos > 0 && ypos > 0 && xpos < (X_LIMIT-1) as i32 && ypos < (Y_LIMIT-1) as i32 {
                    target_x = xpos as usize;
                    target_y = ypos as usize;
                    use_target = true;
                }
            }
        } else {
            if input.btn_z == 1 {
                players[0].shoot_btn = true;
            }
        }

        
        if use_target == true 
        {
            original_color = board.get_color(target_x, target_y);
            board.set_color(target_x, target_y, colors::RED);
        }

        board.update_matrix();
        if use_target == true 
        {
            board.set_color(target_x, target_y, original_color);
        }
        delay.delay_ms(100);
    }
}
