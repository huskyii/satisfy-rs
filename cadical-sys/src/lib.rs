use std::os::raw::{c_char, c_int, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CCaDiCaL {
    _unused: [u8; 0],
}

extern "C" {
    pub fn ccadical_signature() -> *const c_char;

    pub fn ccadical_init() -> *mut CCaDiCaL;
    pub fn ccadical_release(arg1: *mut CCaDiCaL);

    pub fn ccadical_add(arg1: *mut CCaDiCaL, lit: c_int);
    pub fn ccadical_assume(arg1: *mut CCaDiCaL, lit: c_int);

    pub fn ccadical_solve(arg1: *mut CCaDiCaL) -> c_int;

    pub fn ccadical_val(arg1: *mut CCaDiCaL, lit: c_int) -> c_int;
    pub fn ccadical_failed(arg1: *mut CCaDiCaL, lit: c_int) -> c_int;

    pub fn ccadical_set_terminate(
        arg1: *mut CCaDiCaL,
        state: *mut c_void,
        terminate: Option<unsafe extern "C" fn(state: *mut c_void) -> c_int>,
    );

    pub fn ccadical_set_option(arg1: *mut CCaDiCaL, name: *const c_char, val: c_int);
    pub fn ccadical_get_option(arg1: *mut CCaDiCaL, name: *const c_char) -> c_int;

    pub fn ccadical_limit(arg1: *mut CCaDiCaL, name: *const c_char, limit: c_int);

    pub fn ccadical_print_statistics(arg1: *mut CCaDiCaL);
    pub fn ccadical_active(arg1: *mut CCaDiCaL) -> i64;
    pub fn ccadical_irredundant(arg1: *mut CCaDiCaL) -> i64;
    pub fn ccadical_fixed(arg1: *mut CCaDiCaL, lit: c_int) -> c_int;
    pub fn ccadical_terminate(arg1: *mut CCaDiCaL);
    pub fn ccadical_freeze(arg1: *mut CCaDiCaL, lit: c_int);
    pub fn ccadical_frozen(arg1: *mut CCaDiCaL, lit: c_int) -> c_int;
    pub fn ccadical_melt(arg1: *mut CCaDiCaL, lit: c_int);
    pub fn ccadical_simplify(arg1: *mut CCaDiCaL) -> c_int;
}
