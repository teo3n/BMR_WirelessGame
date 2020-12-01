/*
 * 	Teo Niemirepo
 *  teo.niemirepo@tuni.fi
 *
 * 	26.11.2020
 *
 * 	an abstraction for handling all 
 * 	nunchuk -related functionality
 */

/*
 *	TODO: return errors when necessary
 */

pub use gd32vf103xx_hal as hal;
use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::pac::{ I2C0 };
use gd32vf103xx_hal::gpio::{ Alternate, OpenDrain };
use gd32vf103xx_hal::i2c::{BlockingI2c, Mode};
use gd32vf103xx_hal::gpio::gpiob::{ PB8, PB9 };
use gd32vf103xx_hal::rcu::Rcu;
use gd32vf103xx_hal::afio::Afio;
use gd32vf103xx_hal::prelude::*;
use gd32vf103xx_hal::delay::McycleDelay;
use embedded_hal::blocking::delay::DelayMs;

/// contains the controller data neatly formatted
pub struct ControllerInput
{
	pub joy_x: i8,
	pub joy_y: i8,
	pub accel_x: i16,
	pub accel_y: i16,
	pub accel_z: i16,
	pub btn_z: u8,
	pub btn_c: u8,
}

pub struct Nunchuk<'a>
{
	i2c_handle: BlockingI2c<I2C0, (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>)>,
	rcu: &'a mut Rcu,
}

// public methods
impl<'a> Nunchuk<'a>
{
	/// creates a new nunchuk -object on the longan nano
	/// pins <8, 9> (scl, sda)
	pub fn new(
		afio: &mut Afio,
		rcu: &'a mut Rcu,
		i2c0: I2C0,
		scl: PB8<Alternate<OpenDrain>>,
		sda: PB9<Alternate<OpenDrain>>
	) -> Self
	{
		let mut nchuk = Nunchuk
		{
			i2c_handle: BlockingI2c::i2c0 (
		        i2c0,
		        (scl, sda),
		        afio,
		        Mode::standard(100.khz()),
		        rcu,
		        998,
		        1,
		        998,
		        998
		    ),
		    rcu
		};

		let mut delay = McycleDelay::new(&nchuk.rcu.clocks);
    	delay.delay_ms(1);

		nchuk.init_nunchuk();

		return nchuk;
	}

	/// reads from the nunchuk over i2c and returns
	/// the controller data in a nearly formatted structure
	pub fn get_input(&mut self) -> ControllerInput
	{
		let buffer = self.read_input();
		return self.decode_input(buffer);
	}
}

// private methods
impl Nunchuk<'_>
{
	/// initializes the nunchuk internal registers
	fn init_nunchuk(&mut self)
	{
		self.i2c_handle.write(0x52, &[0x40, 0x00]).unwrap();
	}

	/// reads 6 bytes from the nunchuk and then
	/// sends 0x00 to the nunchuk to indicate "prepare to send more data"
	fn read_input(&mut self) -> [u8; 6]
	{
		let mut read_buffer: [u8; 6] = [0; 6];
        self.i2c_handle.read(0x52, &mut read_buffer).unwrap();

        // xor the entire buffer element wise by 0.17.
        // why? I don't think even nintendo knows
        for i in 0..6
        {
        	let current_byte = read_buffer[i];
        	read_buffer[i] = (current_byte ^ 0x17) + 0x17
        }

        let mut delay = McycleDelay::new(&self.rcu.clocks);
    	delay.delay_us(100);

        self.i2c_handle.write(0x52, &[0x00]).unwrap();

		return read_buffer;
	}

	/// decode 
	fn decode_input(&self, buffer: [u8; 6]) -> ControllerInput
	{
		let mut btn_c: u8 = 1;
		let mut btn_z: u8 = 1;
		let mut accel_x: i16 = buffer[2] as i16;
		let mut accel_y: i16 = buffer[3] as i16;
		let mut accel_z: i16 = buffer[4] as i16;

		/*
		 * 	TODO:
		 * 		make sure the accel values are correctly parsed
		 */

		// decode the buttons and accel_ LSB from 
		// the last byte		
		if ((buffer[5] >> 0) & 0x01) != 0
		{
			btn_z = 0;
		}
		if ((buffer[5] >> 1) & 0x01) != 0
		{
			btn_c = 0;
		}
		if ((buffer[5] >> 2) & 0x01) != 0
		{
			accel_x += 1;
		}
		if ((buffer[5] >> 3) & 0x01) != 0
		{
			accel_x += 2;
		}
		if ((buffer[5] >> 4) & 0x01) != 0
		{
			accel_y += 1;
		}
		if ((buffer[5] >> 5) & 0x01) != 0
		{
			accel_y += 2;
		}
		if ((buffer[5] >> 6) & 0x01) != 0
		{
			accel_z += 1;
		}
		if ((buffer[5] >> 7) & 0x01) != 0
		{
			accel_z += 2;
		}

		return ControllerInput
		{
			joy_x: (buffer[0] as i16 - 127) as i8,
			joy_y: (buffer[1] as i16 - 127) as i8,
			accel_x,
			accel_y,
			accel_z,
			btn_z,
			btn_c,
		}
	}
}

