#![feature(libc)]
extern crate libc;
use std::io::prelude::*;
use std::fs::File;

include!("src/zend_module.rs");

fn main() {

    let mut f = File::create("src/zend_module_size.rs").expect("could not open zend_module_size.rs");
    f.write_all(
        &format!("pub const ZEND_MODULE_SIZE : i16 = {};", std::mem::size_of::<zend_module_entry>()).as_bytes()
    ).expect("could not write size file");
}