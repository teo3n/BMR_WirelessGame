//! Pulse width modulation

use embedded_hal::Pwm;
use gd32vf103_pac::{TIMER0, TIMER1, TIMER2, TIMER3, TIMER4};

use crate::gpio::{Alternate, PushPull};
use crate::gpio::gpioa::*;
use crate::gpio::gpiob::*;
use crate::rcu::{Rcu, Enable, Reset, BaseFrequency};
use crate::time::{Hertz, U32Ext};

pub trait PwmChannelPin<TIMER> {}
macro_rules! pwm_pin {
    ($timer:ty, $pin:ident) => {
        impl PwmChannelPin<$timer> for $pin<Alternate<PushPull>> {}
    }
}

// TODO: Handle alternate mappings.

pwm_pin!(TIMER0, PA8);
pwm_pin!(TIMER0, PA9);
pwm_pin!(TIMER0, PA10);
pwm_pin!(TIMER0, PA11);

pwm_pin!(TIMER1, PA0);
pwm_pin!(TIMER1, PA1);
pwm_pin!(TIMER1, PA2);
pwm_pin!(TIMER1, PA3);

pwm_pin!(TIMER2, PA6);
pwm_pin!(TIMER2, PA7);
pwm_pin!(TIMER2, PB0);
pwm_pin!(TIMER2, PB1);

pwm_pin!(TIMER3, PB6);
pwm_pin!(TIMER3, PB7);
pwm_pin!(TIMER3, PB8);
pwm_pin!(TIMER3, PB9);

// Channel 3 only.
// TODO: Enforce this in the code.
pwm_pin!(TIMER4, PA3);

pub struct PwmTimer<'a, TIMER> {
    timer: TIMER,
    timer_clock: Hertz,
    max_duty_cycle: u16,
    period: Hertz,
    duty: [u16; 4],
    ch0: Option<&'a dyn PwmChannelPin<TIMER>>,
    ch1: Option<&'a dyn PwmChannelPin<TIMER>>,
    ch2: Option<&'a dyn PwmChannelPin<TIMER>>,
    ch3: Option<&'a dyn PwmChannelPin<TIMER>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Channel(pub u8);

macro_rules! advanced_pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<'a> PwmTimer<'a, $TIM> {
            pub fn new(timer: $TIM,
                       rcu: &mut Rcu,
                       ch0: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch1: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch2: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch3: Option<&'a dyn PwmChannelPin<$TIM>>) -> Self {

                $TIM::enable(rcu);
                $TIM::reset(rcu);

                /* Advanced TIMER implements a BREAK function that deactivates
                * the outputs. This bit automatically activates the output when
                * no break input is present */
                timer.cchp.modify(|_, w| w.oaen().set_bit());

                PwmTimer {
                    timer,
                    timer_clock: $TIM::base_frequency(rcu),
                    max_duty_cycle: 0,
                    period: 0.hz(),
                    duty: [0u16; 4],
                    ch0,
                    ch1,
                    ch2,
                    ch3,
                }
            }
        }

        pwm_timer!($TIM: $tim);
    };
}


macro_rules! general_pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<'a> PwmTimer<'a, $TIM> {
            pub fn new(timer: $TIM,
                       rcu: &mut Rcu,
                       ch0: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch1: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch2: Option<&'a dyn PwmChannelPin<$TIM>>,
                       ch3: Option<&'a dyn PwmChannelPin<$TIM>>) -> Self {
                $TIM::enable(rcu);
                $TIM::reset(rcu);

                PwmTimer {
                    timer,
                    timer_clock: $TIM::base_frequency(rcu),
                    max_duty_cycle: 0,
                    period: 0.hz(),
                    duty: [0u16; 4],
                    ch0,
                    ch1,
                    ch2,
                    ch3,
                }
            }
        }

        pwm_timer!($TIM: $tim);
    };
}

