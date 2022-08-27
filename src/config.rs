use crate::errors::Errcode;

use crate::hostname::generate_hostname;
use crate::ipc::generate_socketpair;
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub fd: RawFd,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub hostname: String,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].clone();
        let hostname = generate_hostname()?;
        log::info!("Hostname: {}", hostname);
        Ok((
            ContainerOpts {
                path,
                argv,
                fd: sockets.1.clone(),
                hostname: hostname,
                uid,
                mount_dir,
            },
            sockets,
        ))
    }
}
