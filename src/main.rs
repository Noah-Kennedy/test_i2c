extern crate rppal;
use rppal::i2c;

const LENGTH: usize = 1;

fn main() {
    let mut i2c = Box::new(i2c::I2c::new().expect("Failed to open I2C"));
    i2c.set_slave_address(17).expect("Failed to set the slave address");
    //for i in 0..100000 {
    //    println!("Count: {}", i);
    //    i2c = echo(i2c);
    //}
    
    loop {
        let sending_payload = [0, 1, 2, 3, 4, 5, 6, 7];
        i2c.smbus_write_byte(7, 7).expect("Could not send");
        
        let mut receiving_payload = [0; 8];
        
        let num_bytes = i2c.read(& mut receiving_payload).expect("Could not read");
        
        for element in 0..num_bytes {
            println!("{}", receiving_payload[element]);
        }
    }
}

//fn echo(mut arduino: Box<i2c::I2c>) -> Box<i2c::I2c> {
//    let payload = [1, 2, 3, 4, 5, 6, 7, 8];
//
//    let mut data: [u8; LENGTH] = [0; LENGTH];
//
//    let mut queue = std::collections::VecDeque::new();
//
//    let mut input = [0; 8];
//
//    let stopwatch = std::time::Instant::now();
//
//    loop {
//        match arduino.write(&payload){
//            Ok(_) => break,
//            Err(_) => continue
//        }
//    }
//
//    while queue.len() < 8 {
//        loop {
//            match arduino.read(& mut data) {
//                Ok(num_bytes) => {
//                    for i in 0..num_bytes {
//                        queue.push_back(data[i]);
//                    }
//                    break;
//                }
//                Err(_) => {
//                    continue;
//                }
//            };
//        }
//    }
//
//    let elapsed_time_millis = stopwatch.elapsed().subsec_millis();
//
//    for i in 0..8 {
//        input[i] = queue.pop_front().expect("Error, queue empty");
//    }
//
//    //if input[0] != 1 {
//    //    panic!("input[0] != 1");
//    //}
//
//    println!("Input: {:?}", input);
//
//    println!("Time: {} millis", elapsed_time_millis);
//
//    arduino
//}