/* Iron Oxide */
#![feature(asm)]
#![no_std]

extern "rust-intrinsic" {

    pub fn offset<T>(dst: *T, offset: int) -> *T;
    pub fn volatile_load<T>(src: *T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);

}

/* GPIO controller */
static GPIO_BASE: *u32 = 0x20200000 as *u32;
static STATUS_LED_N: int = 16;

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
    let mut reg: int = pin;
    let mut idx: int = 0;
    while reg > 9 {
        reg -= 10;
        idx += 1;
    }
    (reg, idx)
}

// GPIO setup macros. Always use INP_GPIO(x) before using OUT_GPIO(x) or SET_GPIO_ALT(x,y)
fn gpio_inp(pin: int) {
    if pin > 53 { return; }

    let (reg, idx) = gpio_reg_idx_from(pin);
    let mask = !(0x7 << (3 * idx));
    gpio_write(reg, gpio_read(reg) & mask);
}

fn gpio_out(pin: int) {
    if pin > 53 { return; }

    let (reg, idx) = gpio_reg_idx_from(pin);
    let val = 1 << (3 * idx);
    gpio_write(reg, gpio_read(reg) | val);
}

fn gpio_set(pin: int) {
    if pin > 53 { return; }

    let reg = 7 + (pin >> 5);
    let val = 1 << (pin & 32);
    gpio_write(reg, val);
}

fn gpio_clr(pin: int) {
    if pin > 53 {
        return;
    }

    let reg = 10 + (pin >> 5);
    let val = 1 << (pin & 0x1f);
    gpio_write(reg, val);
}

// Failure
/*
#[lang="fail_"]
pub fn fail(_: *i8, _: *i8, _: uint) -> ! {
    unsafe {
        abort()
    }
}
*/


#[no_mangle]
#[no_split_stack]
pub fn main() {
    gpio_inp(STATUS_LED_N);
    gpio_out(STATUS_LED_N);
    gpio_clr(STATUS_LED_N);

    loop {
    }
}
