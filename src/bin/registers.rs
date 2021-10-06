#![no_main]
#![no_std]

use disc_aux::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    disc_aux::registers_init();
    disc_aux::itm_init();

    unsafe {
        // Magic address
        const GPIOE_BSRR: u32 = 0x48001018;

        // Turn on the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << 9;

        // Turn on the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << 11;

        // Turn off the "North" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (9 + 16);

        // Turn off the "East" LED
        *(GPIOE_BSRR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}
