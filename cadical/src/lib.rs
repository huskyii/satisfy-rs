use cadical_sys::*;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Not;
use std::os::raw::{c_int, c_void};
use std::ptr;
use std::time::{Duration, Instant};

pub trait Terminator {
    fn terminate(&mut self) -> bool;
}

#[derive(Debug, Hash)]
pub struct DummyTerminator();

impl Terminator for DummyTerminator {
    fn terminate(&mut self) -> bool {
        false
    }
}

#[allow(non_upper_case_globals)]
pub const NoneTerminator: Option<DummyTerminator> = None::<DummyTerminator>;

#[derive(Debug, Hash)]
pub struct TimeoutTerminator {
    timeout: Duration,
    time_start: Instant,
}

impl TimeoutTerminator {
    pub fn new(timeout: Duration) -> TimeoutTerminator {
        TimeoutTerminator {
            timeout,
            time_start: Instant::now(),
        }
    }
}

impl Terminator for TimeoutTerminator {
    fn terminate(&mut self) -> bool {
        let elapsed = Instant::now() - self.time_start;
        elapsed > self.timeout
    }
}

// keep code here as we may want to check mem leak
// impl Drop for TimeoutTerminator {
//     fn drop(&mut self) {
//         eprintln!("Drop TimeoutTerminator")
//     }
// }

// valid literal require lit != INT_MIN
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Lit(i32);

impl Lit {
    pub fn new(i: i32) -> Option<Lit> {
        if i == i32::min_value() || i == 0 {
            None
        } else {
            Some(Lit(i))
        }
    }

    pub fn get(self) -> i32 {
        self.0
    }

    /// the underlying variable
    pub fn var(self) -> u32 {
        if self.0 > 0 {
            self.0 as u32
        } else {
            -self.0 as u32
        }
    }
}

impl Not for Lit {
    type Output = Lit;
    fn not(self) -> Lit {
        Lit(-self.0)
    }
}

pub type Clause = Vec<Lit>;

#[derive(Debug, Hash)]
struct CadicalPtr(*mut CCaDiCaL);

impl Drop for CadicalPtr {
    fn drop(&mut self) {
        unsafe {
            ccadical_release(self.0);
        }
    }
}

#[derive(Debug, Hash)]
struct TerminatorState {
    state: *mut c_void,
    release_func: unsafe fn(*mut c_void) -> (),
}

impl Drop for TerminatorState {
    fn drop(&mut self) {
        let func = self.release_func;
        unsafe {
            func(self.state);
        }
    }
}

// type state pattern
#[derive(Debug, Hash)]
pub struct Cadical<S: State> {
    ptr: CadicalPtr,
    terminator_state: Option<TerminatorState>,
    marker: PhantomData<S>,
}

pub fn new() -> Option<Cadical<Configuring>> {
    let ptr = unsafe { ccadical_init() };
    if ptr.is_null() {
        None
    } else {
        Some(Cadical {
            ptr: CadicalPtr(ptr),
            terminator_state: None,
            marker: PhantomData,
        })
    }
}

impl<S: State> Cadical<S> {
    pub fn raw_ptr(&self) -> *mut CCaDiCaL {
        self.ptr.0
    }

    pub fn set_terminator<T: Terminator>(&mut self, terminator: Option<T>) {
        unsafe extern "C" fn terminate<T: Terminator>(state: *mut c_void) -> c_int {
            let terminator = &mut *(state as *mut T);
            terminator.terminate() as c_int
        }
        unsafe {
            match terminator {
                None => {
                    ccadical_set_terminate(self.ptr.0, ptr::null_mut(), None);
                    self.terminator_state = None;
                }
                Some(terminator) => {
                    let boxed_terminator = Box::new(terminator);
                    let state = Box::into_raw(boxed_terminator) as *mut c_void;
                    unsafe fn release<T>(state: *mut c_void) {
                        Box::<T>::from_raw(state as *mut T);
                    }
                    self.terminator_state = Some(TerminatorState {
                        state,
                        release_func: release::<T>,
                    });
                    ccadical_set_terminate(self.ptr.0, state, Some(terminate::<T>))
                }
            }
        }
    }

