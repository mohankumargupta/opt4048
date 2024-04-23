#![no_std]

use embedded_hal::i2c;

const I2C_ADDRESS: u8 = 0x44;

#[repr(u8)]
pub enum RegisterMap {
    Channel0 = 0x00,
    Channel1 = 0x02,
    Channel2 = 0x04,
    Channel3 = 0x06,
    ControlA = 0x0A,
    ControlB = 0x0B,
    Flags = 0x0C,
    DeviceID = 0x11,
}

/*
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
*/

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

pub struct OPT4048<I2C> {
    i2c: I2C,
}

#[derive(Debug)]
pub enum OPT4048Error<E> {
    I2C(E),
}

struct ADCCodes {
    ch0: u32,
    ch1: u32,
    ch2: u32,
    ch3: u32,
}

impl ADCCodes {
    fn new(ch0: u32, ch1: u32, ch2: u32, ch3: u32) -> Self {
        Self { ch0, ch1, ch2, ch3 }
    }
}

pub struct XYZ {
    x: f32,
    y: f32,
    z: f32,
}

pub struct CIExyz {
    x: f32,
    y: f32,
    z: f32,
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
            .write_read(I2C_ADDRESS, &[RegisterMap::DeviceID as u8], &mut id)
            .map_err(OPT4048Error::I2C)?;
        Ok(id)
    }

    fn adc(&mut self, block: [u8; 4]) -> u32 {
        let exponent = (block[0] & 0xF0) >> 4;
        let result_msb = ((block[0] & 0x0F) as u16) << 8 | (block[1] as u16);
        let result_lsb = block[2] as u16;
        let mantissa: u32 = result_msb as u32 * 256 + result_lsb as u32;
        mantissa << exponent
    }

    fn read_channel(&mut self, register: RegisterMap) -> Result<u32, OPT4048Error<I2C::Error>> {
        let mut block = [0u8; 4];
        self.i2c
            .write_read(I2C_ADDRESS, &[register as u8], &mut block)
            .map_err(OPT4048Error::I2C)?;
        let adc_code = self.adc(block);
        Ok(adc_code)
    }

    fn read_all_channels(
        &mut self,
        register: RegisterMap,
    ) -> Result<ADCCodes, OPT4048Error<I2C::Error>> {
        let mut block = [0u8; 16];
        self.i2c
            .write_read(I2C_ADDRESS, &[register as u8], &mut block)
            .map_err(OPT4048Error::I2C)?;
        let adc_ch0 = self.adc([block[0], block[1], block[2], block[3]]);
        let adc_ch1 = self.adc([block[4], block[5], block[6], block[7]]);
        let adc_ch2 = self.adc([block[8], block[9], block[10], block[11]]);
        let adc_ch3 = self.adc([block[12], block[13], block[14], block[15]]);
        Ok(ADCCodes::new(adc_ch0, adc_ch1, adc_ch2, adc_ch3))
    }

    // We these ones, XYZ are scaled so that Y=lux
    pub fn read_XYZ() {}
    pub fn read_XYZ_Lux() {}
    pub fn read_CIExy() {}
    pub fn read_CIExyz() {}
    pub fn read_CIExyz_Lux() {}

    // XYZ are not scaled, the values comes from applying CIE matrix to
    // adc codes
    pub fn read_XYZ_Unscaled() {}

    pub fn read_lux(&mut self) -> Result<f32, OPT4048Error<I2C::Error>> {
        /*
        let mut block = [0u8; 4];
        self.i2c
            .write_read(I2C_ADDRESS, &[OPT4048_REGISTER_CHANNEL1], &mut block)
            .map_err(OPT4048Error::I2C)?;

        let exponent_ch1 = (block[0] & 0xF0) >> 4;
        let result_msb_ch1 = ((block[0] & 0x0F) as u16) << 8 | (block[1] as u16);
        let result_lsb_ch1 = block[2] as u16;
        let mantissa_ch1: u32 = result_msb_ch1 as u32 * 256 + result_lsb_ch1 as u32;
        let adc_ch1 = mantissa_ch1 << exponent_ch1;
        */
        let adc_ch1 = self.read_channel(RegisterMap::Channel1)?;
        let lux = adc_ch1 as f32 * M1L;
        Ok(lux)
    }
}
