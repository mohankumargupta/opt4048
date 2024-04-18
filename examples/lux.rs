use opt4048::OPT4048;
use linux_embedded_hal as hal;
use hal::I2cdev;

fn main() {
  let i2cbus = I2cdev::new("/dev/i2c-1").unwrap();  
  let mut opt4048 = OPT4048::new(i2cbus);
  let device_id = opt4048.read_device_id().unwrap();
  let id: u16 = device_id[0] as u16 * 256 + device_id[1] as u16;
  println!("0: {} 1: {} 2: {}", device_id[0], device_id[1], id);
  let lux = opt4048.read_lux().unwrap();
  println!("lux: {}", lux);
}