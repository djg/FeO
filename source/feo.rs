/* Iron Oxide */
#![feature(asm)]
#![no_std]

/* GPIO controller */
static GPIO_BASE: uint = 0x20200000;
static STATUS_LED_N: u32 = 16;

fn gpio_read(reg: u32) -> u32 {
    let mut result : u32;
    unsafe {
        asm!("LDR $0, [$1, $2, LSL #2]"
             : "=r"(result) : "r"(GPIO_BASE), "r"(reg) :: "volatile");
    }
    result
}

fn gpio_write(reg: u32, val: u32) {
    unsafe {
        asm!("STR $0, [$1, $2, LSL #2]"
             :: "r"(val), "r"(GPIO_BASE), "r"(reg) :: "volatile");
    }
}

// GPIO setup macros. Always use INP_GPIO(x) before using OUT_GPIO(x) or SET_GPIO_ALT(x,y)
fn gpio_inp(p: u32) {
    if p > 53 {
        return;
    }
    let mut pin = p;
    let mut reg = 0;
    while pin > 9 {
        pin -= 10;
        reg += 1;
    }
    let mask = !(0x7 << (3 * pin));
    gpio_write(reg, gpio_read(reg) & mask);
}

fn gpio_out(p: u32) {
    let mut pin = p;
    let mut reg = 0;
    while pin > 9 {
        pin -= 10;
        reg += 1;
    }
    let val = 1 << (3 * pin);
    gpio_write(reg, gpio_read(reg) | val);

}

fn gpio_set(pin: u32) {
    if pin > 53 {
        return;
    }

    let reg = 7 + (pin >> 5);
    let val = 1 << (pin & 32);
    gpio_write(reg, val);
}

fn gpio_clr(pin: u32) {
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
