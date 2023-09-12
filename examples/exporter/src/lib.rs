pub use seq_macro::seq;

seq!{N in 0..1000 {
    #[no_mangle]
    pub extern "C" fn add_~N(a: u32) -> u32 {
        a + N
    }
}}
