/* cli.rs
 *
 * Copyright 2026 v81d
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use clap::Parser;
use std::net::IpAddr;

/// A simple and fast Minecraft server status logger
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// IP of the server
    #[arg(short, long)]
    pub ip: String,

    /// Port of the server
    #[arg(short, long)]
    pub port: u16,

    /// Minecraft game edition
    #[arg(short, long, value_parser = ["java", "bedrock"], default_value_t = String::from("java"))]
    pub edition: String,

    /// Interval in seconds over which the server is pinged
    #[arg(short = 'I', long, default_value_t = 20)]
    pub interval: u64,

    /// Duration in seconds to wait before closing the pinger client
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,

    /// Output type
    #[arg(long, value_parser = ["all", "players"], default_value_t = String::from("all"))]
    pub output_type: String,

    /// Location to save log output
    #[arg(short, long)]
    pub output: Option<String>,

    /// Do not save log to output file
    #[arg(long, default_value_t = false)]
    pub no_output: bool,
}

impl Args {
    pub fn is_valid_ip(&self) -> bool {
        if self.ip.parse::<IpAddr>().is_ok() {
            return true;
        }

        hostname_validator::is_valid(&self.ip)
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
