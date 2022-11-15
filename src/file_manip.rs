use std::{
    fs::File,
    io::{Read, Write},
};

pub fn write_file(filename: &str, contents: &[u8], append_to: bool) {
    let mut file = if append_to {
        File::options().append(true).open(filename).unwrap()
    } else {
        File::create(filename).unwrap()
    };
    file.write_all(contents).unwrap();
}

pub fn read_file_to_string(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

pub fn read_file_to_buffer(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod tests {
    use std::{fs::{File, self}, io::Write};

    use crate::file_manip::read_file_to_buffer;

    use super::{write_file, read_file_to_string};

    
    #[test]
    fn write_file_test() {
        write_file("testfile", b"content", false);
        fs::remove_file("testfile").unwrap();
    }

    fn write_file_append_test() {
        write_file("testfile2", b"content", false);
        write_file("testfile2", b"content", true);
        let res = read_file_to_string("read_file_to_string_test");
        fs::remove_file("testfile2").unwrap();

        assert_eq!(res, "contentcontent");
    }

    #[test]
    fn read_file_to_string_test() {
        let mut file = File::create("read_file_to_string_test").unwrap();
        file.write_all(b"test_string").unwrap();
        let res = read_file_to_string("read_file_to_string_test");
        fs::remove_file("read_file_to_string_test").unwrap();

        assert_eq!(res, "test_string");
    }

    #[test]
    fn read_file_to_buffer_test() {
        let mut file = File::create("read_file_to_buffer").unwrap();
        file.write_all(b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D").unwrap();
        let res = read_file_to_buffer("read_file_to_buffer");
        fs::remove_file("read_file_to_buffer").unwrap();

        assert_eq!(res, b"\xAC\x54\x67\xA0\xAF\x9E\x17\x1D\xD4\x14\xD9\xB5\x8E\x20\x1C\x2D");
    }
}