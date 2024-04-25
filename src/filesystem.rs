//! TODO doc

use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::Path;

// TODO cache errors to report them cleanly (no assertion nor unwrap)
pub fn basic0() {
    const PATH: &str = "test";
    // Test creating file
    let mut file = OpenOptions::new()
        .create_new(true)
		.read(true)
        .write(true)
        .open(PATH)
        .unwrap();

    // Test writing
    let len = file.write(b"hello world!").unwrap();
    assert_eq!(len, 12);

    // Test seeking
    let off = file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(off, 0);
    let off = file.seek(SeekFrom::End(0)).unwrap();
    assert_eq!(off, 12);

    // Test reading
    let mut buf: [u8; 16] = [0; 16];
    let len = file.read(&mut buf).unwrap();
    assert_eq!(len, 0);
    assert_eq!(&buf, &[0u8; 16]);
    let off = file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(off, 0);
    let len = file.read(&mut buf).unwrap();
    assert_eq!(len, 12);
    assert_eq!(&buf, b"hello world!\0\0\0\0");

    // Test overwriting
    let off = file.seek(SeekFrom::Start(6)).unwrap();
    assert_eq!(off, 6);
    let len = file.write(b"abcdefghij").unwrap();
    assert_eq!(len, 10);

    // Test removing the file
    assert!(Path::new(PATH).exists());
    fs::remove_file(PATH).unwrap();
    assert!(!Path::new(PATH).exists());
    assert!(matches!(fs::remove_file(PATH), Err(e) if e.kind() == io::ErrorKind::NotFound));

    // Test file remove defer (file is still open)
    let off = file.seek(SeekFrom::End(0)).unwrap();
    assert_eq!(off, 16);
    let off = file.seek(SeekFrom::Start(0)).unwrap();
    assert_eq!(off, 0);
    let mut buf: [u8; 16] = [0; 16];
    let len = file.read(&mut buf).unwrap();
    assert_eq!(len, 16);
    assert_eq!(&buf, b"hello abcdefghij");
    drop(file);
}
