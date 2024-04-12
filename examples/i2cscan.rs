use opt4048::OPT4048;
use linux_embedded_hal as hal;
use hal::I2cdev;

fn main() {
  let i2cbus = I2cdev::new("/dev/i2c-1").unwrap();  
  let mut opt4048 = OPT4048::new(i2cbus);
  let device_id = opt4048.read_device_id().unwrap();
  println!("id: {}", device_id);

}