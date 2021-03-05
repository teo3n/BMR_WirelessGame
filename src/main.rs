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
use gameboard::GameBoard;
use ws2812::{ Ws2812, RGB };


pub mod scoreboard;
use scoreboard::ScoreBoard;

// Configuration
const PIXEL_TOTAL_AMOUNT: usize = 256;
const X_LIMIT: usize = 16;
const Y_LIMIT: usize = 16;

const OLED_DEBUG_SCREEN: bool = false;
const SERIAL_DEBUG: bool = false;
const MASTER_DEVICE: bool = false;
const DEFAULT_TIMEOUT: u8 = 10;
const INCOMING_DATA_HEADER: [char;4] = ['D','A','T','A'];
const INCOMING_DATA_LEN: i8 = 4;
const NUNCHUK_THRES: i8 = 100; // threshold for moving
const PROJECTILE_NONE: char = '.';
const PROJECTILE_P1: char = '*';
const PROJECTILE_P2: char = '#';

#[derive(Copy, Clone)]
struct Player {
    x: f32,
    y: f32,
    color: RGB,
    shoot_timeout: u8,
    shoot_btn: bool,
    target_x:usize,
    target_y:usize,
    use_target: bool,
    input: nunchuk::ControllerInput,
}

// Read UART for input containing the remote player joystick etc data
// Checks for header string "DATA" and reads 4 bytes after that
// Returns either the full input values or None
fn read_remote_joy(rx: &mut gd32vf103xx_hal::serial::Rx<gd32vf103xx_hal::pac::USART0>,
     nunchuk_incoming_data: &mut [u8;4],
      nunchuk_index: &mut i8,
      tx: &mut gd32vf103xx_hal::serial::Tx<gd32vf103xx_hal::pac::USART1>,) -> Option<nunchuk::ControllerInput>
{
    let mut full_input_received = false;
    let mut input = nunchuk::ControllerInput{joy_x:0,joy_y:0,btn_z:0,btn_c:0,accel_x:0,accel_y:0,accel_z:0};
    let mut bytes = 0;
    loop {
        let read_byte = nb::block!(rx.read());
        let byte = match read_byte {
            Ok(o) => Some(o),
            Err(e) => None,
        };

        if byte.is_none() {
            break;
        }
        bytes = bytes +1;
        //tx.write(byte.unwrap());
        

        // Waiting for DATA string
        if *nunchuk_index < 0
        {
            // Iterate through the expected header string "DATA"
            if byte.unwrap() == INCOMING_DATA_HEADER[((*nunchuk_index)+4) as usize] as u8
            {
                *nunchuk_index = *nunchuk_index+1;
            } else { // Start from the beginning even if one character is wrong                
                *nunchuk_index = -4;
            }
        } else { // Header received
            // Receive 4 bytes of actual data
            nunchuk_incoming_data[(*nunchuk_index) as usize] = byte.unwrap();
            *nunchuk_index = *nunchuk_index+1;

            // Check if the last byte is received
            if *nunchuk_index == INCOMING_DATA_LEN 
            {
                // Reset the index at the 4th byte and extract the data to the ControllerInput
                *nunchuk_index = -4;
                full_input_received = true;
                input.joy_x = nunchuk_incoming_data[0] as i8;
                input.joy_y = nunchuk_incoming_data[1] as i8;
                input.btn_z = nunchuk_incoming_data[2];
                input.btn_c = nunchuk_incoming_data[3];

                if input.joy_x > 100 {
                    input.joy_x = -120;
                } else if input.joy_x < -100 {
                    input.joy_x = 120;
                }

                if input.joy_y > 100 {
                    input.joy_y = -120;
                } else if input.joy_y < -100 {
                    input.joy_y = 120;
                }

                // Debug
                write!(tx, "Got {} {}\r\n", input.joy_x, input.joy_y).expect("failed to create buffer");

            }
        }
        
    }
    write!(tx, "Read: {}\r\n", bytes).expect("failed to create buffer");
    if full_input_received {
        return Some(input);
    }
    return None;
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

    let pin_tx = gpioa.pa9.into_alternate_push_pull();
    let pin_rx = gpioa.pa10.into_floating_input();

    // Initialize serial for inputting player data through ESP8266
    let serial = Serial::new(
        periph.USART0,
        (pin_tx, pin_rx),
        Config::default().baudrate(74880.bps()),
        &mut afio,
        &mut rcu,
    );

    let pin_tx2 = gpioa.pa2.into_alternate_push_pull();
    let pin_rx2 = gpioa.pa3.into_floating_input();

    // Use 2nd UART for debugging while the other is occupied with player input data
    let serial2 = Serial::new(
        periph.USART1,
        (pin_tx2, pin_rx2),
        Config::default().baudrate(74880.bps()),
        &mut afio,
        &mut rcu,
    );
    //serial.listen(gd32vf103xx_hal::serial::Event::Rxne);

    let (mut tx, mut rx) = serial.split();
    let (mut tx2, rx2) = serial2.split();

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

    let temp_player = Player { x: 7.0f32, y:1.0f32, 
        color:colors::PURPLE,
        shoot_timeout: 4,
        shoot_btn: false,
        target_x: 0,
        target_y: 0,
        use_target: false,
        input: nunchuk::ControllerInput{joy_x:0,joy_y:0,btn_z:0,btn_c:0,accel_x:0,accel_y:0,accel_z:0},
    };
    let mut players = [temp_player, temp_player];
    players[1].y = 14.0f32;
    players[1].x = 8.0f32;
    players[1].color = colors::OLIVE;

    let mut objects: [Option<game::MovingObject>; game::MAXIMUM_OBJECTS] = [None; game::MAXIMUM_OBJECTS];
    let mut number_of_objects: usize = 0;
	
    delay.delay_ms(10);

    let mut wspin = gpiob.pb5.into_push_pull_output();
    let mut ws2 = Ws2812::<_, PIXEL_TOTAL_AMOUNT>::new(clock_speed, &mut wspin);
    let mut board = gameboard::GameBoard::<_>::new(&mut ws2);

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
    let sboard = ScoreBoard::new(&mut sboard_pin, 5);

    delay.delay_ms(100);

    // Temporary buffers for storing incoming nunchuk data
    let mut nunchuk_data: [u8;4];
    let mut nunchuk_incoming_data: [u8;4] = [0;4];
    let mut nunchuk_index: i8 = -4;

    loop
    {
        players[0].input = nchuck.get_input();        
        // Read other player's nunchuk data from UART, if available
        let remote_data = read_remote_joy(&mut rx, &mut nunchuk_incoming_data, &mut nunchuk_index, &mut tx2);        
        if remote_data.is_none() == false
        {
            players[1].input = remote_data.unwrap();
            board.set_color(0,0, colors::BLACK);
        }
        
        let mut input = players[0].input;

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

        // Master device handles the game logic and drawing to the screen
        if MASTER_DEVICE == true {

            // Apply the game physics
            if number_of_objects > 0 {
                game::game_tick(&mut objects, number_of_objects);
            }
            game::clear_board();

            for i in 0..number_of_objects {
                let mut object = match objects[i] {
                    Some(o) => o,
                    None => continue,
                };

                let pos = object.position();
                
                // "Explode" objects that have stopped
                if object.moving() == false || object.get_age() > 100 {                    
                    for obj_x in pos.0-2..=pos.0+2
                    {
                        for obj_y in pos.1-2..=pos.1+2
                        {
                            if obj_x > 0 && obj_x < (game::BOARD_WIDTH - 1) &&
                                obj_y > 0 && obj_y < (game::BOARD_WIDTH - 1)
                            {
                                if object.symbol == PROJECTILE_P1
                                {
                                    board.set_color(obj_x, obj_y, players[0].color);
                                } else {
                                    board.set_color(obj_x, obj_y, players[1].color);
                                }
                            }
                        }
                    }
                    objects[i] = None;
                } else { // Object not stopped yet, add to the game logic output
                    if pos.0 < 1 || pos.1 < 1 || pos.0 > game::BOARD_WIDTH - 1 || pos.1 > game::BOARD_WIDTH - 1 {
                        // ToDo: do something with out-of-bounds object
                    } else {
                        unsafe {                    
                            game::BOARD[pos.0 + pos.1 * game::BOARD_WIDTH] = object.symbol;
                        }
                        object.add_age();
                    }
                }
            }

            // Game logic processing done, parse through the list to find object positions from the output
            for y in 1..(game::BOARD_WIDTH - 1) {
                for x in 1..(game::BOARD_WIDTH - 1) {
                    unsafe {
                        if game::BOARD[x + y * game::BOARD_WIDTH] != PROJECTILE_NONE 
                        {
                            board.set_color_in_buffer(x, y, colors::NAVY);
                        }
                    }
                }
            }

            // Iterate both players
            for i in 0..=1
            {
                input = players[i].input;

                // If we are waiting after shooting or moving
                if players[i].shoot_timeout > 0
                {
                    players[i].shoot_timeout = players[i].shoot_timeout-1;
                } else if players[i].shoot_btn == true { // Trigger is pressed down, draw the targeting indicator

                    // Calculate target direction
                    let x_float:f32 = input.joy_x as f32;
                    let y_float:f32 = input.joy_y as f32;
                    let len:f32 = game::fast_sqrt(game::pow2(game::abs(x_float)) + game::pow2(game::abs(y_float)));

                    // Shoot when player button was pressed and now released
                    if input.btn_z == 0 {
                        players[i].shoot_timeout = DEFAULT_TIMEOUT;
                        players[i].shoot_btn = false;
                        players[i].use_target = false;

                        // Add offset to the projectile to compensate the game logic
                        let xpos = players[i].x - 1.0f32;
                        let ypos = players[i].y - 1.0f32;

                        let xdir = (x_float / len) * 2.0f32;
                        let ydir = (y_float / len) * 2.0f32;

                        // Iterate through the existing object list to find first empty space to push new
                        for ii in 0..(game::MAXIMUM_OBJECTS - 1) {

                            if objects[ii].is_none() || objects[ii].unwrap().symbol == PROJECTILE_NONE {

                                // Select correct symbol to know which player was shooting
                                let mut symbol = PROJECTILE_P1;
                                if i == 1
                                {
                                    symbol = PROJECTILE_P2;
                                }

                                let object = game::MovingObject::new(
                                        game::Vector { x: xpos, y: ypos },
                                        game::Vector { x: xdir, y: ydir },
                                        symbol);
                                objects[ii] = Some(object);
                                if ii + 1 > number_of_objects
                                {
                                    number_of_objects = ii + 1;
                                }
                                break;
                            }
                        }

                    } else { // Draw target vector if player still holding the trigger
                        
                        let xpos = (players[i].x + (x_float / len) * 3.0f32 ) as i32;
                        let ypos = (players[i].y + (y_float / len) * 3.0f32 ) as i32;

                        // Make sure the target is not outside the boundaries
                        if xpos > 0 && ypos > 0 && xpos < (X_LIMIT-1) as i32 && ypos < (Y_LIMIT-1) as i32 {
                            players[i].target_x = xpos as usize;
                            players[i].target_y = ypos as usize;
                            players[i].use_target = true;
                        }
                    }
                } else { // If player is not waiting and not pressing the trigger yet
                    if input.btn_z == 1 {
                        players[i].shoot_btn = true;
                    } else if input.joy_x > NUNCHUK_THRES ||
                              input.joy_x < -NUNCHUK_THRES ||
                              input.joy_y > NUNCHUK_THRES ||
                              input.joy_y < -NUNCHUK_THRES {
                        // Move player when joystick (range -128..127) over NUNCHUK_THRES
                        let mut moved:bool = false;
                        let mut x_dir:i8 = 0;
                        let mut y_dir:i8 = 0;

                        // Check if moving on X-axis
                        if input.joy_x > NUNCHUK_THRES {
                            x_dir = 1;
                        } else if input.joy_x < -NUNCHUK_THRES {
                            x_dir = -1;
                        }

                        // Check if moving on Y-axis
                        if input.joy_y > NUNCHUK_THRES {
                            y_dir = 1;
                        } else if input.joy_y < -NUNCHUK_THRES {
                            y_dir = -1;
                        }

                        // If we are moving make sure we are not over the boundaries in any direction
                        if x_dir != 0 || y_dir != 0 {
                            moved = true;
                            if (players[i].x as i8)+x_dir > 0 && (players[i].x as i8)+x_dir < (X_LIMIT-1) as i8
                            {
                                players[i].x += x_dir as f32;
                            }
                            if (players[i].y as i8)+y_dir > 0 && (players[i].y as i8)+y_dir < (Y_LIMIT-1)  as i8
                            {
                                players[i].y += y_dir as f32;
                            }
                        }
                        
                        // Add "shoot timeout" also after moving
                        if moved == true {
                            players[i].shoot_timeout = DEFAULT_TIMEOUT>>1;
                        }

                    }
                }
                
                // If targeting was enabled, draw the actual red target to screen
                if players[i].use_target == true 
                {
                    board.set_color_in_buffer(players[i].target_x, players[i].target_y, colors::RED);
                }

                // Draw the player to the screen without adding to the game board
                board.set_color_in_buffer(players[i].x as usize, players[i].y as usize, colors::YELLOW);
            }

            // Update the whole display
            board.update_matrix();
            board.flush_to_buffer();

            // Draw a trail after the projectiles using the shooting player color
            for i in 0..number_of_objects {
                let object = match objects[i] {
                    Some(o) => o,
                    None => continue,
                };
                let pos = object.position();
                if object.symbol == '*'
                {
                    board.set_color(pos.0 as usize, pos.1 as usize, players[0].color);
                } else {
                    board.set_color(pos.0 as usize, pos.1 as usize, players[1].color);
                }                    
            }
        } else { // Client device only sends the current nunchuk data to the master

            nunchuk_data = nchuck.serialize();

            // Send data to the master
            write!(tx,"DATA").expect("failed to create buffer");
            delay.delay_ms(1);
            for i in 0..4 {
                nb::block!(tx.write(nunchuk_data[i])).expect("failed to write");
            }
            write!(tx,"END\n").expect("failed to create buffer");
        }

        //
        // Calculate score
        //
        const TOTAL_PIXELS: u8 = 14*14;
        let mut score: [u8;2] = [0;2];
        for y in 1..(game::BOARD_WIDTH - 1) {
            for x in 1..(game::BOARD_WIDTH - 1) {
                let color = board.get_color(x, y);
                if color == players[0].color
                {
                    score[0] = score[0] + 1;
                } 
                if color == players[1].color
                {
                    score[1] = score[1] + 1;
                } 
            }
        }

        // Ending condition, two colors cover the whole board
        if score[0]+score[1] == TOTAL_PIXELS
        {
            let color_end: RGB;

            // Check for the winned and select the color
            if score[0] > score[1] {
                color_end = players[0].color;
            } else if score[0] < score[1] {
                color_end = players[1].color;
            } else { // Tie
                color_end = colors::GREEN;
            }

            // Blink the winner color until reset
            loop {                
                for y in 1..(game::BOARD_WIDTH - 1) {
                    for x in 1..(game::BOARD_WIDTH - 1) {
                        board.set_color_in_buffer(x, y, color_end);
                    }
                }
                board.update_matrix();
                board.flush_to_buffer();
                delay.delay_ms(1000);
                for y in 1..(game::BOARD_WIDTH - 1) {
                    for x in 1..(game::BOARD_WIDTH - 1) {
                        board.set_color_in_buffer(x, y, colors::BLACK);
                    }
                }
                board.update_matrix();
                board.flush_to_buffer();
                delay.delay_ms(1000);
            }
        }

        delay.delay_ms(100);
    }
}
