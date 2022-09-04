mod cli;
mod errors;
use errors::exit_with_retcode;
use std::process::exit;
mod check_linux_version;
mod child;
mod config;
mod container;
mod hostname;
mod ipc;
mod mounts;
mod namespaces;

#[macro_use]
extern crate scan_fmt;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
