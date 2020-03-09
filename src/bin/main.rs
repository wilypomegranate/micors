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

static PERIPH_BASE: u32 = 0x40000000u32;
static AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x00020000u32;
static RCC_BASE: u32 = AHBPERIPH_BASE + 0x00001000u32;
static mut RCC_APB2ENR: u32 = RCC_BASE + 0x18u32;
static APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x00010000u32;
static GPIOC_BASE: u32 = APB2PERIPH_BASE + 0x00001000u32;
static mut GPIOC_CRH: u32 = GPIOC_BASE + 0x4u32;
static mut GPIOC_BSRR: u32 = GPIOC_BASE + 0x10u32;

#[entry]
fn main() -> ! {
    // Enable clock for GPIOC.
    unsafe {
        let rcc_apb2enr = &mut (RCC_APB2ENR) as *mut u32;
        core::ptr::write_volatile(rcc_apb2enr, 0x0u32 | 0x10u32);
    }

    // Setup as 10Mhz output.
    unsafe {
        let gpioc_crh = &mut (GPIOC_CRH) as *mut u32;
        core::ptr::write_volatile(gpioc_crh, 0x44444444u32 | 0x100000u32);
    }

    // Set PC13 to write.
    unsafe {
        let gpioc_bsrr = &mut (GPIOC_BSRR) as *mut u32;
        core::ptr::write_volatile(gpioc_bsrr, 0x2000u32);
    }

    hprintln!("Hello, world!").unwrap();

    loop {}
}
