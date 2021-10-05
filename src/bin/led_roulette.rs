#![deny(unsafe_code)]
#![no_main]
#![no_std]

use disc_aux::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = disc_aux::led_init();

    let mut half_period = 50_u16;
    let v_half_period = Volatile::new(&mut half_period);

    let led_order = [0, 1, 2, 3, 4, 5, 6, 7];
    let mut idx = 0;

    loop {
        let next_idx = (idx + 1) % led_order.len();

        leds[led_order[next_idx]].on().ok();
        delay.delay_ms(v_half_period.read());

        leds[led_order[idx]].off().ok();
        delay.delay_ms(v_half_period.read());

        idx = next_idx;
    }
}
