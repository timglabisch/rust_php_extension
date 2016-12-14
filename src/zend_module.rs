use libc::{c_void, c_int, c_char, c_uchar, c_uint, c_short};

#[repr(C)]
pub struct zval;

#[repr(C)]
pub struct _zend_arg_info;
#[repr(C)]
pub struct _zend_ini_entry;
#[repr(C)]
pub struct _zend_module_dep;

// https://github.com/php/php-src/blob/6053987bc27e8dede37f437193a5cad448f99bce/Zend/zend_modules.h#L73
#[repr(C)]
pub struct zend_module_entry {
    pub size:       c_short,
    pub zend_api:   c_uint,
    pub zend_debug: c_char,
    pub zts:        c_char,
    pub ini_entry:  *const _zend_ini_entry,
    pub deps:       *const _zend_module_dep,
    pub name:       *const c_char,

    pub functions: *const zend_function_entry,
    pub module_startup_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    pub module_shutdown_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    pub request_startup_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    pub request_shutdown_func: Option<extern "C" fn(c_int, c_int) -> c_int>,
    pub info_func: Option<extern "C" fn(*const zend_module_entry)>,
    pub version: *const c_char,
    pub globals_size: usize,
    pub globals_ptr: *const c_void,
    pub globals_ctor: Option<extern "C" fn(*const c_void)>,
    pub globals_dtor: Option<extern "C" fn(*const c_void)>,
    pub post_deactivate_func: Option<extern "C" fn(c_void) -> c_int>,
    pub module_started: c_int,
    pub _type: c_uchar,
    pub handle: *const c_void,
    pub module_number: c_int,
    pub build_id: *const c_char
}

unsafe impl Sync for zend_module_entry {}

#[repr(C)]
pub struct zend_function_entry {
    pub fname:    *const c_char,
    pub handler:  Option<extern "C" fn(c_int, *mut zval, *mut *mut zval, *mut zval, c_int)>,
    pub arg_info: *const _zend_arg_info,
    pub num_args: c_uint,
    pub flags:    c_uint
}

unsafe impl Sync for zend_function_entry {}