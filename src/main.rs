use std::io::Write;

use heapless::String as HString;
use serialport;

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in &ports {
        println!("{}", p.port_name);
    }

    let port = ports.first().unwrap();
    println!("Using port: {}", port.port_name);

    let mut tx = serialport::new(&port.port_name, 57_600)
        .timeout(std::time::Duration::from_millis(10))
        .open()
        .expect("Failed to open port");
    let mut rx = tx.try_clone().unwrap();

    let mut res_buffer = String::new();
    loop {
        let mut buf = [0u8; 1];

        match rx.read(&mut buf) {
            Ok(count) => {
                // stdout.write(&buf[..count]).unwrap();
                // stdout.flush().unwrap();
                let character = buf[..count][0] as char;

                // Each message string ends with a divide character
                if character == '÷' {
                    println!("{}", res_buffer);

                    continue;
                }

                // Add the character to the res_buffer
                res_buffer.push(character);
            }
            Err(e) => {
                assert!(e.kind() == std::io::ErrorKind::TimedOut);
            }
        }
    }
}
