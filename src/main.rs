#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::{asm};
use cortex_m_rt::entry;
use sam3x8e::{interrupt};

const RTT_MR:u32 = 0x400e_1a30;
const RTT_SR:u32 = 0x400e_1a3C;
const PB_SODR:u32 = 0x400e_1030;


#[entry]
fn main() -> ! {
    //let p = sam3x8e::Peripherals::take().unwrap();
    //let c = sam3x8e::CorePeripherals::take().unwrap();
    //let mut core = cortex_m::Peripherals::take().unwrap();
    let timer_inc_interrupt: u32 = 1 << 17;
    let divider = 55; //approx 1ms
    reg_write(RTT_MR, timer_inc_interrupt | divider);
    unsafe { sam3x8e::NVIC::unmask(interrupt::RTT); }
    

    let pb_per = 0x400e_1000;
    let pb_oer = 0x400e_1010;
    let pb_codr = 0x400e_1034;
    let led_pin = 1 << 27;

    reg_write(pb_per, led_pin);
    reg_write(pb_oer, led_pin);
    loop {
        reg_write(pb_codr, led_pin);
        sleep(1000);
        reg_write(PB_SODR, led_pin);
        sleep(1000);
    }

}

fn sleep(ms: usize) {
    for _ in 0..ms {
        asm::wfe();
    }
}

fn reg_write(addr: u32, value:u32) {
    let ptr = addr as *mut u32;
    unsafe { *ptr = value; }
}

#[interrupt]
fn RTT() {
    sam3x8e::NVIC::mask(interrupt::RTT);
    unsafe { core::ptr::read_volatile(RTT_SR as *const u32); }
    unsafe { sam3x8e::NVIC::unmask(interrupt::RTT); }

    asm::sev();
}
