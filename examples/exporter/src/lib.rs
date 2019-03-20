utils::repeat!(N in {
    #[no_mangle]
    pub extern "C" fn add_~N(a: u32) -> u32 {
        a + N
    }
});
