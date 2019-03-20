pub use platform::*;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DylibHandle(pub usize);

pub type Address = usize;

#[cfg(unix)]
mod platform {
    use super::{Address, DylibHandle};
    use crate::Error;
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_int, c_void};

    const RTLD_LAZY: c_int = 0x0001;
    #[cfg(target_os = "linux")]
    const RTLD_GLOBAL: c_int = 0x0100;
    #[cfg(target_os = "macos")]
    const RTLD_GLOBAL: c_int = 0x0008;

    #[link(name = "dl")]
    extern "C" {
        fn dlopen(filename: *const c_char, flag: c_int) -> DylibHandle;
        fn dlsym(raw_handle: *const c_void, symbol: *const c_char) -> Address;
        fn dlerror() -> *const c_char;
    }

    pub unsafe fn load_library(path: &CStr) -> Result<DylibHandle, Error> {
        let handle = dlopen(path.as_ptr() as *const c_char, RTLD_GLOBAL | RTLD_LAZY);
        if handle.0 == 0 {
            Err(format!("{:?}", CStr::from_ptr(dlerror())).into())
        } else {
            Ok(handle)
        }
    }

    pub unsafe fn find_symbol(handle: DylibHandle, name: &CStr) -> Result<Address, Error> {
        let ptr = dlsym(handle.0 as *const c_void, name.as_ptr() as *const c_char);
        if ptr == 0 {
            Err(format!("{:?}", CStr::from_ptr(dlerror())).into())
        } else {
            Ok(ptr)
        }
    }
}

#[cfg(windows)]
mod platform {
    use super::{Address, DylibHandle};
    use crate::Error;
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_void};

    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryA(filename: *const c_char) -> DylibHandle;
        fn GetProcAddress(raw_handle: *const c_void, symbol: *const c_char) -> Address;
        fn GetLastError() -> u32;
    }

    pub unsafe fn load_library(path: &CStr) -> Result<DylibHandle, Error> {
        let handle = LoadLibraryA(path.as_ptr() as *const c_char);
        if handle.0 == 0 {
            Err(format!("Could not load {:?} (err=0x{:08X})", path, GetLastError()).into())
        } else {
            Ok(handle)
        }
    }

    pub unsafe fn find_symbol(handle: DylibHandle, name: &CStr) -> Result<Address, Error> {
        let ptr = GetProcAddress(handle.0 as *const c_void, name.as_ptr() as *const c_char);
        if ptr == 0 {
            Err(format!("Could not find {:?} (err=0x{:08X})", name, GetLastError()).into())
        } else {
            Ok(ptr)
        }
    }
}
