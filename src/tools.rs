// Copyright (c) 2015 T. Okubo
// This file is part of vlc-rs.
// Licensed under the MIT license, see the LICENSE file.

use libc::c_char;
use std::borrow::Cow;
use std::ffi::{CStr, CString, NulError};
use std::path::Path;

// Convert String to CString.
// Panic if the string includes null bytes.
pub fn to_cstr(s: &str) -> CString {
    CString::new(s.to_owned()).expect("Error: Unexpected null byte")
}

// Convert *const c_char to String
pub unsafe fn from_cstr(p: *const c_char) -> Option<String> {
    if p.is_null() {
        None
    } else {
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy().into_owned())
    }
}

// Convert *const c_char to &str
pub unsafe fn from_cstr_ref<'a>(p: *const c_char) -> Option<Cow<'a, str>> {
    if p.is_null() {
        None
    } else {
        let cstr = CStr::from_ptr(p);

        Some(cstr.to_string_lossy())
    }
}

// Create CString from &Path
pub fn path_to_cstr(path: &Path) -> Result<CString, NulError> {
    let path = CString::new(path.to_string_lossy().into_owned())?;

    Ok(path)
}
