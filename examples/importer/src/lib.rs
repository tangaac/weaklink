#![cfg(not(test))]

pub use seq_macro::seq;

extern "C" {
    seq!{N in 0..10 {
        pub fn add_~N(a: u32) -> u32;
    }}

    pub fn get_SOMEDATA() -> *const i32;
}

pub fn addition1(a: u32) -> u32 {
    let mut a = a;
    seq!{N in 0..5 {
        a = unsafe { add_~N(a) };
    }};
    a
}

pub fn addition2(a: u32) -> u32 {
    let mut a = a;
    seq!{N in 5..10 {
        a = unsafe { add_~N(a) };
    }};
    a
}
