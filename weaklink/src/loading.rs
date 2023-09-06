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

    pub const RTLD_LAZY: c_int = 0x0001;
    pub const RTLD_NOW: c_int = 0x0002;

    #[cfg(target_os = "linux")]
    pub const RTLD_LOCAL: c_int = 0x0000;
    #[cfg(target_os = "macos")]
    pub const RTLD_LOCAL: c_int = 0x0004;

    #[cfg(target_os = "linux")]
    pub const RTLD_GLOBAL: c_int = 0x0100;
    #[cfg(target_os = "macos")]
    pub const RTLD_GLOBAL: c_int = 0x0008;

    #[link(name = "dl")]
    extern "C" {
        fn dlopen(filename: *const c_char, flag: c_int) -> DylibHandle;
        fn dlsym(raw_handle: *const c_void, symbol: *const c_char) -> Address;
        fn dlerror() -> *const c_char;
    }

    pub fn load_library_with_flags(path: &CStr, flags: c_int) -> Result<DylibHandle, Error> {
        unsafe {
            let handle = dlopen(path.as_ptr() as *const c_char, flags);
            if handle.0 == 0 {
                Err(format!("{:?}", CStr::from_ptr(dlerror())).into())
            } else {
                Ok(handle)
            }
        }
    }

    pub fn load_library(path: &CStr) -> Result<DylibHandle, Error> {
        load_library_with_flags(path, RTLD_LAZY | RTLD_GLOBAL)
    }

    pub fn find_symbol(handle: DylibHandle, name: &CStr) -> Result<Address, Error> {
        unsafe {
            let ptr = dlsym(handle.0 as *const c_void, name.as_ptr() as *const c_char);
            if ptr == 0 {
                Err(format!("{:?}", CStr::from_ptr(dlerror())).into())
            } else {
                Ok(ptr)
            }
        }
    }
}

#[cfg(windows)]
mod platform {
    use super::{Address, DylibHandle};
    use crate::Error;
    use std::ffi::CStr;
    use std::os::raw::{c_char, c_void};

    const LOAD_WITH_ALTERED_SEARCH_PATH: u32 = 0x00000008;
    const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: u32 = 0x00000200;
    const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: u32 = 0x00001000;
    const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: u32 = 0x00000100;
    const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x00000800;
    const LOAD_LIBRARY_SEARCH_USER_DIRS: u32 = 0x00000400;
    const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: u32 = 0x00000080;
    const LOAD_IGNORE_CODE_AUTHZ_LEVEL: u32 = 0x00000010;
    const LOAD_LIBRARY_SAFE_CURRENT_DIRS: u32 = 0x00002000;

    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryExA(filename: *const c_char, hfile: DylibHandle, flags: u32) -> DylibHandle;
        fn GetProcAddress(raw_handle: *const c_void, symbol: *const c_char) -> Address;
        fn GetLastError() -> u32;
    }

    pub fn load_library_ex(path: &CStr, flags: u32) -> Result<DylibHandle, Error> {
        unsafe {
            let handle = LoadLibraryExA(path.as_ptr() as *const c_char, DylibHandle(0), flags);
            if handle.0 == 0 {
                Err(format!("Could not load {:?} (err=0x{:08X})", path, GetLastError()).into())
            } else {
                Ok(handle)
            }
        }
    }

    pub fn load_library(path: &CStr) -> Result<DylibHandle, Error> {
        load_library_ex(path, LOAD_WITH_ALTERED_SEARCH_PATH)
    }

    pub fn find_symbol(handle: DylibHandle, name: &CStr) -> Result<Address, Error> {
        unsafe {
            let ptr = GetProcAddress(handle.0 as *const c_void, name.as_ptr() as *const c_char);
            if ptr == 0 {
                Err(format!("Could not find {:?} (err=0x{:08X})", name, GetLastError()).into())
            } else {
                Ok(ptr)
            }
        }
    }
}
