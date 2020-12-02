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
 *      - write bit setting and maybe all nops in assembly as well
 * 
 *  HOW TO USE:
 *      - initialize Ws2812 with
 * 
 *          let mut wspin = gpiob.pb5.into_push_pull_output();
 *          let mut ws2 = Ws2812::<_, LED_COUNT>::new(clock_speed, &mut wspin);
 * 
 *        where LED_COUNT is a compile-time constant, and wspin is a pin 
 *        implementing the OutputPin -trait (Output<PushPull>)
 * 
 *      - to set an led at index call 
 *          ws2.set_color(RGB { r: 255, g: 0 as u8, b: 0}, index);
 * 
 *      - to write changes to the leds call
 *          ws2.write_leds();
 */


/*
 * how it works:
 *  - every led is controlled by 3 8bit 
 *      brightness values, effectively 24 bits/led
 *  - every led takes in the first 3 bytes and forwards
 *      the rest to the next led
 *  - data is sent MSB first, data order in GRB
 */

pub use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::gpio;
use gd32vf103xx_hal::gpio::{ Output, PushPull };
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::digital::v2::OutputPin;

#[derive(Clone, Copy)]
pub struct RGB
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

enum State
{
    Low,
    High
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

    pub fn get_as_buffer(&self) -> [u8; 3]
    {
        return [self.g, self.r, self.b];
    }
}

// on average, one clock cycle in the delay_ns 
// amounts to this many nanoseconds, at 108MHz
const NS_WAIT_CLOCK_CYCLE: u32 = 37;

// at delay of 0ns this overhead still exists
const OVERHEAD_NS: u32 = 30;

pub struct Ws2812<'a, T, const N: usize>
{
    data_pin: &'a mut T,
    data_buffer: [RGB; N],
    clock_speed: u32,
}


// public methods
impl<'a, T, const N: usize> Ws2812<'a, T, N>
where T: OutputPin
{
    /// takes in a push-pull output pin, which is used as the data pin for the leds
    pub fn new(
        clock_speed: u32,
        data_pin: &'a mut T,
    ) -> Self
    {
        let buffer: [RGB; N] = [ RGB::zero(); N];
        // get the system clock speed

        let mut ws = Ws2812
        {
            data_pin,
            data_buffer: buffer,
            clock_speed,
        };

        return ws;
    }

    pub fn set_color(&mut self, rgb: RGB, led_index: u32)
    {
        self.data_buffer[led_index as usize] = rgb;
    }

    pub fn write_leds(&mut self)
    {
        for i in 0..self.data_buffer.len()
        {
            self.write_at(i as u32);
        }

        self.reset();
    }

    pub fn get_led_count(&self) -> u32
    {
        return self.data_buffer.len() as u32;
    }
}


// private methods
impl<'a, T, const N: usize> Ws2812<'a, T, N>
where T: OutputPin
{
    // writes the entire buffer to the led strip
    fn write_at(&mut self, write_index: u32)
    {
        let gbr_buffer: [u8; 3] = self.data_buffer[write_index as usize].get_as_buffer();
        
        // loop over GRB
        for j in 0..3
        {
            // loop over the bits in the byte
            for k in 0..8
            {
                match (gbr_buffer[j] >> (k - 7)) & 0x01
                {
                    0x00 => self.write_zero(),
                    0x01 => self.write_one(),

                    // this branch should never be reached, but 
                    // compiler nags if it isn't implemented
                    _ => (),
                }
            }
        }
    }

    // prepares the led strip for mroe data, holds reset
    fn reset(&mut self)
    {
        self.hold_pin_for_ns(5000, State::Low);
    }

    // using u32 is fine for this, because nobody in their right minds
    // would ever try to sleep for more than 1s with nanoseconds
    #[inline(always)]
    fn delay_ns(&mut self, ns: u32)
    {
        // this line here takes about 30ns
        let wait_for_cycles: u32 = (ns - OVERHEAD_NS) / NS_WAIT_CLOCK_CYCLE;

        // a for-loop in risc-v assembly
        unsafe
        {
            // https://shakti.org.in/docs/risc-v-asm-manual.pdf
            // https://github.com/Amanieu/rfcs/blob/inline-asm/text/0000-inline-asm.md
            // https://doc.rust-lang.org/beta/unstable-book/library-features/asm.html

            let mut counter: u32 = 0;
            asm!(
                "1:",
                    "addi  {0}, {0}, 1",
                    "blt   {0}, {1}, 1b",

                inout(reg) counter,
                in(reg) wait_for_cycles,
            );
        }
    }

    #[inline(always)]
    fn hold_pin_for_ns(&mut self, ns: u32, state: State)
    {
        match state
        {
            State::High => self.data_pin.set_high(),
            State::Low => self.data_pin.set_low(),
        };

        self.delay_ns(ns);
    }

    /// one is indicated by 800 ns high, 450 ns low
    #[inline(always)]
    fn write_one(&mut self)
    {
        // 850 ns
        self.hold_pin_for_ns(850, State::High);
        // 350
        self.hold_pin_for_ns(350+70, State::Low);
    }

    /// zero is indicated by 400 ns high, 850 ns low
    #[inline(always)]
    fn write_zero(&mut self)
    {
        // 360
        self.hold_pin_for_ns(360-10, State::High);
        // 850
        self.hold_pin_for_ns(850+100, State::Low);
    }
}