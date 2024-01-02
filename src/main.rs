mod devices;
use devices::Device;
use devices::DEVICE_LIST;
use std::env::args;
use std::io::{self, Write};

use crate::devices::device_by_str;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        // prompt user about devices available
        for (i, dev_name) in DEVICE_LIST.into_iter().enumerate() {
            println!("{}: {}", i, dev_name);
        }

        println!("**NOTE** you can also run `vboox <device name>` to avoid list");
        let mut dev = loop {
            // take input
            print!("Choose a your device [{}-{}]: ", 0, DEVICE_LIST.len() - 1);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let choice: usize = match input.trim().parse() {
                Ok(choice) => choice,
                Err(e) => {
                    println!("{}: Enter a valid integer.", e);
                    continue;
                }
            };

            match device_by_str(DEVICE_LIST[choice]) {
                Some(dev) => break dev,
                None => continue,
            };
        };

        dev.try_connect().unwrap();
        dev.fetch_events();

        return;
    }

    let mut dev = device_by_str(args[1].as_str()).expect("input should be a valid device");
    dev.try_connect().unwrap();
    dev.fetch_events();
}