    pub fn add_clause<'a>(self, clause: impl Iterator<Item = &'a Lit>) -> Cadical<Unknown> {
        for lit in clause {
            unsafe {
                ccadical_add(self.ptr.0, lit.get() as c_int);
            }
        }
        unsafe {
            ccadical_add(self.ptr.0, 0);
        }
        Cadical {
            ptr: self.ptr,
            terminator_state: self.terminator_state,
            marker: PhantomData,
        }
    }

    /// expose low-level api may benefit performance sometimes
    #[inline]
    pub fn add_lit(self, lit: i32) -> Cadical<Unknown> {
        unsafe { ccadical_add(self.ptr.0, lit as c_int) };
        Cadical {
            ptr: self.ptr,
            terminator_state: self.terminator_state,
            marker: PhantomData,
        }
    }

    pub fn assume(self, lit: Lit) -> Cadical<Unknown> {
        unsafe {
            ccadical_assume(self.ptr.0, lit.get() as c_int);
        }
        Cadical {
            ptr: self.ptr,
            terminator_state: self.terminator_state,
            marker: PhantomData,
        }
    }

    pub fn solve(self) -> Result {
        let res = unsafe { ccadical_solve(self.ptr.0) };
        match res {
            0 => Result::Unknown(Cadical {
                ptr: self.ptr,
                terminator_state: self.terminator_state,
                marker: PhantomData,
            }),
            10 => Result::Sat(Cadical {
                ptr: self.ptr,
                terminator_state: self.terminator_state,
                marker: PhantomData,
            }),
            20 => Result::Unsat(Cadical {
                ptr: self.ptr,
                terminator_state: self.terminator_state,
                marker: PhantomData,
            }),
            _ => unreachable!(),
        }
    }

    pub fn signature() -> &'static str {
        unsafe {
            // trust cadical's implementation
            CStr::from_ptr(ccadical_signature()).to_str().unwrap()
        }
    }

    pub fn print_statistics(&self) {
        unsafe { ccadical_print_statistics(self.ptr.0) }
    }

    pub fn active(&self) -> i64 {
        unsafe { ccadical_active(self.ptr.0) }
    }

    pub fn irredundant(&self) -> i64 {
        unsafe { ccadical_irredundant(self.ptr.0) }
    }
}

impl Cadical<Configuring> {
    pub fn finish(self) -> Cadical<Unknown> {
        Cadical {
            ptr: self.ptr,
            terminator_state: self.terminator_state,
            marker: PhantomData,
        }
    }
}

impl Cadical<Sat> {
    pub fn val(&self, lit: Lit) -> bool {
        let lit = lit.get() as c_int;
        let res = unsafe { ccadical_val(self.ptr.0, lit) };
        res > 0
    }
}

impl Cadical<Unsat> {
    pub fn failed(&self, lit: Lit) -> bool {
        let lit = lit.get() as c_int;
        let res = unsafe { ccadical_failed(self.ptr.0, lit) };
        res != 0
    }
}

#[derive(Debug, Hash)]
pub enum Result {
    Unknown(Cadical<Unknown>),
    Sat(Cadical<Sat>),
    Unsat(Cadical<Unsat>),
}

// sealed trait pattern
pub trait State: private::Sealed {}

#[derive(Debug, Hash)]
pub enum Configuring {}
#[derive(Debug, Hash)]
pub enum Unknown {}
#[derive(Debug, Hash)]
pub enum Sat {}
#[derive(Debug, Hash)]
pub enum Unsat {}

impl State for Configuring {}
impl State for Unknown {}
impl State for Sat {}
impl State for Unsat {}

mod private {
    // sealed trait pattern
    pub trait Sealed {}

    impl Sealed for super::Configuring {}
    impl Sealed for super::Unknown {}
    impl Sealed for super::Sat {}
    impl Sealed for super::Unsat {}

    impl<S: super::State> Sealed for super::Cadical<S> {}
}
