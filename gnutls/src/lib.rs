#[macro_use]
extern crate libc;
extern crate gnutls_sys as gt;

use std::ffi::CString;
use std::ffi::CStr;
use std::sync::{Once, ONCE_INIT};

pub mod error;
use error::{
    Error,
    AsError
};

use gt::consts::*;
use gt::gen::{
    gnutls_global_init,
    gnutls_global_deinit,
    gnutls_check_version,
};

macro_rules! is_succ {
    ($e:ident) => (
        if $e.as_error() == Error::None { Ok($e.as_error()) } else { Err($e.as_error()) }
    );
}

/// Globally initialize the library.
pub fn init() -> Result<Option<Error>, Error> {
    static mut INIT: Once = ONCE_INIT;
    let mut val: Option<i32> = None;

    unsafe{
        INIT.call_once(|| {
            val = Some(gnutls_global_init());
        });
    }

    match val {
        Some(val) => {
            if val == 0 {
                Ok(Some(Error::None))
            } else {
               Err(val.as_error())
            }
        },
        None => {
            // We couldn't initialize, return a failing code but don't break.
            Ok(Some(Error::CryptoInitFailed))
        }
    }
}

#[test]
fn test_init() {
    assert_eq!(init().ok().unwrap(), Some(Error::None));

    // Calling init twice should be successful, but return an error code.
    assert_eq!(init().ok().unwrap(), Some(Error::CryptoInitFailed));
}

/// Globally deinitialize the library.
pub fn deinit() {
    unsafe {
        gnutls_global_deinit()
    }
}

/// Check that the minimum libgnutls version is `req_version`. Returns the installed
/// version on success.
///
/// See: http://gnutls.org/manual/gnutls.html#gnutls_005fcheck_005fversion
///
pub fn check_version(req_version: Option<&'static str>) -> Result<&'static  str, &'static str> {
    let version = match req_version {
        Some(x) => x,
        None => GNUTLS_VERSION
    };

    unsafe {
        let result = gnutls_check_version(CString::new(version)
                                          .unwrap()
                                          .as_ptr());

        if result.is_null() {
            return Err("required version not found");
        }

        let something = CStr::from_ptr(result);
        Ok(something.to_str().unwrap())
    }
}

#[test]
fn test_check_version() {
    assert_eq!(check_version(Some("3.4.8")).unwrap(), "3.4.8");
    assert_eq!(check_version(None).unwrap(), "3.4.8");
    assert_eq!(check_version(Some("4")).unwrap_or(""), "");
}

pub mod sec_params;
pub mod dh_params;
pub mod certificate;
