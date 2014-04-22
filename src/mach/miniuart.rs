// Iron Oxide
// Copyright 2014 Dan Glastonbury <dan.glastonbury@gmail.com>

use core::mem;
use mach::gpio;

/* mini-UART */
pub static AUX_ENABLES: uint = 0x20215004; // Auxiliary enables
pub static AUX_MU_IO_REG: uint = 0x20215040; // Mini UART I/O Data
pub static AUX_MU_IER_REG: uint = 0x20215044; // Mini UART Interrupt Enable
pub static AUX_MU_IIR_REG: uint = 0x20215048; // Mini UART Interrupt Identify
pub static AUX_MU_LCR_REG: uint = 0x2021504C; // Mini UART Line Control
pub static AUX_MU_MCR_REG: uint = 0x20215050; // Mini UART Modem Control
pub static AUX_MU_LSR_REG: uint = 0x20215054; // Mini UART Line Status
pub static AUX_MU_MSR_REG: uint = 0x20215058; // Mini UART Modem Status
pub static AUX_MU_SCRATCH: uint = 0x2021505C; // Mini UART Scratch
pub static AUX_MU_CNTL_REG: uint = 0x20215060; // Mini UART Extra Control
pub static AUX_MU_STAT_REG: uint = 0x20215064; // Mini UART Extra Status
pub static AUX_MU_BAUD_REG: uint = 0x20215068; // Mini UART Baudrate

// GPIO14 TXD0 and TXD1
// GPIO15 RXD0 and RXD1
// alt function 5 for uart1
// alt function 0 for uart0
// Baudrate settings - ((250,000,000/115200)/8)-1 = 270

pub static UART0_TXD: u32 = gpio::GPFSEL_ALT0;
pub static UART1_TXD: u32 = gpio::GPFSEL_ALT5;
static GPIO_PIN_14: int = 14;
static GPIO_STATUS_LED_N: int = 16;

pub fn init () {
    // Configure mini UART
    mem::put32(AUX_ENABLES, 1);
    mem::put32(AUX_MU_IER_REG, 0);
    mem::put32(AUX_MU_CNTL_REG, 0);
    mem::put32(AUX_MU_LCR_REG, 3);
    mem::put32(AUX_MU_MCR_REG, 0);
    mem::put32(AUX_MU_IER_REG, 0);
    mem::put32(AUX_MU_IIR_REG, 0xC6); // 0b11000110
    mem::put32(AUX_MU_BAUD_REG, 270);

    // Set Pin P1-08 - GPIO 14 to UART1_TXD
    gpio::set_func(GPIO_PIN_14, UART1_TXD);

    // Disable internal pull-up/down registers on transmit pin
    mem::put32(gpio::GPPUD, 0x0); // Off – disable pull-up/down ..
    let mut ra: uint;
    ra = 0u; while ra < 150u { mem::dummy(ra); ra += 1; } // Wait at least 150 cycles
    mem::put32(gpio::GPPUDCLK0, 1 << GPIO_PIN_14); // .. for pin 14
    ra = 0u; while ra < 150u { mem::dummy(ra); ra += 1; } // Wait at least 150 cycles
    mem::put32(gpio::GPPUDCLK0, 0);

    // Enable mini UART transmit
    mem::put32(AUX_MU_CNTL_REG, 0b10); // 0b10 - Transmitter enable
}

pub fn putc(ch: uint) {
    loop {
        if mem::get32(AUX_MU_LSR_REG) & 0x20 != 0 { break; }
        mem::put32(AUX_MU_IO_REG, ch as u32 & 0xFF);
    }
}
