//! # Overview
//!
//! Weaklink is a cross-platform implementation of weak dynamic linkage.
//!
//! This is intended for programs that need to load external plugins with some symbols possibly missing
//! (because of different versions, etc).
//!
//! Features:
//! - Does not require changing client call sites to explicitly resolve and use pointers to the functions.  
//!   This is especially for calling into plugins that export mangled symbols (like C++ or Rust), since finding out
//!   the mangled symbol name for a function may be non-trivial.
//! - Allows fine control of when a plugin dylib is loaded and from which file.
//! - Allows separating the plugin API into subsets, some of which may be optional.  The client code may
//!   check whether all symbols in a subset are available before using them.
//!
//! ## How this works:
//! - At build time, you will use the companion weaklink_build crate to create a stub library for each
//!   plugin dylib you intend to load.  The library contains:
//!   - Data structures similar to [GOT/PLT](https://en.wikipedia.org/wiki/Global_Offset_Table) for symbols of your
//!     choosing.
//!   - API that allows controlling dylib loading and symbol resolution.
//! - You statically link your client code against the stub library.
//! - At run time the stub library redirects function calls to the corresponding functions in the loaded plugin dylib.
//!
//! ## Limitations
//! Transparent redirection is implemented only for code symbols (functions).  Supporting data symbols would required
//! linker support.  Wrapping data symbols is still possible, but will require use site changes (basically, you'll
//! need to call a function that returns the address of the data).
//!
//! ## Supported OS and architectures:
//! - Linux: x86_64, arm, aarch64
//! - MacOS: x86_64, arm64
//! - Windows: x86_64

pub use loading::{Address, DylibHandle};
use std::{
    cell::UnsafeCell,
    ffi::{CStr, CString},
    mem,
    path::Path,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
};

pub type Error = Box<dyn std::error::Error>;

mod loading;

/// Represents a weakly linked dynamic library.
#[repr(C)]
pub struct Library {
    handle: AtomicUsize,
    dylib_names: &'static [&'static str],
    symbol_names: &'static [&'static CStr],
    symbol_table: &'static [Address],
}

impl Library {
    #[doc(hidden)]
    pub const fn new(
        dylib_names: &'static [&'static str],
        symbol_names: &'static [&'static CStr],
        symbol_table: &'static [Address],
    ) -> Library {
        Library {
            handle: AtomicUsize::new(0),
            dylib_names,
            symbol_names,
            symbol_table,
        }
    }

    /// Load library with default name (configured during the build).
    pub fn load(&self) -> Result<DylibHandle, Error> {
        let raw_handle = self.handle.load(Ordering::Acquire);
        if raw_handle != 0 {
            return Err("Already loaded.".into());
        } else {
            for name in self.dylib_names {
                let cpath = CString::new(*name).unwrap();
                if let Ok(handle) = unsafe { loading::load_library(&cpath) } {
                    self.handle.store(handle.0, Ordering::Release);
                    return Ok(handle);
                }
            }
        }
        Err("Library not found.".into())
    }

    /// Load library from the specified path.
    pub fn load_from(&self, path: &Path) -> Result<DylibHandle, Error> {
        let raw_handle = self.handle.load(Ordering::Acquire);
        if raw_handle != 0 {
            Err("Already loaded.".into())
        } else {
            let cpath = CString::new(path.as_os_str().to_str().unwrap().as_bytes()).unwrap();
            match unsafe { loading::load_library(&cpath) } {
                Ok(handle) => {
                    self.handle.store(handle.0, Ordering::Release);
                    Ok(handle)
                }
                Err(err) => Err(err),
            }
        }
    }

    // Sets the library handle.
    pub fn set_handle(&self, handle: DylibHandle) {
        self.handle.store(handle.0, Ordering::Release);
    }

    // Returns the library handle, if already loaded.
    pub fn handle(&self) -> Option<DylibHandle> {
        let raw_handle = self.handle.load(Ordering::Acquire);
        if raw_handle != 0 {
            Some(DylibHandle(raw_handle))
        } else {
            None
        }
    }

    // Make sure the library is loaded (or panic).
    fn ensure_loaded(&self) -> DylibHandle {
        match self.handle() {
            Some(handle) => handle,
            None => match self.load() {
                Ok(handle) => handle,
                Err(err) => panic!("{}", err),
            },
        }
    }

    // Get a reference to symbol pointer.
    unsafe fn symbol_table_entry(&self, sym_index: usize) -> *mut Address {
        let ptr: &UnsafeCell<Address> = mem::transmute(&self.symbol_table[0]);
        ptr.get().offset(sym_index as isize) as *mut Address
    }
}

/// Represents a symbol group defined at build time.
#[repr(C)]
pub struct Group {
    library: &'static Library,
    sym_indices: &'static [u32],
    status: AtomicU8,
}

const GROUP_STATUS_UNRESOLVED: u8 = 0;
const GROUP_STATUS_RESOLVED: u8 = 1;
const GROUP_STATUS_FAILED: u8 = 2;

impl Group {
    #[doc(hidden)]
    pub const fn new(library: &'static Library, sym_indices: &'static [u32]) -> Group {
        Group {
            library,
            sym_indices,
            status: AtomicU8::new(GROUP_STATUS_UNRESOLVED),
        }
    }

    /// Attempt to resolve all symbols in the group.
    pub fn resolve_uncached(&self) -> Result<(), Error> {
        let handle = self.library.ensure_loaded();
        unsafe {
            for sym_index in self.sym_indices {
                let sym_index = *sym_index as usize;
                let sym_name = self.library.symbol_names[sym_index];
                let sym_addr = match loading::find_symbol(handle, sym_name) {
                    Ok(sym_addr) => sym_addr,
                    Err(err) => return Err(err),
                };
                self.library.symbol_table_entry(sym_index).write(sym_addr);
            }
        }
        Ok(())
    }

    /// Calls resolve_uncached(), and caches resolution status.
    pub fn resolve(&self) -> bool {
        match self.status.load(Ordering::Acquire) {
            GROUP_STATUS_UNRESOLVED => {
                let result = self.resolve_uncached().is_ok();
                let status = match result {
                    true => GROUP_STATUS_RESOLVED,
                    false => GROUP_STATUS_FAILED,
                };
                self.status.store(status, Ordering::Release);
                result
            }
            GROUP_STATUS_RESOLVED => true,
            GROUP_STATUS_FAILED | _ => false,
        }
    }
}
