//! Boot stub for integration tests.
//!
//! This file exists to run the tests as a second process in order to retrieve the exit code, then shutdown the machine.

use std::process::Command;

pub fn main() {
    let status = Command::new("/sbin/maestro-test").status().unwrap();
    let cmd = if status.success() { -1 } else { -2 };
    // Shutdown
    unsafe {
        libc::syscall(libc::SYS_reboot, 0xde145e83u32, 0x40367d6eu32, cmd, 0);
    }
    // Loop in case shutdown failed for some reason
    loop {}
}
