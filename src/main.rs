/* main.rs
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

mod app;

use crate::app::cli::Args;
use ::boxen::{BorderStyle, BoxenOptions, Spacing, boxen};
use chrono::{DateTime, Local};
use clap::Parser;
use inline_colorization::*;
use rust_mc_status::{JavaPlayer, McClient, ServerData, ServerEdition, ServerStatus};
use std::fs::OpenOptions;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Crash if IP is invalid
    if !args.is_valid_ip() {
        return Err("Invalid IP address or hostname.".into());
    }

    // Config from args
    let address: String = args.address();
    let edition: ServerEdition = if args.edition == "java" {
        ServerEdition::Java
    } else {
        ServerEdition::Bedrock
    };
    let interval: u64 = args.interval;
    let timeout: u64 = args.timeout;
    let output_file: String = args.output.unwrap_or(format!(
        "{address}-{}.log",
        Local::now().format("%Y-%m-%d_%H-%M-%S")
    ));
    let output_type: String = args.output_type;

    let client = McClient::new()
        .with_timeout(Duration::from_secs(timeout))
        .with_max_parallel(10);

    // Previous status
    let mut last_players_online: i64 = 0;
    let mut last_players_max: i64 = 0;

    let mut i = 1;

    loop {
        let status: ServerStatus = client.ping(&address, edition).await?;
        let timestamp: DateTime<Local> = Local::now();

        let mut players_sample: Vec<JavaPlayer> = Vec::new();
        let mut players_online: i64 = 0;
        let mut players_max: i64 = 0;

        let mut lines: Vec<String> = Vec::new();
        lines.push(format!(
            "{style_bold}{color_red}[{timestamp}]{color_cyan} Pinged server with address {color_yellow}{address}{color_cyan}.{style_reset}{color_reset}",
        ));

        // Display data for Java server
        if let ServerData::Java(data) = &status.data {
            lines.push(format!(
                "\n{color_green}Description:{color_reset}\n{}",
                data.description
            ));
            lines.push(format!(
                "\n{color_green}Version:{color_blue} {}{color_reset}",
                data.version.name
            ));
            lines.push(format!(
                "\n{color_green}Players:{color_blue} {}/{}{color_reset}",
                data.players.online, data.players.max
            ));

            players_online = data.players.online;
            players_max = data.players.max;

            // List sample of players (if supplied)
            if let Some(sample) = &data.players.sample {
                for player in sample {
                    lines.push(format!("  {color_green}-{color_blue} {} ({color_magenta}{}{color_blue}){color_reset}", player.name, player.id));
                }

                players_sample = sample.clone();
            }

            // Show current map name (if supplied)
            if let Some(map_name) = &data.map {
                lines.push(format!(
                    "\n{color_green}Map Name:{color_blue} {map_name}{color_reset}"
                ));
            }

            // Show game mode (if supplied)
            if let Some(game_mode) = &data.gamemode {
                lines.push(format!(
                    "\n{color_green}Game Mode:{color_blue} {game_mode}{color_reset}"
                ));
            }

            // Show server software (if supplied)
            if let Some(software) = &data.software {
                lines.push(format!(
                    "\n{color_green}Software:{color_blue} {software}{color_reset}"
                ));
            }

            // Show plugins (if supplied)
            if let Some(plugins) = &data.plugins {
                lines.push(format!(
                    "\n{color_green}Plugins:{color_blue} {}",
                    plugins.len()
                ));
                for plugin in plugins {
                    lines.push(format!(
                        "  {color_green}-{color_blue} {} ({color_magenta}{}{color_blue}){color_reset}",
                        plugin.name,
                        plugin.version.clone().unwrap_or("unknown".to_string())
                    ));
                }
            } else if let Some(mods) = &data.mods {
                lines.push(format!("\n{color_green}Mods:{color_blue} {}", mods.len()));
                for m in mods {
                    lines.push(format!(
                        "  {color_green}-{color_blue} {} ({color_magenta}{}{color_blue}){color_reset}",
                        m.modid,
                        m.version.clone().unwrap_or("unknown".to_string())
                    ));
                }
            }
        }

        // Display data for Bedrock server
        if let ServerData::Bedrock(data) = &status.data {
            lines.push(format!(
                "\n{color_green}Edition:{color_blue} {}{color_reset}",
                data.edition
            ));
            lines.push(format!("\n{color_green}MOTD:{color_reset}\n{}", data.motd));
            lines.push(format!(
                "\n{color_green}Secondary MOTD:{color_reset}\n{}",
                data.motd2
            ));
            lines.push(format!(
                "\n{color_green}Protocol Version:{color_blue} {}{color_reset}",
                data.protocol_version
            ));
            lines.push(format!(
                "\n{color_green}Version:{color_blue} {}{color_reset}",
                data.version
            ));
            lines.push(format!(
                "\n{color_green}Players:{color_blue} {}/{}{color_reset}",
                data.online_players, data.max_players
            ));

            players_online = data.online_players.parse().unwrap_or_default();
            players_max = data.max_players.parse().unwrap_or_default();

            lines.push(format!(
                "\n{color_green}Server UID:{color_blue} {}{color_reset}",
                data.server_uid
            ));
            lines.push(format!(
                "\n{color_green}Game Mode:{color_blue} {} ({}){color_reset}",
                data.game_mode, data.game_mode_numeric
            ));
            lines.push(format!(
                "\n{color_green}IPv4 Port:{color_blue} {}{color_reset}",
                data.port_ipv4
            ));
            lines.push(format!(
                "\n{color_green}IPv6 Port:{color_blue} {}{color_reset}",
                data.port_ipv6
            ));

            // Show current map name (if supplied)
            if let Some(map_name) = &data.map {
                lines.push(format!(
                    "\n{color_green}Map Name:{color_blue} {map_name}{color_reset}"
                ));
            }

            // Show server software (if supplied)
            if let Some(software) = &data.software {
                lines.push(format!(
                    "\n{color_green}Software:{color_blue} {software}{color_reset}"
                ));
            }
        }

        let output: String = boxen(
            lines.join("\n"),
            Some(BoxenOptions {
                border_style: BorderStyle::Round,
                border_color: Some(::boxen::Color::Named("white".to_string())),
                padding: Spacing::from(1),
                title: Some(format!("Iteration {i}")),
                ..Default::default()
            }),
        )
        .unwrap();
        println!("{output}");

        // Write output to file
        if !args.no_output {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&output_file)?;

            match output_type.as_str() {
                "players" => {
                    if last_players_online != players_online || last_players_max != players_max {
                        let mut content: Vec<String> = Vec::new();
                        content.push(format!(
                            "[{timestamp}] There are now {players_online}/{players_max} players on the server {address}. (Iteration {i})"
                        ));

                        for player in players_sample {
                            content.push(format!("  - {} ({})", player.name, player.id));
                        }

                        writeln!(file, "{}\n", content.join("\n"))?;

                        last_players_online = players_online;
                        last_players_max = players_max;
                    }
                }
                _ => writeln!(file, "{}", strip_ansi_escapes::strip_str(output))?,
            }
        }

        i += 1;

        sleep(Duration::from_secs(interval));
    }
}
