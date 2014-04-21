/* Iron Oxide */
#![feature(asm)]
#![no_std]
#![no_main]

extern "rust-intrinsic" {

    pub fn offset<T>(dst: *T, offset: int) -> *T;
    pub fn volatile_load<T>(src: *T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);

}

/* General volatile read/write */
#[allow(ctypes)]
extern {
    fn _dummy(x: uint);
    fn _put32(addr: uint, data: u32);
    fn _get32(addr: uint) -> u32;
}

fn get32(addr: uint) -> u32 {
    unsafe {
        _get32(addr)
    }
}

fn put32(addr: uint, data: u32) {
    unsafe {
        _put32(addr, data);
    }
}

fn dummy(x: uint) {
    unsafe {
        _dummy(x);
    }
}


/* GPIO controller */
pub static GPIO_BASE: *u32 = 0x20200000 as *u32;
pub static GPPUD: uint = 0x20200094; // GPIO Pin Pull-up/down Enable
pub static GPPUDCLK0: uint = 0x20200098; // GPIO Pin Pull-up/down Enable Clock 0
pub static GPPUDCLK1: uint = 0x2020009C; // GPIO Pin Pull-up/down Enable Clock 1

pub static GPIO_FUNC_INPUT: u32 = 0x0;
pub static GPIO_FUNC_OUTPUT: u32 = 0x1;
pub static GPIO_FUNC_ALT_0: u32 = 0x4;
pub static GPIO_FUNC_ALT_1: u32 = 0x5;
pub static GPIO_FUNC_ALT_2: u32 = 0x6;
pub static GPIO_FUNC_ALT_3: u32 = 0x7;
pub static GPIO_FUNC_ALT_4: u32 = 0x3;
pub static GPIO_FUNC_ALT_5: u32 = 0x2;

fn gpio_read(reg: int) -> u32 {
    unsafe {
        volatile_load(offset(GPIO_BASE, reg))
    }
}

fn gpio_write(reg: int, val: u32) {
    unsafe {
        volatile_store(offset(GPIO_BASE, reg) as *mut u32, val);
    }
}

fn gpio_reg_idx_from(pin: int) -> (int, int) {
    let mut idx: int = pin;
    let mut reg: int = 0;
    while idx > 9 {
        idx -= 10;
        reg += 1;
    }
    (reg, idx)
}

fn gpio_set_func(pin: int, func: u32) {
    if pin < 0 || pin > 53 { return; }
    if func < 0 || func > 7 { return; }

    let (reg, idx) = gpio_reg_idx_from(pin);

    let val: u32 = func << (3 * idx);
    let mask: u32 = 0x7 << (3 * idx);

    let old_func = gpio_read(reg);
    let new_func = (old_func & !mask) | val;

    gpio_write(reg, new_func);
}

fn gpio_set(pin: int, val: u32) {
    if pin < 0 || pin > 53 { return; }

    let idx: u32 = 1 << (pin & 0x1f);
    let reg = (pin >> 5) +
        match val {
            0 => 0xa,
            _ => 0x7
        };
    gpio_write(reg, idx);
}

/* System Timer */
pub static SYSTEM_TIMER_COUNTER_LO: uint = 0x20003004;

fn system_timer_timestamp_lo() -> u32 {
    get32(SYSTEM_TIMER_COUNTER_LO)
}

/* delay is in microseconds. */
fn usleep(delay: u32) {
    let start = system_timer_timestamp_lo();
    let mut elapsed: u32 = 0;
    while elapsed < delay {
        elapsed = system_timer_timestamp_lo() - start;
    }
}

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

//GPIO14 TXD0 and TXD1
//GPIO15 RXD0 and RXD1
//alt function 5 for uart1
//alt function 0 for uart0
//((250,000,000/115200)/8)-1 = 270

pub static UART0_TXD: u32 = GPIO_FUNC_ALT_0;
pub static UART1_TXD: u32 = GPIO_FUNC_ALT_5;
static GPIO_PIN_14: int = 14;
static GPIO_STATUS_LED_N: int = 16;

/* */
#[no_mangle]
#[no_split_stack]
pub extern "C" fn main() {
    // Turn on OK LED. (pin low is on)
    gpio_set_func(GPIO_STATUS_LED_N, GPIO_FUNC_OUTPUT);
    gpio_set(GPIO_STATUS_LED_N, 0);

    // Configure mini UART
    put32(AUX_ENABLES, 1);
    put32(AUX_MU_IER_REG, 0);
    put32(AUX_MU_CNTL_REG, 0);
    put32(AUX_MU_LCR_REG, 3);
    put32(AUX_MU_MCR_REG, 0);
    put32(AUX_MU_IER_REG, 0);
    put32(AUX_MU_IIR_REG, 0xC6); // 0b11000110
    put32(AUX_MU_BAUD_REG, 270);

    // Set Pin P1-08 - GPIO 14 to UART1_TXD
    gpio_set_func(GPIO_PIN_14, UART1_TXD);

    // Disable internal pull-up/down registers on transmit pin
    put32(GPPUD, 0x0); // Off – disable pull-up/down ..
    let mut ra: uint;
    ra = 0u; while ra < 150u { dummy(ra); ra += 1; } // Wait at least 150 cycles
    put32(GPPUDCLK0, 1 << GPIO_PIN_14); // .. for pin 14
    ra = 0u; while ra < 150u { dummy(ra); ra += 1; } // Wait at least 150 cycles
    put32(GPPUDCLK0, 0);

    // Enable mini UART transmit
    put32(AUX_MU_CNTL_REG, 0x2); // 0b10 - Transmitter enable

    let mut ra = 0;
    loop {
        loop {
            if (get32(AUX_MU_LSR_REG) & 0x20) != 0 { break; }
        }

        put32(AUX_MU_IO_REG, 0x30 + (ra & 0x7));
        ra += 1;
    }
}
