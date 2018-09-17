extern crate rppal;
use rppal::i2c;

fn main() {
    let mut i2c = i2c::I2c::new().expect("Failed to create i2c");
    i2c.write("hello".as_bytes());
    std::thread::sleep_ms(200);
    let mut input:  [u8; 5] = [0, 0, 0, 0, 0];
    i2c.read(& mut input);
    
    for c in input.iter() {
        println!("{}", c);
    }
}
