/*
 * 	Teo Niemirepo
 *  teo.niemirepo@tuni.fi
 *
 * 	01.12.2020
 *
 * 	a driver for ws2812 -leds
 * 
 *  TODO: 
 *      - should error checking be implemented?
 *      - move to macro -based, so multiple instances
 *          with different led counts can exist
 */

/*
 * how it works:
 *  - every led is controlled by 3 8bit 
 *      brightness values, effectively 24 bits/led
 *  - every led takes in the first 3 bytes and forwards
 *      the rest to the next led
 */

pub use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::gpio::{ Output, PushPull };
use gd32vf103xx_hal::gpio; //::{ PB4 };
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

pub struct RGB
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB
{
    pub fn zero() -> Self
    {
        return RGB
        {
            r: 0,
            g: 0,
            b: 0,
        };
    }
}

const LED_COUNT: usize = 3;

pub struct Ws2812<'a>
{
    data_pin: &'a mut gpio::Pxx<Output<PushPull>>,
    data_buffer: [RGB; LED_COUNT],
}


// public methods
impl<'a> Ws2812<'a>
{
    /// takes in a push-pull output pin, which is used as the data pin for the leds
    pub fn new(
        data_pin: &'a mut gpio::Pxx<Output<PushPull>>) -> Self
    {
        let mut buffer: [RGB; LED_COUNT] = [ RGB::zero(); LED_COUNT];

        let mut ws = Ws2812
        {
            data_pin,
            data_buffer: buffer,
        };

        ws.init();
        return ws;
    }

    pub fn set_color(&mut self, rgb: RGB, led_index: u32)
    {
        self.data_buffer[led_index as usize] = rgb;
        self.write_data(led_index);
    }
}


// private methods
impl<'a> Ws2812<'a>
{
    // writes the entire buffer to the led strip
    fn write_data(&mut self, write_count: u32)
    {
        for i in 0..write_count
        {
            // TODO: write here
        }
    }

    // initializes the led strip, holds reset
    fn init(&mut self)
    {

    }
}