macro_rules! pwm_timer {
    ($TIM:ident: $tim:ident) => {
        impl<'a> Pwm for PwmTimer<'a, $TIM> {
            type Channel = Channel;
            type Time = Hertz;
            type Duty = u16;

            fn disable(&mut self, channel: Self::Channel) {
                match channel.0 {
                    0 if self.ch0.is_some() => self.timer.chctl2.modify(|_r, w| w.ch0en().clear_bit()),
                    1 if self.ch1.is_some() => self.timer.chctl2.modify(|_r, w| w.ch1en().clear_bit()),
                    2 if self.ch2.is_some() => self.timer.chctl2.modify(|_r, w| w.ch2en().clear_bit()),
                    3 if self.ch3.is_some() => self.timer.chctl2.modify(|_r, w| w.ch3en().clear_bit()),
                    _ => (),
                }
            }

            fn enable(&mut self, channel: Self::Channel) {
                match channel.0 {
                    0 if self.ch0.is_some() => self.timer.chctl2.modify(|_r, w| w.ch0en().set_bit()),
                    1 if self.ch1.is_some() => self.timer.chctl2.modify(|_r, w| w.ch1en().set_bit()),
                    2 if self.ch2.is_some() => self.timer.chctl2.modify(|_r, w| w.ch2en().set_bit()),
                    3 if self.ch3.is_some() => self.timer.chctl2.modify(|_r, w| w.ch3en().set_bit()),
                    _ => (),
                }
            }

            fn get_period(&self) -> Self::Time {
                return self.period;
            }

            fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
                self.duty[channel.0 as usize]
            }

            fn get_max_duty(&self) -> Self::Duty {
                self.max_duty_cycle
            }

            fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
                let mut duty = duty;
                if duty > self.max_duty_cycle {
                    duty = self.max_duty_cycle
                }
                self.duty[channel.0 as usize] = duty;
                self.disable(channel.clone());
                match channel.0 {
                    0 if self.ch0.is_some() => self.timer.ch0cv.write(|w| unsafe { w.bits(duty) }),
                    1 if self.ch1.is_some() => self.timer.ch1cv.write(|w| unsafe { w.bits(duty) }),
                    2 if self.ch2.is_some() => self.timer.ch2cv.write(|w| unsafe { w.bits(duty) }),
                    3 if self.ch3.is_some() => self.timer.ch3cv.write(|w| unsafe { w.bits(duty) }),
                    _ => (),
                }
                self.enable(channel);
            }

            fn set_period<P>(&mut self, period: P) where
                P: Into<Self::Time> {
                self.timer.ctl0.modify(|_, w| w.cen().clear_bit());
                self.timer.cnt.reset();

                let freq = period.into();

                let ticks = self.timer_clock.0 / freq.0;
                let psc = ((ticks - 1) / (1 << 16)) as u16;
                let car = (ticks / ((psc + 1) as u32)) as u16;

                self.max_duty_cycle = car;
                self.period = freq;

                self.timer.psc.write(|w| unsafe { w.bits(psc) });
                self.timer.car.write(|w| unsafe { w.bits(car) });

                self.timer.chctl0_output().modify(|_r, w| unsafe {
                    w
                        // Enable PWM Mode 0 for channel 0 and 1
                        .ch0comctl().bits(0b110)
                        .ch1comctl().bits(0b110)

                        // Output mode for channel 0 and 1
                        .ch0ms().bits(0b00)
                        .ch1ms().bits(0b00)
                });
                self.timer.chctl1_output().modify(|_r, w| unsafe {
                    w
                        // Enable PWM Mode 0 for channel 2 and 3
                        .ch2comctl().bits(0b110)
                        .ch3comctl().bits(0b110)

                        // Output mode for channel 2 and 3
                        .ch2ms().bits(0b00)
                        .ch3ms().bits(0b00)
                });

                // Enable the timer
                self.timer.ctl0.write(|w| {
                    w
                        .updis().clear_bit()
                        .cen().set_bit()
                });
            }
        }
    }
}

advanced_pwm_timer! {TIMER0: timer0}
general_pwm_timer!  {TIMER1: timer1}
general_pwm_timer!  {TIMER2: timer2}
general_pwm_timer!  {TIMER3: timer3}
general_pwm_timer!  {TIMER4: timer4}
