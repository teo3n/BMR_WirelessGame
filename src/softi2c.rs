use gd32vf103xx_hal::pac;
use gd32vf103xx_hal::gpio::gpiob;


pub struct SoftwareI2c
{

}

impl SoftwareI2c
{
	pub fn new() -> Self
	{
		return SoftwareI2c
		{

		};
	}

	pub fn write(&self, address: u8, write_buffer: &[u8])
	{

	}

	pub fn read(&self, address: u8, read_buffer: &mut [u8])
	{

	}
}