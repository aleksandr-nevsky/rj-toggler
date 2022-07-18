use std::{thread, time};

use clap::Parser;
use gpio_cdev::{Chip, LineRequestFlags};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    pin: u32,

    #[clap(short, long, value_parser)]
    duration_millis: u64,
}

fn main() {
    let args = Args::parse();
    let duration = time::Duration::from_millis(args.duration_millis);
    // Read the state of GPIO4 on a raspberry pi.  /dev/gpiochip0
    // maps to the driver for the SoC (builtin) GPIO controller.
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    // let output = chip.get_line(4).unwrap();
    let output = chip.get_line(args.pin).unwrap();
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "toggle").unwrap();

    output_handle.set_value(1).unwrap();
    thread::sleep(duration);
    output_handle.set_value(0).unwrap();
}
