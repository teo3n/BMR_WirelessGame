/*
 * 	Teo Niemirepo
 *  teo.niemirepo@tuni.fi
 *
 * 	14.12.2020
 *
 * 	an easy to use per-controller scoreboard,
 *  requires a strip of ws2812 LEDs to work
 */


use crate::ws2812::{ RGB, Ws2812 };
pub use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::gpio;
use gd32vf103xx_hal::gpio::{ Output, PushPull };
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::digital::v2::OutputPin;

const LED_COUNT: usize = 5;

pub struct ScoreBoard<'a, T>
{
    pub score: u8,
    ws: Ws2812<'a, T, LED_COUNT>,
    max_score: u8,
}

// public methods
impl<'a, T> ScoreBoard<'a, T>
where T: OutputPin
{
    pub fn new(pin: &'a mut T, max_score: u8 ) -> Self
    {
        let mut ws = Ws2812::<_, LED_COUNT>::new(108_000_000, pin);

        return ScoreBoard
        {
            score: 0,
            ws,
            max_score,
        };
    }

    // increments the score by incr
    pub fn add_score(&mut self, incr: u8)
    {
        self.score += incr;
        if self.score > self.max_score
        {
            self.score = self.max_score;
        }
        self.update_scoreboard();
    }

    // decrements the score by decr
    pub fn delete_score(&mut self, decr: u8)
    {
        if (self.score as i16 - decr as i16) < 0
        {
            self.score = 0;
        }
        self.score -= decr;
        self.update_scoreboard();
    }

    // updates the scoreboard leds
    pub fn update_scoreboard(&mut self)
    {
        let score_colors = self.score_to_colors();
        for (i, color) in score_colors.iter().enumerate()
        {
            self.ws.set_color(*color, i as u32);
        }

        self.ws.write_leds();
    }
}

// private methods
impl<'a, T> ScoreBoard<'a, T>
where T: OutputPin
{
    fn score_to_colors(&self) -> [RGB; LED_COUNT]
    {
        let mut colors: [RGB; LED_COUNT] = [ RGB::zero(); LED_COUNT];
        
        let score_leds: u8 = ((self.score as f32 / self.max_score as f32) * LED_COUNT as f32) as u8;
        
        for i in 0..(LED_COUNT as u8)
        {
            if i <= score_leds
            {
                colors[i as usize] = RGB{ r: 255, g: 0, b: 0 };
            }
            else
            {
                colors[i as usize] = RGB::zero();
            }
        }

        return colors;
    }
}