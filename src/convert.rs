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

use crate::{ADCCodes, CIExy};

pub(crate) fn convert_raw_to_adc(block: [u8; 4]) -> u32 {
    let exponent = (block[0] & 0xF0) >> 4;
    let result_msb = ((block[0] & 0x0F) as u16) << 8 | (block[1] as u16);
    let result_lsb = block[2] as u16;
    let mantissa: u32 = result_msb as u32 * 256 + result_lsb as u32;
    mantissa << exponent
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_X(adc: &ADCCodes) -> f32 {
    (adc.ch0 as f32) * M0X + (adc.ch1 as f32) * M0Y + (adc.ch2 as f32) * M0Z
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_Y(adc: &ADCCodes) -> f32 {
    (adc.ch0 as f32) * M1X + (adc.ch1 as f32) * M1Y + (adc.ch2 as f32) * M1Z
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_Z(adc: &ADCCodes) -> f32 {
    (adc.ch0 as f32) * M2X + (adc.ch1 as f32) * M2Y + (adc.ch2 as f32) * M2Z
}

pub(crate) fn convert_channel1_to_lux(adc: u32) -> f32 {
    adc as f32 * M1L
}

/*
pub(crate) fn convert_adc_to_lux(adc: ADCCodes) -> f32 {
    adc.ch1 as f32 * M1L
}
*/

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_cie_xy(adc: ADCCodes) -> CIExy {
    let X = convert_adc_to_X(&adc);
    let Y = convert_adc_to_Y(&adc);
    let Z = convert_adc_to_Z(&adc);
    let total = X + Y + Z;
    CIExy {
        x: X / total,
        y: Y / total,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_adc() {
        let mut block = [0u8; 4];
        block[0] = 0x01;
        block[1] = 0x59;
        block[2] = 0x80;
        block[3] = 0xFC;
        assert_eq!(88448, convert_raw_to_adc(block));
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_adc_Y() {
        let mut block = [0u8; 4];
        block[0] = 0x00;
        block[1] = 0xA2;
        block[2] = 0x48;
        block[3] = 0x09;
        let adc_ch0 = convert_raw_to_adc(block);

        block[0] = 0x01;
        block[1] = 0x59;
        block[2] = 0x80;
        block[3] = 0xFC;
        let adc_ch1 = convert_raw_to_adc(block);

        block[0] = 0x00;
        block[1] = 0x55;
        block[2] = 0x54;
        block[3] = 0xF5;
        let adc_ch2 = convert_raw_to_adc(block);
        let adc = ADCCodes::new(adc_ch0, adc_ch1, adc_ch2, 0);
        let X = convert_adc_to_X(&adc);
        let Y = convert_adc_to_Y(&adc);
        let Z = convert_adc_to_Z(&adc);
        assert_eq!(15.39f32, (X * 100.0).round() / 100.0);
        assert_eq!(16.44f32, (Y * 100.0).round() / 100.0);
        assert_eq!(13.82f32, (Z * 100.0).round() / 100.0);

        let ciexy = convert_adc_to_cie_xy(adc);
        assert_eq!(0.337, (ciexy.x * 1000.0).round() / 1000.0);
        assert_eq!(0.360, (ciexy.y * 1000.0).round() / 1000.0);
    }
}

/*

The parameter values of the light source read by using the Minolta spectrometer CL-500 are:

X=183.2
Y=198.9
Z=183.7
Lux=198.9
CIEx=0.3238
CIEy=0.3515

Read the data through the sensor 00h-07h as follows:

REG[0] = 0x00A2
REG[1] = 0x4809
REG[2] = 0x0159
REG[3] = 0x80FC
REG[4] = 0x0055
REG[5] = 0x54F5
REG[6] = 0x0473
REG[7] = 0xECFF

Convert to decimal to
CH[0] = 41544
CH[1] = 88448
CH[2] = 21844
CH[3] = 291820

The XYZ values that result from the matrix (I got 15.39, 16.44, 13.82) are not scaled and are typically just used as an intermediate step in CCT calculation. To get the proper XYZ values, scale XYZ up by the factor that makes Y equal to lux. Therefore you would scale 16.44 up by 11.57 in order to have Y = Lux. Then the XYZ values would be 178, 190, 160.

By the way, CIEx = X/(X+Y+Z), so XYZ doesn't need to be scaled to get to CIEx, but the above will hopefully help you understand how to scale if you are interested in the proper XYZ values.

From this, I got CIEx = 0.337 and CIEy = 0.360.

In terms of CCT (formula I used for CCT is below), there is a 9% difference between the 4048 and the color meter.

n = (CIEx-0.3320)/(0.1858-CIEy);
CCT = 437*n^3 + 3601*n^2 + 6861*n + 5517

*/
