// IronOxide - Toy OS in Rust
// Copyright 2014 Dan Glastonbury <dan.glastonbury@gmail.com>

// Raspberry Pi - Broadcom BCM2835 General Purpose I/O (GPIO)
// see Ch. 6, "BCM2835 ARM Peripherals" for documentation

// GPIO Registers
pub static GPFSEL0: uint = 0x20200000; // GPIO Function Select 0 - 32 R/W
pub static GPFSEL1: uint = 0x20200004; // GPIO Function Select 1 - 32 R/W
pub static GPFSEL2: uint = 0x20200008; // GPIO Function Select 2 - 32 R/W
pub static GPFSEL3: uint = 0x2020000C; // GPIO Function Select 3 - 32 R/W
pub static GPFSEL4: uint = 0x20200010; // GPIO Function Select 4 - 32 R/W
pub static GPFSEL5: uint = 0x20200014; // GPIO Function Select 5 - 32 R/W
pub static GPSET0:  uint = 0x2020001C; // GPIO Pin Output Set 0 - 32 W
pub static GPSET1:  uint = 0x20200020; // GPIO Pin Output Set 1 - 32 W
pub static GPCLR0:  uint = 0x20200028; // GPIO Pin Output Clear 0 - 32 W
pub static GPCLR1:  uint = 0x2020002C; // GPIO Pin Output Clear 1 - 32 W
pub static GPLEV0:  uint = 0x20200034; // GPIO Pin Level 0 - 32 R
pub static GPLEV1:  uint = 0x20200038; // GPIO Pin Level 1 - 32 R
pub static GPEDS0:  uint = 0x20200040; // GPIO Pin Event Detect Status 0 - 32 R/W
pub static GPEDS1:  uint = 0x20200044; // GPIO Pin Event Detect Status 1 - 32 R/W
pub static GPREN0:  uint = 0x2020004C; // GPIO Pin Rising Edge Detect Enable 0 - 32 R/W
pub static GPREN1:  uint = 0x20200050; // GPIO Pin Rising Edge Detect Enable 1 - 32 R/W
pub static GPFEN0:  uint = 0x20200058; // GPIO Pin Falling Edge Detect Enable 0 - 32 R/W
pub static GPFEN1:  uint = 0x2020005C; // GPIO Pin Falling Edge Detect Enable 1 - 32 R/W
pub static GPHEN0:  uint = 0x20200064; // GPIO Pin High Detect Enable 0 - 32 R/W
pub static GPHEN1:  uint = 0x20200068; // GPIO Pin High Detect Enable 1 - 32 R/W
pub static GPLEN0:  uint = 0x20200070; // GPIO Pin Low Detect Enable 0 - 32 R/W
pub static GPLEN1:  uint = 0x20200074; // GPIO Pin Low Detect Enable 1 - 32 R/W
pub static GPAREN0: uint = 0x2020007C; // GPIO Pin Async. Rising Edge Detect 0 - 32 R/W
pub static GPAREN1: uint = 0x20200080; // GPIO Pin Async. Rising Edge Detect 1 - 32 R/W
pub static GPAFEN0: uint = 0x20200088; // GPIO Pin Async. Falling Edge Detect 0 - 32 R/W
pub static GPAFEN1: uint = 0x2020008C; // GPIO Pin Async. Falling Edge Detect 1 - 32 R/W
pub static GPPUD:   uint = 0x20200094; // GPIO Pin Pull-up/down Enable - 32 R/W
pub static GPPUDCLK0: uint = 0x20200098; // GPIO Pin Pull-up/down Enable Clock 0 - 32 R/W
pub static GPPUDCLK1: uint = 0x2020009C; // GPIO Pin Pull-up/down Enable Clock 1 - 32 R/W

//
pub static GPFSEL_IN:   u32 = 0x0;
pub static GPFSEL_OUT:  u32 = 0x1;
pub static GPFSEL_ALT0: u32 = 0x4;
pub static GPFSEL_ALT1: u32 = 0x5;
pub static GPFSEL_ALT2: u32 = 0x6;
pub static GPFSEL_ALT3: u32 = 0x7;
pub static GPFSEL_ALT4: u32 = 0x3;
pub static GPFSEL_ALT5: u32 = 0x2;

extern "rust-intrinsic" {
    fn offset<T>(dst: *T, offset: int) -> *T;
    fn volatile_load<T>(src: *T) -> T;
    fn volatile_store<T>(dst: *mut T, val: T);
}

#[inline(always)]
fn gpio_read(reg: int) -> u32 {
    unsafe {
        volatile_load(offset(GPFSEL0 as *u32, reg))
    }
}

#[inline(always)]
fn gpio_write(reg: int, val: u32) {
    unsafe {
        volatile_store(offset(GPFSEL0 as *u32, reg) as *mut u32, val);
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

#[no_split_stack]
pub fn set_func(pin: int, func: u32) {
    if pin < 0  || pin > 53 { return; }
    if func < 0 || func > 7 { return; }

    let (reg, idx) = gpio_reg_idx_from(pin);

    let shift = 3 * idx;
    let fsel: u32 = func << shift;
    let mask: u32 = 0x7  << shift;

    let old_fsel = gpio_read(reg);
    let new_fsel = (old_fsel & !mask) | fsel;

    gpio_write(reg, new_fsel);
}

#[no_split_stack]
pub fn set(pin: int, val: u32) {
    if pin < 0 || pin > 53 { return; }

    let idx: u32 = 1 << (pin & 0x1f);
    let reg = (pin >> 5) +
        match val {
            0 => 0xA,
            _ => 0x7
        };
    gpio_write(reg, idx);
}
