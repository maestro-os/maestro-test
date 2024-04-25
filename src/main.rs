//! `maestro-test` is a test suite for [Maestro](https://github.com/llenotre/maestro).

mod filesystem;

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
    start: fn(),
}

/// The list of tests to perform.
const TESTS: &[TestSuite] = &[TestSuite {
    name: "filesystem",
    desc: "Files and filesystems handling",
    tests: &[Test {
        name: "basic0",
        desc: "Create, remove and modifiy the properties of a single file",
        start: filesystem::basic0,
    }],
}];

fn main() {
    // Start marker
    println!();
    println!("---start---");
    for suite in TESTS {
		// TODO print info
		for test in suite.tests {
			// TODO print info
			(test.start)();
		}
	}
}
