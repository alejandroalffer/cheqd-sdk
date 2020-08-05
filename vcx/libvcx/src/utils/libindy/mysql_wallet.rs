use indy::ErrorCode;

pub fn init_mysql_wallet() -> Result<(), ErrorCode> {
    debug!("initializing mysql wallet");

    let lib = _load_lib("libmysqlstorage.so").map_err(|_| ErrorCode::CommonInvalidState)?;

    unsafe {
        let init_func: libloading::Symbol<unsafe extern fn() -> ErrorCode> = lib.get("mysql_storage_init".as_bytes())
            .map_err(|_| ErrorCode::CommonInvalidState)?;

        match init_func() {
            ErrorCode::Success => Ok(()),
            e => {
                error!("Mysql storage has not been loaded");
                Err(e)
            }
        }
    }
}

#[cfg(all(unix, test))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::os::unix::Library::open(Some(library), ::libc::RTLD_NOW | ::libc::RTLD_NODELETE)
        .map(libloading::Library::from)
}

#[cfg(any(not(unix), not(test)))]
fn _load_lib(library: &str) -> libloading::Result<libloading::Library> {
    libloading::Library::new(library)
}
