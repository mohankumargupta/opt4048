use std::time::Duration;

use hal::nb::block;
use hal::I2cdev;
use hal::{CountDown, SysTimer};
use linux_embedded_hal as hal;
use opt4048::OPT4048;

fn main() {
    let i2cbus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut timer = hal::SysTimer::new();
    let mut opt4048 = OPT4048::new(i2cbus);
    let device_id = opt4048.read_device_id().unwrap();
    let id: u16 = device_id[0] as u16 * 256 + device_id[1] as u16;
    println!("0: {} 1: {} 2: {}", device_id[0], device_id[1], id);
    loop {
        let lux = opt4048.read_lux().unwrap();
        println!("lux: {}", lux);
        timer.start(Duration::from_millis(1000)).unwrap();
        block!(timer.wait()).unwrap();
    }
}
