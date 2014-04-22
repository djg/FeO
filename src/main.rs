// Iron Oxide
// Copyright 2014, Dan Glastonbury <dan.glastonbury@gmail.com>
#![feature(asm)]
#![no_std]
#![no_main]

use core::mem;
use mach::{gpio, miniuart};

mod core;
mod mach;

/* System Timer */
pub static SYSTEM_TIMER_COUNTER_LO: uint = 0x20003004;

fn system_timer_timestamp_lo() -> u32 {
    mem::get32(SYSTEM_TIMER_COUNTER_LO)
}

/* delay is in microseconds. */
fn usleep(delay: u32) {
    let start = system_timer_timestamp_lo();
    let mut elapsed: u32 = 0;
    while elapsed < delay {
        elapsed = system_timer_timestamp_lo() - start;
    }
}

static GPIO_STATUS_LED_N: int = 16;

/* */
#[no_mangle]
#[no_split_stack]
pub extern "C" fn main() {
    // Turn on OK LED. (pin low is on)
    gpio::set_func(GPIO_STATUS_LED_N, gpio::GPFSEL_OUT);
    gpio::set(GPIO_STATUS_LED_N, 0);

    // Configure mini UART
    miniuart::init();

    let mut ra = 0;
    loop {
        miniuart::putc(0x30 + (ra & 0x7));
        if (ra & 0x7) == 0x7 {
            miniuart::putc(0x0D);
            miniuart::putc(0x0A);
        }
        ra += 1;
    }
}
