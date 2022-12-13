use std::{
    fs::File,
    io::{Read, Write},
};

use anyhow::Result;

const LINE_FEED: &[u8; 1] = b"\n";
const CARRIGE_RETURN: &[u8; 1] = b"\r";

pub fn write_file(filename: &str, contents: &[u8], append_to: bool) -> Result<()> {
    let mut file = if append_to {
        File::options().append(true).open(filename)?
    } else {
        File::create(filename)?
    };
    file.write_all(contents)?;
    Ok(())
}

pub fn read_file_to_buffer(filename: &str) -> Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    let last_byte_index = contents.len() - 1;
    // CR LF
    if contents[last_byte_index - 1] == CARRIGE_RETURN[0]
        && contents[last_byte_index] == LINE_FEED[0]
    {
        contents.remove(last_byte_index);
        contents.remove(last_byte_index - 1);
    } else if contents[last_byte_index] == LINE_FEED[0] { // LF
        contents.remove(last_byte_index);
    } else if contents[last_byte_index] == CARRIGE_RETURN[0] { // CR
        contents.remove(last_byte_index);
    }
    Ok(contents)
}
