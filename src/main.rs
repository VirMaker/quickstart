#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm, peripheral};
use cortex_m_rt::entry;
use sam3x8e::{Peripherals};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let delay = 3000000;
    unsafe {
        peripherals.PIOB.per.write_with_zero(|w| w.p27().set_bit());
        peripherals.PIOB.oer.write_with_zero(|w| w.p27().set_bit());
        loop {
            peripherals.PIOB.sodr.write_with_zero(|w| w.p27().set_bit());
            asm::delay(delay);
            peripherals.PIOB.codr.write_with_zero(|w| w.p27().set_bit());
            asm::delay(delay);

        }
    }
}
