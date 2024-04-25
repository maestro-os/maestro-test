//! Utility features.

use std::io;
use std::process::{Command, Stdio};

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
