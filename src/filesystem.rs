//! TODO doc

use crate::util::TestResult;
use crate::{test_assert, test_assert_eq};
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

pub fn basic0() -> TestResult {
    const PATH: &str = "test";
    // Test creating file
    let mut file = OpenOptions::new()
        .create_new(true)
        .read(true)
        .write(true)
        .open(PATH)?;

    // Test writing
    let len = file.write(b"hello world!")?;
    test_assert_eq!(len, 12);

    // Test seeking
    let off = file.seek(SeekFrom::Start(0))?;
    test_assert_eq!(off, 0);
    let off = file.seek(SeekFrom::End(0))?;
    test_assert_eq!(off, 12);

    // Test reading
    let mut buf: [u8; 16] = [0; 16];
    let len = file.read(&mut buf)?;
    test_assert_eq!(len, 0);
    test_assert_eq!(&buf, &[0u8; 16]);
    let off = file.seek(SeekFrom::Start(0))?;
    test_assert_eq!(off, 0);
    let len = file.read(&mut buf)?;
    test_assert_eq!(len, 12);
    test_assert_eq!(&buf, b"hello world!\0\0\0\0");

    // Test overwriting
    let off = file.seek(SeekFrom::Start(6))?;
    test_assert_eq!(off, 6);
    let len = file.write(b"abcdefghij")?;
    test_assert_eq!(len, 10);

    // TODO chmod
    // TODO change access/modification times

    // Test removing the file
    test_assert!(Path::new(PATH).exists());
    fs::remove_file(PATH)?;
    test_assert!(!Path::new(PATH).exists());
    test_assert!(matches!(fs::remove_file(PATH), Err(e) if e.kind() == io::ErrorKind::NotFound));

    // Test file remove defer (file is still open)
    let off = file.seek(SeekFrom::End(0))?;
    test_assert_eq!(off, 16);
    let off = file.seek(SeekFrom::Start(0))?;
    test_assert_eq!(off, 0);
    let mut buf: [u8; 16] = [0; 16];
    let len = file.read(&mut buf)?;
    test_assert_eq!(len, 16);
    test_assert_eq!(&buf, b"hello abcdefghij");

    Ok(())
}
