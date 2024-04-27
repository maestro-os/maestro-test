//! `maestro-test` is a test suite for [Maestro](https://github.com/llenotre/maestro).

use crate::util::{exec, TestResult};
use std::process::Command;

mod filesystem;
mod procfs;
mod util;

/*
 * TODO when the serial port is unlinked from the TTY,
 * setup the output so that it is printed on both the stdout and serial port
 */

struct TestSuite {
    name: &'static str,
    desc: &'static str,
    tests: &'static [Test],
}

struct Test {
    name: &'static str,
    desc: &'static str,
    start: fn() -> TestResult,
}

/// The list of tests to perform.
const TESTS: &[TestSuite] = &[
    TestSuite {
        name: "filesystem",
        desc: "Files and filesystems handling",
        tests: &[
            Test {
                name: "basic0",
                desc: "Create, remove and modify the properties of a single file",
                start: filesystem::basic0,
            },
            // TODO umask
            Test {
                name: "hardlinks",
                desc: "Test hardlinks creation",
                start: filesystem::hardlinks,
            },
            // TODO directory (with permissions)
            // TODO symbolic links
            // TODO mount/umount (procfs and tmpfs. check /proc/mounts too)
            // TODO mount/umount another real filesystem
            // TODO rename (including across different filesystems)
            // TODO file fifo/socket (including in tmpfs)
            // TODO check /dev/* contents
        ],
    },
    // TODO fork/clone (threads)
    // TODO signals (handlers and masking)
    // TODO ELF files (execve)
    // TODO user/group file accesses (including SUID/SGID)
    // TODO mmap/munmap (including shared libraries)
    // TODO time ((non)-monotonic clock, sleep and timer_*)
    // TODO termcaps
    // TODO SSE/MMX/AVX states consistency
    TestSuite {
        name: "procfs",
        desc: "",
        tests: &[
            Test {
                name: "/proc/self/cwd",
                desc: "/proc/self/cwd",
                start: procfs::cwd,
            },
            Test {
                name: "/proc/self/exe",
                desc: "/proc/self/exe",
                start: procfs::exe,
            },
            Test {
                name: "/proc/self/cmdline",
                desc: "/proc/self/cmdline",
                start: procfs::cmdline,
            },
            Test {
                name: "/proc/self/environ",
                desc: "/proc/self/environ",
                start: procfs::environ,
            },
            // TODO /proc/self/stat
        ],
    },
    TestSuite {
        name: "command",
        desc: "Basic commands testing",
        tests: &[
            Test {
                name: "ls -l /",
                desc: "ls -l /",
                start: || exec(Command::new("ls").args(["-l", "/"])),
            },
            Test {
                name: "ls -lR /",
                desc: "ls -lR /",
                start: || exec(Command::new("ls").args(["-lR", "/"])),
            },
            // TODO `cat`
            // TODO `cat -e`
            // TODO `cp`
            // TODO `rm`
        ],
    },
    // TODO scripts (Shell/Perl)
    // TODO compilation (C/C++/Rust)
    // TODO network
];

fn main() {
    // Start marker
    println!();
    println!("[START]");
    for suite in TESTS {
        println!("[SUITE] {}", suite.name);
        println!("[DESC] {}", suite.desc);
        for test in suite.tests {
            println!("[TEST] {}", test.name);
            println!("[DESC] {}", test.desc);
            let res = (test.start)();
            match res {
                Ok(_) => println!("[OK]"),
                Err(_) => println!("[KO]"),
            }
        }
    }
    // End marker
    println!("[END]");
}
