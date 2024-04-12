#![no_std]

use embedded_hal::i2c;

const I2C_ADDRESS: u8 = 0x44;

// Register map
const OPT4048_DEVICE_ID: u8 = 0x11;

pub struct OPT4048<I2C> {
    i2c: I2C
}

#[derive(Debug)]
pub enum OPT4048Error<E> {
    /// I²C bus communication error
    I2C(E),
    /// Invalid input data provided
    InvalidInputData,
}

impl<I2C> OPT4048<I2C> 
where I2C: i2c::I2c,
{
  pub fn new(i2c: I2C) -> Self {
    Self{i2c}
  }

  pub fn read_device_id(&mut self) -> Result<u8, OPT4048Error<I2C::Error>> {
    let mut id = [0];
    self.i2c.write_read(I2C_ADDRESS,&[OPT4048_DEVICE_ID], &mut id).map_err(OPT4048Error::I2C)?;
    Ok(id[0])
  }

}

