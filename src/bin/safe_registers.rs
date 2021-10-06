#![no_main]
#![no_std]

use disc_aux::{entry, iprintln, ITM};

#[entry]
fn main() -> ! {
    let gpioe = disc_aux::registers_init();
    disc_aux::itm_init();

    gpioe.bsrr.write(|w| w.bs9().set_bit());

    gpioe.bsrr.write(|w| w.bs11().set_bit());

    gpioe.bsrr.write(|w| w.br9().set_bit());

    gpioe.bsrr.write(|w| w.br11().set_bit());

    loop {}
}
