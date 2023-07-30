#![feature(iter_collect_into)]

mod devices;

use devices::boox::BooxNoteAir2;

fn main() {
    let mut dev = BooxNoteAir2::new().unwrap();
    dev.try_connect().unwrap();
    dev.fetch_events();
    loop {}
}
