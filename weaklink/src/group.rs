use crate::{Error, Library};
use std::{
    mem,
    sync::atomic::{AtomicU8, Ordering},
};

/// Represents a group of symbols defined at build time.
#[repr(C)]
pub struct Group {
    name: &'static str,
    library: &'static Library,
    sym_indices: &'static [u32],
    status: AtomicU8,
}

/// Not yet attempted to resolve
const GROUP_STATUS_UNKNOWN: u8 = 0;
/// All symbols have been resolved successfuly
const GROUP_STATUS_RESOLVED: u8 = 1;
/// At least one symbol could not be resolved
const GROUP_STATUS_FAILED: u8 = 2;

impl Group {
    #[doc(hidden)]
    pub const fn new(name: &'static str, library: &'static Library, sym_indices: &'static [u32]) -> Group {
        Group {
            name,
            library,
            sym_indices,
            status: AtomicU8::new(GROUP_STATUS_UNKNOWN),
        }
    }

    /// Resolves the group's symbols if they haven't been resolved yet.
    /// The result is cached, so repeated calls will not trigger re-resolution.
    ///
    /// On success, this function returns a resolution state token. In [checked mode](index.html#checked-mode),
    /// the group’s resolution state is considered "resolved" only for the lifetime of the token. Once the token
    /// is dropped, the group's state reverts to "unknown" from the perspective of the calling thread.
    pub fn resolve(&self) -> Result<GroupResolved, Error> {
        let is_resolved = match self.status.load(Ordering::Acquire) {
            GROUP_STATUS_UNKNOWN => {
                for sym_index in self.sym_indices {
                    if let Err(err) = self.library.resolve_symbol(*sym_index) {
                        // Cache failed status
                        self.status.store(GROUP_STATUS_FAILED, Ordering::Release);
                        return Err(err);
                    }
                }
                // In checked mode we can't cache the "resolved" state, as the symbol table entries
                // will be reset to null upon dropping the token.
                #[cfg(not(feature = "checked"))]
                self.status.store(GROUP_STATUS_RESOLVED, Ordering::Release);
                true
            }
            GROUP_STATUS_RESOLVED => true,
            GROUP_STATUS_FAILED | _ => false,
        };
        if is_resolved {
            self.library.assert_resolved(self.sym_indices);
            Ok(GroupResolved(self))
        } else {
            Err(format!("Group {} could not be resolved", self.name).into())
        }
    }

    /// Marks the group as having failed symbol resolution.
    ///
    /// The purpose of this function is to simulate a failed group resolution in [checked mode](index.html#checked-mode).
    pub fn mark_failed(&self) {
        self.status.store(GROUP_STATUS_FAILED, Ordering::Release);
    }
}

/// Represents resolved state of a [Group]. See [Group::resolve()]
pub struct GroupResolved<'a>(&'a Group);

impl<'a> GroupResolved<'a> {
    /// Make group resolution permanent.
    ///
    /// Intended for permanently resolving one or more non-optional API groups.
    pub fn mark_permanent(self) {
        mem::forget(self);
    }
}

impl<'a> Drop for GroupResolved<'a> {
    fn drop(&mut self) {
        self.0.library.deassert_resolved(self.0.sym_indices);
    }
}
