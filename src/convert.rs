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

use crate::ADCCodes;

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_X(adc: ADCCodes) -> f32 {
    (adc.ch0 as f32) * M0X + (adc.ch1 as f32) * M1X + (adc.ch2 as f32) * M2X
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_Y(adc: ADCCodes) -> f32 {
    (adc.ch0 as f32) * M0Y + (adc.ch1 as f32) * M1Y + (adc.ch2 as f32) * M2Y
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_toZ(adc: ADCCodes) -> f32 {
    (adc.ch0 as f32) * M0Z + (adc.ch1 as f32) * M1Z + (adc.ch2 as f32) * M2Z
}

#[allow(non_snake_case)]
pub(crate) fn convert_channel1_to_lux(adc: u32) -> f32 {
    adc as f32 * M1L
}

#[allow(non_snake_case)]
pub(crate) fn convert_adc_to_lux(adc: ADCCodes) -> f32 {
    adc.ch1 as f32 * M1L
}
