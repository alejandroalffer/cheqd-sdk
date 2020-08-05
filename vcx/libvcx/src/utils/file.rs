use std::io::{Read, Write};
use std::path::{PathBuf, Path};
use std::fs::{File, DirBuilder, OpenOptions};
use error::prelude::*;

pub fn read_file<P: AsRef<Path>>(file: P) -> VcxResult<String> {
    let mut file = File::open(file)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Cannot open file. Err: {:?}", err)))?;

    let content = {
        let mut s = String::new();
        file.read_to_string(&mut s)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Cannot read file. Err: {:?}", err)))?;
        s
    };

    Ok(content)
}

pub fn write_file<P: AsRef<Path>>(file: P, content: &str) -> VcxResult<()> where P: std::convert::AsRef<std::ffi::OsStr> {
    let path = PathBuf::from(&file);

    if let Some(parent_path) = path.parent() {
        DirBuilder::new()
            .recursive(true)
            .create(parent_path)
            .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Can't create the path to file. Err: {:?}", err)))?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Cannot open file. Err: {:?}", err)))?;

    file
        .write_all(content.as_bytes())
        .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Can't write content: \"{}\" to the file. Err: {:?}", content, err)))?;

    file.flush()
        .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Can't write content: \"{}\" to the file. Err: {:?}", content, err)))?;

    file.sync_data()
        .map_err(|err| VcxError::from_msg(VcxErrorKind::IOError, format!("Can't write content: \"{}\" to the file. Err: {:?}", content, err)))
}