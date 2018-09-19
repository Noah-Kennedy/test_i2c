extern crate rppal;
use rppal::i2c;

const LENGTH: usize = 1;

fn main() {
    let mut i2c = Box::new(i2c::I2c::new().expect("Failed to open I2C"));
    i2c.set_slave_address(17).expect("Failed to set the slave address");
    
    loop {
        let mut receiving_payload = [0; 5];
        
        let num_bytes = i2c.read(& mut receiving_payload).expect("Failed to read bytes");
        
        if num_bytes > 0 {
            println!("{}", std::str::from_utf8(&receiving_payload).expect("Invalid utf 8"));
        }
    }
}