//! Utility features.

use libc::mode_t;
use std::error::Error;
use std::ffi::c_int;
use std::ffi::c_ulong;
use std::ffi::c_void;
use std::ffi::CStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::{io, mem};

pub struct TestError(pub String);

impl<E: Error> From<E> for TestError {
    fn from(err: E) -> Self {
        TestError(err.to_string())
    }
}

/// Result of a test.
pub type TestResult = Result<(), TestError>;

/// TODO doc
#[macro_export]
macro_rules! test_assert {
    ($predicate:expr) => {{
        let pred = ($predicate);
        if !pred {
            return Err($crate::util::TestError(format!(
                "Assertion failed: {}",
                stringify!($predicate)
            )));
        }
    }};
}

/// TODO doc
#[macro_export]
macro_rules! test_assert_eq {
    ($a:expr, $b:expr) => {{
        let a = ($a);
        let b = ($b);
        if a != b {
            return Err($crate::util::TestError(format!(
                "Assertion failed\n\tleft: `{:?}`\n\tright: `{:?}`",
                a, b
            )));
        }
    }};
}

pub fn chmod<P: AsRef<Path>>(path: P, mode: mode_t) -> io::Result<()> {
    let path = path.as_ref().as_os_str().as_bytes().as_ptr() as _;
    let res = unsafe { libc::chmod(path, mode) };
    if res >= 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn fchmod(fd: c_int, mode: mode_t) -> io::Result<()> {
    let res = unsafe { libc::fchmod(fd, mode) };
    if res >= 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

pub fn stat<P: AsRef<Path>>(path: P) -> io::Result<libc::stat> {
    unsafe {
        let mut stat: libc::stat = mem::zeroed();
        let path = path.as_ref().as_os_str().as_bytes().as_ptr() as _;
        let res = libc::stat(path, &mut stat);
        if res >= 0 {
            Ok(stat)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

pub fn fstat(fd: c_int) -> io::Result<libc::stat> {
    unsafe {
        let mut stat: libc::stat = mem::zeroed();
        let res = libc::fstat(fd, &mut stat);
        if res >= 0 {
            Ok(stat)
        } else {
            Err(io::Error::last_os_error())
        }
    }
}

pub fn mount(
    src: &CStr,
    target: &CStr,
    fstype: &CStr,
    flags: c_ulong,
    data: *const c_void,
) -> io::Result<()> {
    let res = unsafe { libc::mount(src.as_ptr(), target.as_ptr(), fstype.as_ptr(), flags, data) };
    if res >= 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// TODO doc
pub fn exec(cmd: &mut Command) -> TestResult {
    // TODO capture output and compare to expected output?
    let cmd = cmd.stdout(Stdio::null()).stderr(Stdio::null());
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(TestError(format!(
            "Command failed (status: {}): {:?}",
            status.code().unwrap(),
            cmd
        )))
    }
}
