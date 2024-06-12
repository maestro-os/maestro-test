//! procfs filesystem testing.

use crate::test_assert_eq;
use crate::util;
use crate::util::{TestError, TestResult};
use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::ptr::null;
use std::{env, fs};

pub fn mount() -> TestResult {
    fs::create_dir_all("/proc")?;
    let src = CString::new("procfs")?;
    let target = CString::new("/proc")?;
    let fstype = CString::new("procfs")?;
    util::mount(
        src.as_c_str(),
        target.as_c_str(),
        fstype.as_c_str(),
        0,
        null(),
    )?;
    Ok(())
}

pub fn cwd() -> TestResult {
    let cwd = fs::read_link("/proc/self/cwd")?;
    test_assert_eq!(cwd, current_dir()?);
    Ok(())
}

pub fn exe() -> TestResult {
    let exe = fs::read_link("/proc/self/exe")?;
    test_assert_eq!(exe.as_os_str().as_bytes(), b"/maestro-test");
    Ok(())
}

pub fn cmdline() -> TestResult {
    let args0 = fs::read("/proc/self/cmdline")?;
    let args1 = env::args_os();
    for (a0, a1) in args0.split(|b| *b == b'\0').zip(args1) {
        test_assert_eq!(a0, a1.as_bytes());
    }
    Ok(())
}

pub fn environ() -> TestResult {
    let environ = fs::read("/proc/self/environ")?;
    let args0 = environ
        .split(|b| *b == b'\0')
        .filter(|var| !var.is_empty())
        .map(|var| {
            let off = var
                .iter()
                .enumerate()
                .find(|(_, b)| **b == b'=')
                .map(|(i, _)| i)
                .ok_or_else(|| TestError("missing `=` for environment variable".to_owned()))?;
            let (name, value) = var.split_at(off);
            Ok((name, &value[1..]))
        })
        .collect::<Result<HashMap<_, _>, TestError>>()?;
    let args1: Vec<_> = env::vars_os().collect();
    let args1 = args1
        .iter()
        .map(|(name, val)| (name.as_bytes(), val.as_bytes()))
        .collect::<HashMap<_, _>>();
    test_assert_eq!(args0, args1);
    Ok(())
}
