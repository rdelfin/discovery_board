#![deny(unsafe_code)]
#![no_main]
#![no_std]

use disc_aux::{entry, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = disc_aux::itm_init();

    iprintln!(&mut itm.stim[0], "Hello, world!");
    panic!("Goodbye, world :(");

    loop {}
}
