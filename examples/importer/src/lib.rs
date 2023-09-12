#![cfg(not(test))]

pub use seq_macro::seq;

extern "C" {
    seq!{N in 0..1000 {
        fn add_~N(a: u32) -> u32;
    }}
}

pub fn addition1(a: u32) -> u32 {
    let mut a = a;
    seq!{N in 0..500 {
        a = unsafe { add_~N(a) };
    }};
    a
}

pub fn addition2(a: u32) -> u32 {
    let mut a = a;
    seq!{N in 500..1000 {
        a = unsafe { add_~N(a) };
    }};
    a
}
