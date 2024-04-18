#![no_std]

use embedded_hal::i2c;

const I2C_ADDRESS: u8 = 0x44;

// Register map
const OPT4048_REGISTER_CHANNEL0: u8 = 0x00;
const OPT4048_REGISTER_CHANNEL0_EXTRA: u8 = 0x01;
const OPT4048_REGISTER_CHANNEL1: u8 = 0x02;
const OPT4048_REGISTER_CHANNEL1_EXTRA: u8 = 0x03;
const OPT4048_REGISTER_CHANNEL2: u8 = 0x04;
const OPT4048_REGISTER_CHANNEL2_EXTRA: u8 = 0x05;
const OPT4048_REGISTER_CHANNEL3: u8 = 0x06;
const OPT4048_REGISTER_CHANNEL3_EXTRA: u8 = 0x07;
const OPT4048_REGISTER_THRESHOLD_LOW: u8 = 0x08;
const OPT4048_REGISTER_THRESHOLD_HIGH: u8 = 0x09;
const OPT4048_REGISTER_CONTROLA: u8 = 0x0A;
const OPT4048_REGISTER_CONTROLB: u8 = 0x0B;
const OPT4048_REGISTER_FLAGS: u8 = 0x0C;

const OPT4048_REGISTER_DEVICE_ID: u8 = 0x11;

// CIE Matrix
const M0X: f32 = 0.000234892992;
const M0Y: f32 = 0.0000407467441;
const M0Z: f32 = 0.0000928619404;
const M1X: f32 = -0.0000189652390;
const M1Y: f32 = 0.000198958202;
const M1Z: f32 = -0.0000169739553;
const M2X: f32 = 0.0000120811684;
const M2Y: f32 = -0.0000158848115;
const M2Z: f32 = 0.000674021520;
const M1L: f32 = 0.00215; //lux

//MANTISSA_CHx=(RESULT_MSB_CHx<<8) + RESULT_LSB_CHx
//RESULT_MSB_CHx register carries the most significant 12 bits of the MANTISSA_CHx
//RESULT_LSB_CHx register carries the least significant 8 bits of the MANTISSA_CHx.
//MANTISSA_CHx is then computed using the above equations to get the 20 bit number.
//EXPONENT_CHx is directly read from the register which is 4 bits.
//ADC_CODES_CHx = (MANTISSA_CHx<<EXPONENT_CHx)
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
where
    I2C: i2c::I2c,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_device_id(&mut self) -> Result<[u8; 2], OPT4048Error<I2C::Error>> {
        let mut id = [0, 0];
        self.i2c
            .write_read(I2C_ADDRESS, &[OPT4048_REGISTER_DEVICE_ID], &mut id)
            .map_err(OPT4048Error::I2C)?;
        Ok(id)
    }

    pub fn read_lux(&mut self) -> Result<f32, OPT4048Error<I2C::Error>> {
        let mut block = [0u8; 4];
        self.i2c
            .write_read(I2C_ADDRESS, &[OPT4048_REGISTER_CHANNEL1], &mut block)
            .map_err(OPT4048Error::I2C)?;
          
        let exponent_ch1 = (block[0] & 0xF0) >> 4;
        let result_msb_ch1 = ((block[0] & 0x0F) as u16) << 8 | (block[1] as u16);
        let result_lsb_ch1 = block[2] as u16;
        let mantissa_ch1: u32 = result_msb_ch1 as u32 * 256 + result_lsb_ch1 as u32;
        let adc_ch1 = mantissa_ch1 << exponent_ch1;
        let lux = adc_ch1 as f32 * M1L;
        Ok(lux)
    }
  }
