extern crate rppal;
use rppal::i2c;

fn main() {
    let mut i2c = i2c::I2c::new().expect("Failed to create i2c");
    i2c.set_slave_address(13).expect("Failed to set slave address");
    let output = [1, 2, 3, 4, 5];
    
    let mut input = [0, 0, 0, 0, 0];
    i2c.write_read(&output, &mut input).expect("Failed to write/read");
    
    for c in input.iter() {
        println!("{}", c);
    }
}
