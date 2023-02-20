pub mod boring;

pub mod macos;

use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;
use std::os::unix::io::RawFd;

pub struct Driver {
    fd: RawFd,
    kernel_name: String,
    custom_name: String,
}
