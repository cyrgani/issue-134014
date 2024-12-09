use enginelib::EngineAPI;

use std::marker;
use std::os::raw;

pub struct Library {
    handle: *mut raw::c_void,
}

impl Library {
    unsafe fn open() -> Library {
        Library {
            handle: dlopen(c"target/debug/libengine_core.so".as_ptr(), 1),
        }
    }

    unsafe fn get<T>(&self) -> Symbol<T> {
        Symbol {
            pointer: dlsym(self.handle, c"run".as_ptr()),
            pd: marker::PhantomData,
        }
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            dlclose(self.handle);
        }
    }
}

pub struct Symbol<T> {
    pointer: *mut raw::c_void,
    pd: marker::PhantomData<T>,
}

impl<T> ::std::ops::Deref for Symbol<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            // Additional reference level for a dereference on `deref` return value.
            &*(&self.pointer as *const *mut _ as *const T)
        }
    }
}

#[link(name = "dl")]
extern "C" {
    fn dlopen(filename: *const raw::c_char, flags: raw::c_int) -> *mut raw::c_void;
    fn dlclose(handle: *mut raw::c_void) -> raw::c_int;
    fn dlsym(handle: *mut raw::c_void, symbol: *const raw::c_char) -> *mut raw::c_void;
}

fn main() {
    let mut api = EngineAPI::default();

    unsafe {
        let lib = Library::open();
        let run: Symbol<unsafe extern "Rust" fn(reg: &mut EngineAPI)> = lib.get();
        run(&mut api);
    }
}
