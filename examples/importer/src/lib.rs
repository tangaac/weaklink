#![cfg(not(test))]

#[no_mangle]
pub fn addition(a: u32) -> u32 {
    extern "C" {
        utils::repeat! {N in {
            fn add_~N(a: u32) -> u32;
        }}
    }

    let mut a = a;
    utils::repeat!(N in {
        a = unsafe { add_~N(a) };
    });
    a
}
