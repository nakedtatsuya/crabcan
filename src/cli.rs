use crate::errors::Errcode;
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(name = "crabcan", about = "A simple container in Rust.")]
pub struct Args {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[clap(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[clap(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[clap(parse(from_os_str), short = 'm', long = "mount")]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args: Args = Args::parse();

    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(Errcode::ArgumentInvalid("mount"));
    }

    // If args.debug: Setup log at debug level
    // Else: Setup log at info level

    // Validate arguments

    Ok(args)
}

#[inline(always)]
pub fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
