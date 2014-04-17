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

    let val: u32 = func << idx;
    let mask: u32 = 0x7 << idx;

    let old_func = gpio_read(reg);
    let new_func = (old_func & !mask) | val;

    gpio_write(reg, new_func);
}

fn gpio_set(pin: int, val: u32) {
    if pin > 53 { return; }

    let idx: u32 = 1 << (pin & 0x1f);
    let reg = (pin >> 5) +
        match val {
            0 => 10,
            _ =>  7
        };
    gpio_write(reg, idx);
}

/* System Timer */
static SYSTEM_TIMER_COUNTER_LO: *u32 = 0x20003004 as *u32;

fn system_timer_timestamp_lo() -> u32 {
    unsafe {
        volatile_load(SYSTEM_TIMER_COUNTER_LO)
    }
}



/* delay is in microseconds. */
fn usleep(delay: u32) {
    let start = system_timer_timestamp_lo();
    let mut elapsed = 0;
    while elapsed < delay {
        elapsed = system_timer_timestamp_lo() - start;
    }
}

/* */
#[no_mangle]
#[no_split_stack]
pub fn main() {
    gpio_set_func(STATUS_LED_N, 1);

    loop {
        gpio_set(STATUS_LED_N, 0);
        usleep(500000);
        gpio_set(STATUS_LED_N, 1);
        usleep(250000);
    }
}
