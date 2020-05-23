// A simple program that reads from a TUN device and prints the bytes of the IP
// packets sent to the TUN device.
use std::env;
use tun_tap::{Iface, Mode};

const MTU: usize = 1_500;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("TUN device name must be provided");
    }
    let device_name = &args[1];

    let iface =
        Iface::without_packet_info(device_name, Mode::Tun).expect("failed to create TUN device");

    let mut buf = vec![0; MTU];

    loop {
        match iface.recv(&mut buf) {
            Ok(bytes_read) => {
                print!("received {} bytes: ", bytes_read);
                for i in 0..bytes_read {
                    print!("{:x} ", &buf[i]);
                }
                println!();
            }
            Err(e) => panic!("read failed: {}", e),
        };
    }
}
