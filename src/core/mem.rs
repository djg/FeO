// IronOxide - Toy OS in Rust
// Copyright 2014 Dan Glastonbury <dan.glastonbury@gmail.com>

/*
extern "rust-intrinsic" {
    pub fn volatile_load<T>(src: *T) -> T;
    pub fn volatile_store<T>(dst: *mut T, val: T);
}

#[no_split_stack]
pub fn get32(addr: uint) -> u32 {
    unsafe { volatile_load(addr as *u32) }
}

#[no_split_stack]
pub fn put32(addr: uint, data: u32) {
    unsafe { volatile_store(addr as *mut u32, data); }
}
*/

/* General volatile read/write */
#[allow(ctypes)]
extern {
    fn _dummy(x: uint);
    fn _put32(addr: uint, data: u32);
    fn _get32(addr: uint) -> u32;
}


#[no_split_stack]
#[inline(always)]
pub fn get32(addr: uint) -> u32 {
    unsafe {
        _get32(addr)
    }
}

#[no_split_stack]
#[inline(always)]
pub fn put32(addr: uint, data: u32) {
    unsafe {
        _put32(addr, data);
    }
}

#[no_split_stack]
#[inline(always)]
pub fn dummy(x: uint) {
    unsafe {
        _dummy(x);
    }
}
