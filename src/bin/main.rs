//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![no_std]
#![no_main]

// makes `panic!` print messages to the host stderr using semihosting
// extern crate panic_semihosting;

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate

    hprintln!("Hello, world!").unwrap();

    // Enable clock for GPIOC.
    let rcc_apb2enr = &mut (0x18) as *mut i32;
    unsafe {
        let cur_val = core::ptr::read_volatile(rcc_apb2enr);
        core::ptr::write_volatile(rcc_apb2enr, cur_val | 0x10);
    }

    loop {}
}
