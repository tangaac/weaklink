pub use seq_macro::seq;

seq! {N in 0..10 {
    #[no_mangle]
    pub extern "C" fn add_~N(a: u32) -> u32 {
        a + N
    }
}}

#[no_mangle]
pub static SOMEDATA: i32 = 123;
