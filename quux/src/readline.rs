extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;

mod ext_readline {
    use readline::libc::c_char;

    #[link(name="readline")]
    extern {
        pub fn add_history(l: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: &str) {
    unsafe {
        ext_readline::add_history(CString::new(line).unwrap().as_ptr());
    }
}

pub fn readline(prompt: &str) -> Option<String> {
    let cprmpt = CString::new(prompt).unwrap().as_ptr();

    unsafe {
        let ptr = ext_readline::readline(cprmpt);

        if ptr.is_null() {
            None
        } else {
            let bytes = CStr::from_ptr(ptr).to_bytes();
            let line = String::from_utf8_lossy(bytes).into_owned().clone();

            libc::free(ptr as *mut _);

            Some(line)
        }
    }
}
