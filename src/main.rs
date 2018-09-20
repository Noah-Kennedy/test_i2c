extern crate rppal;
use rppal::i2c;

fn main() {
    let mut i2c = Box::new(i2c::I2c::new().expect("Failed to open I2C"));
    i2c.set_slave_address(17).expect("Failed to set the slave address");
    
    loop {
        let rec = i2c.smbus_receive_byte().expect("Failed to receive byte");
        println!("{:?}", rec)
    }
}