use ::boxen::{BorderStyle, BoxenOptions, Spacing, boxen};
use chrono::{DateTime, Local};
use clap::Parser;
use inline_colorization::*;
use rust_mc_status::{McClient, ServerData, ServerEdition, ServerStatus};
use std::thread::sleep;
use std::time::Duration;

/// A simple but fast Minecraft server status logger
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP of the server
    #[arg(short, long)]
    ip: String,

    /// Port of the server
    #[arg(short, long)]
    port: u16,

    /// Minecraft game edition
    #[arg(short, long, value_parser = ["java", "bedrock"], default_value_t = String::from("java"))]
    edition: String,

    /// Interval in seconds over which the server is pinged
    #[arg(short = 'I', long, default_value_t = 20)]
    interval: u64,

    /// Duration in seconds to wait before closing the pinger client
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get config from args
    let args = Args::parse();
    let address: String = format!("{}:{}", args.ip, args.port);
    let edition: ServerEdition = if args.edition == "java" {
        ServerEdition::Java
    } else {
        ServerEdition::Bedrock
    };
    let interval: u64 = args.interval;
    let timeout: u64 = args.timeout;

    let client = McClient::new()
        .with_timeout(Duration::from_secs(timeout))
        .with_max_parallel(10);

    let mut i = 1;

    loop {
        let status: ServerStatus = client.ping(&address, edition).await?;
        let timestamp: DateTime<Local> = Local::now();

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

            // List sample of players (if supplied)
            if let Some(players_sample) = &data.players.sample {
                for player in players_sample {
                    lines.push(format!("  {color_green}-{color_blue} {} ({color_magenta}{}{color_blue}){color_reset}", player.name, player.id));
                }
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
            }

            // Show mods (if supplied)
            if let Some(mods) = &data.mods {
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

        let output = boxen(
            lines.join("\n"),
            Some(BoxenOptions {
                border_style: BorderStyle::Round,
                padding: Spacing::from(1),
                title: Some(format!("Iteration {i}")),
                ..Default::default()
            }),
        )
        .unwrap();
        println!("{output}");

        i += 1;

        sleep(Duration::from_secs(interval));
    }
}
