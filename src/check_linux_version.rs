use crate::errors::Errcode;
use nix::sys::utsname::uname;

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = match uname() {
        Ok(host) => host,
        Err(e) => {
            log::error!("Error while getting host info: {:?}", e);
            return Err(Errcode::ContainerError(1));
        }
    };

    let kernel_version = host.release().to_str().unwrap();

    log::debug!("Linux release: {}", kernel_version);

    if let Ok(version) = scan_fmt!(kernel_version, "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::NotSupported(0));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(Errcode::NotSupported(1));
    }

    Ok(())
}
