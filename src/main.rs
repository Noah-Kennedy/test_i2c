extern crate rppal;
use rppal::i2c;
use std::collections::VecDeque;

type Message = [u8; 8];

fn main() {
    let mut i2c = Box::new(i2c::I2c::new().expect("Failed to open I2C"));
    i2c.set_slave_address(17).expect("Failed to set the slave address");
    
    loop {
        let message: Message = [1, 2, 3, 4, 5, 6, 7, 7];
        let mut queue = VecDeque::new();
    
        i2c.write(& message).expect("Failed to write");
        
        loop {
            let received = i2c.smbus_receive_byte().expect("Failed to receive byte");
            
            queue.push_back(received);
            
            if queue.len() >= 8 {
                break;
            }
        }
        
        println!("{:?}", queue);
        
        while !queue.is_empty() {
            queue.pop_front();
        }
    }
}

