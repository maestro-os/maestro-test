//! TODO doc

use crate::util::{TestError, TestResult};
use crate::{test_assert, test_assert_eq, util};
use std::ffi::OsString;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::path::Path;
use std::str::FromStr;

pub fn basic() -> TestResult {
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

    // chmod
    for mode in 0..=0o7777 {
        unsafe {
            libc::fchmod(file.as_raw_fd(), mode);
            test_assert_eq!(io::Error::last_os_error().raw_os_error().unwrap_or(0), 0);
            let stat = util::fstat(file.as_raw_fd())?;
            test_assert_eq!(io::Error::last_os_error().raw_os_error().unwrap_or(0), 0);
            test_assert_eq!(stat.st_mode & 0o7777, mode);
        }
    }

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

pub fn directories() -> TestResult {
    // Create directory at path that do not exist (invalid)
    let res = fs::create_dir("/abc/def");
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::NotFound));

    // Create directories
    fs::create_dir_all("/abc/def/ghi")?;
    let stat = util::stat(OsString::from_str("/").unwrap().as_os_str())?;
    test_assert_eq!(stat.st_nlink, 3);
    let stat = util::stat(OsString::from_str("/abc").unwrap().as_os_str())?;
    test_assert_eq!(stat.st_nlink, 3);
    let stat = util::stat(OsString::from_str("/def").unwrap().as_os_str())?;
    test_assert_eq!(stat.st_nlink, 3);
    let stat = util::stat(OsString::from_str("/ghi").unwrap().as_os_str())?;
    test_assert_eq!(stat.st_nlink, 2);

    // TODO permissions

    // Entries iteration
    for i in 0..1000 {
        fs::create_dir(format!("/abc/{i}"))?;
    }
    let mut entries = fs::read_dir("/abc")?
        .map(|ent| {
            let ent = ent?;
            test_assert!(ent.file_type()?.is_dir());
            let file_name = ent.file_name();
            let file_name = file_name
                .to_str()
                .ok_or_else(|| TestError("invalid entry".to_owned()))?;
            Ok(file_name.parse::<u32>()? as _)
        })
        .collect::<Result<Vec<u32>, TestError>>()?;
    entries.sort_unstable();

    // Remove non-empty directory (invalid)
    let res = fs::remove_dir("/abc");
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::DirectoryNotEmpty));

    // Cleanup
    fs::remove_dir_all("/abc")?;

    Ok(())
}

pub fn hardlinks() -> TestResult {
    // Link to directory (invalid)
    fs::create_dir("/test_dir")?;
    let res = fs::hard_link("/test_dir", "/bad_link");
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::PermissionDenied));
    // Check the link has not been created
    let res = fs::remove_dir("/bad_link");
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::NotFound));
    // Cleanup
    fs::remove_dir("/test_dir")?;

    // Link to file
    fs::hard_link("/test", "/good_link")?;
    let inode0 = util::stat(OsString::from_str("/test").unwrap().as_os_str())?.st_ino;
    let inode1 = util::stat(OsString::from_str("/good_link").unwrap().as_os_str())?.st_ino;
    test_assert_eq!(inode0, inode1);
    // Remove and check
    fs::remove_file("/good_link")?;
    util::stat(OsString::from_str("/test").unwrap().as_os_str())?;
    let res = util::stat(OsString::from_str("/good_link").unwrap().as_os_str());
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::NotFound));

    // Link to file that don't exist (invalid)
    let res = fs::hard_link("/not_found", "/link");
    test_assert!(matches!(res, Err(e) if e.kind() == io::ErrorKind::NotFound));

    Ok(())
}
