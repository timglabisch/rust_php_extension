#![feature(libc)]
#[macro_use] extern crate lazy_static;

extern crate libc;
mod zend_module;
mod zend_module_size;
use zend_module::*;
use libc::{c_void, c_int, c_char, c_uchar, c_uint, c_short};

//extern {
    //fn zend_parse_parameters(num_args: c_int, type_spce: *const c_char, ...);
//}

#[no_mangle]
pub extern "C" fn zif_confirm_rust_a_star_compiled(_: c_int, _: *mut zval, _: *mut *mut zval, _: *mut zval, _: c_int) {
//    panic!("yay");
//    println!("yes!");
}

#[no_mangle]
pub extern "C" fn zm_generic_rust_a_star(_: c_int, _: c_int) -> c_int {
    //panic!("yay");
    1
}

#[no_mangle]
pub extern "C" fn zm_info_rust_a_star(_: *const zend_module_entry) { }

lazy_static! {
    static ref ZEND_ENTRY_SIZE: c_short = ::std::mem::size_of::<zend_module_entry>() as c_short;
}

#[no_mangle]
pub extern "C" fn get_module(_: c_void) -> *const zend_module_entry {
    use std::mem;

    struct Foo;

    static rust_a_star_functions: [zend_function_entry;2] = [
        zend_function_entry {
            fname:    b"confirm_rust_a_star_compiled\0" as *const [u8] as *const i8,
            handler:  Some(zif_confirm_rust_a_star_compiled),
            arg_info: 0 as *const _zend_arg_info,
            num_args: 0,
            flags:    0
         },
         zend_function_entry {
            fname:    0 as *const c_char,
            handler:  None,
            arg_info: 0 as *const _zend_arg_info,
            num_args: 0,
            flags:    0
         }
    ];

    let size : c_short = *ZEND_ENTRY_SIZE as c_short;

    static module_entry: &'static zend_module_entry = &zend_module_entry {
        size:       zend_module_size::ZEND_MODULE_SIZE,
        zend_api:   20160303,
        zend_debug: 0,
        zts:        0,
        ini_entry:  0 as *const _zend_ini_entry,
        deps:       0 as *const _zend_module_dep,
        name:       b"rust_a_star\0" as *const [u8] as *const i8,
        functions:  &rust_a_star_functions as *const [zend_function_entry] as *const zend_function_entry,

        module_startup_func:   Some(zm_generic_rust_a_star),
        module_shutdown_func:  Some(zm_generic_rust_a_star),
        request_startup_func:  Some(zm_generic_rust_a_star),
        request_shutdown_func: Some(zm_generic_rust_a_star),
        info_func:             Some(zm_info_rust_a_star),

        version:              b"0.1.0\0" as *const [u8] as *const i8,
        globals_size:         0,
        globals_ptr:          0 as *const c_void,
        globals_ctor:         None,
        globals_dtor:         None,
        post_deactivate_func: None,
        module_started:       0,
        _type:                0,
        handle:               0 as *const c_void,
        module_number:        0,
        build_id:             b"API20160303,NTS,debug\0" as *const [u8] as *const i8
    };

    module_entry
}
