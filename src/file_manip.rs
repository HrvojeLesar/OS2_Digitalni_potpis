use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Result;

pub fn write_file(filename: &str, contents: &[u8], append_to: bool) -> Result<()> {
    let mut file = if append_to {
        File::options().append(true).open(filename)?
    } else {
        File::create(filename)?
    };
    file.write_all(contents)?;
    Ok(())
}

pub fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn read_file_to_buffer(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };

    use crate::file_manip::read_file_to_buffer;

    use super::{read_file_to_string, write_file};

    #[test]
    fn write_file_test() {
        write_file("write_file_test", b"content", false);
        fs::remove_file("write_file_test").unwrap();
    }

    #[test]
    fn write_file_append_test() {
        write_file("write_append_test", b"content", false);
        write_file("write_append_test", b"content", true);
        let res = read_file_to_string("write_append_test").unwrap();
        fs::remove_file("write_append_test").unwrap();
        assert_eq!(res, "contentcontent");
    }

    #[test]
    fn read_file_to_string_test() {
        let mut file = File::create("read_file_to_string_test").unwrap();
        file.write_all(b"test_string").unwrap();
        let res = read_file_to_string("read_file_to_string_test").unwrap();
        fs::remove_file("read_file_to_string_test").unwrap();

        assert_eq!(res, "test_string");
    }

    #[test]
    fn read_file_to_buffer_test() {
        let mut file = File::create("read_file_to_buffer_test").unwrap();
        file.write_all(b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D")
            .unwrap();
        let res = read_file_to_buffer("read_file_to_buffer_test").unwrap();
        fs::remove_file("read_file_to_buffer_test").unwrap();

        assert_eq!(
            res,
            b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D"
        );
    }
}
