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

use micrors::RCC;
use micrors::GPIOC;

#[entry]
fn main() -> ! {
    // Enable clock for GPIOC.
    unsafe {
        (*RCC::ptr()).apb2enr.write(|w| w.bits(0x0u32 | 0x10u32));
    }

    // Setup as 10Mhz output.
    unsafe {
        (*GPIOC::ptr()).crh.write(|w| w.bits(0x44444444u32 | 0x100000u32));
    }

    // Set PC13 to write.
    unsafe {
        (*GPIOC::ptr()).brr.write(|w| w.bits(0x2000u32));
    }

    hprintln!("Hello, world!").unwrap();

    loop {}
}
