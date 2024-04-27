//! Utility features.

use std::ffi::{c_int, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::process::{Command, Stdio};
use std::{io, mem};

pub struct TestError;

impl From<io::Error> for TestError {
    fn from(_: io::Error) -> Self {
        TestError
    }
}

/// Result of a test.
pub type TestResult = Result<(), TestError>;

/// TODO doc
#[macro_export]
macro_rules! test_assert {
    ($predicate:expr) => {{
        if !($predicate) {
            return Err($crate::util::TestError);
        }
    }};
}

/// TODO doc
#[macro_export]
macro_rules! test_assert_eq {
    ($a:expr, $b:expr) => {{
        if ($a) != ($b) {
            return Err($crate::util::TestError);
        }
    }};
}

pub fn stat(path: &OsStr) -> io::Result<libc::stat> {
    unsafe {
        let mut stat: libc::stat = mem::zeroed();
        let path = path.as_bytes().as_ptr() as _;
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

/// TODO doc
pub fn exec(cmd: &mut Command) -> TestResult {
    // TODO capture output and compare to expected output?
    let cmd = cmd.stdout(Stdio::null()).stderr(Stdio::null());
    let status = cmd.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(TestError)
    }
}
