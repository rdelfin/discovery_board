#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use disc_aux::{entry, iprintln, ITM};

// Print the current contents of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR =  0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    disc_aux::registers_init();
    let mut itm = disc_aux::itm_init();

    unsafe {
        // Magic address
        const GPIOE_BSRR: u32 = 0x48001018;

        // Print initial to get an idea
        iprint_odr(&mut itm);

        // Turn on the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        iprint_odr(&mut itm);

        // Turn on the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}
