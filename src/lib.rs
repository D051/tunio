// Copyright © 2023 Dominik Schweigler
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

mod driver;

const NAME_SIZE: usize = 32;

pub struct Interface {
    kernel_name: [u8; NAME_SIZE],
    custom_name: [u8; NAME_SIZE],
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}

/// Basic custom implementations for Interface
impl Interface {
    /// Create a new Interface with default ip addresses
    pub fn new(name: &str, ipv4: Option<Ipv4Addr>, ipv6: Option<Ipv6Addr>) -> Interface {
        // allocate name buffers
        let mut kernel_name: [u8; NAME_SIZE] = [0; NAME_SIZE];
        let mut custom_name: [u8; NAME_SIZE] = [0; NAME_SIZE];
        // copy the name into the buffers
        kernel_name[..name.len()].copy_from_slice(name.as_bytes());
        custom_name[..name.len()].copy_from_slice(name.as_bytes());
        // return new Interface
        Interface {
            kernel_name,
            custom_name,
            ipv4,
            ipv6,
        }
    }
    /// Set the IPv4 address of the interface
    pub fn set_ipv4(&mut self, ipv4: Ipv4Addr) {
        self.ipv4 = Some(ipv4);
    }
    /// Set the IPv6 address of the interface
    pub fn set_ipv6(&mut self, ipv6: Ipv6Addr) {
        self.ipv6 = Some(ipv6);
    }
    /// Set the interface names
    pub fn set_names(&mut self, name: &str) {
        // wipe the buffers
        self.kernel_name = [0; NAME_SIZE];
        self.custom_name = [0; NAME_SIZE];
        // copy the name into the buffers
        self.kernel_name[..name.len()].copy_from_slice(name.as_bytes());
        self.custom_name[..name.len()].copy_from_slice(name.as_bytes());
    }
}

/// Default implementation for Interface
impl Default for Interface {
    fn default() -> Interface {
        Interface {
            kernel_name: [0; NAME_SIZE],
            custom_name: [0; NAME_SIZE],
            ipv4: None,
            ipv6: None,
        }
    }
}