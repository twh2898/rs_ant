#![allow(dead_code)]

pub mod console {
    #[cfg(windows)]
    pub fn clear_console() {
        extern crate libc;
        use std::ffi::CString;
        unsafe {
            let c_cmd = CString::new("cls").unwrap();
            let _ = libc::system(c_cmd.as_ptr());
        }
    }

    #[cfg(not(windows))]
    pub fn clear_console() {
        print!("{}[2J", 27 as char);
    }
}
