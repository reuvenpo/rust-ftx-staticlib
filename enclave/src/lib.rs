#![feature(sgx_platform)]

use std::os::fortanix_sgx::arch::{
    egetkey,
    Align512,
};

#[no_mangle] pub extern "C" fn x_add(x: i32, y: i32) -> i32 {
    println!("{} + {} = {}; {}", x, y, x + y, egetkey(&Align512([0; 512])).unwrap().0[0]);
    x + y
}
