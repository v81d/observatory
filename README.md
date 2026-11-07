# Observatory

Observatory is a fast CLI Minecraft server status logger, compatible with both Java and Bedrock servers. It supports continuous logging and saving the output to a log file.

## Quick Start

The following guide provides instructions on how to build and run Observatory.

### Prerequisites

You must have Rust and Cargo installed. If you don't already have them, you can do so by following the [official install guide](https://rust-lang.org/tools/install).

### Build Instructions

After installing the requirements, you can build the project. To do so, follow the steps:

1. Clone the repository:

```bash
git clone https://github.com/v81d/observatory.git
cd observatory
```

2. Build the project:

```bash
cargo build --release
```

Now, the project should be built and compiled into an executable in `target/release/observatory`.

### Running Observatory

To run Observatory, first make sure the project has already been compiled successfully in `target/release/observatory`. Then, launch the executable:

```bash
./target/release/observatory --help
```

This will display a help page describing Observatory and its command usage.

## Usage

Observatory is a command-line tool, meaning it is designed to be run from your terminal. Below is a table showing the possible options you can pass.

| Option                    | Usage                                                                   |
| ------------------------- | ----------------------------------------------------------------------- |
| -i, --ip <IP>             | IP of the server                                                        |
| -p, --port <PORT>         | Port of the server                                                      |
| -e, --edition <EDITION>   | Minecraft game edition [default: java] [possible values: java, bedrock] |
| -I, --interval <INTERVAL> | Interval in seconds over which the server is pinged [default: 20]       |
| -o, --output <OUTPUT>     | Location to save log output                                             |
| --no-output               | Do not save log to output file                                          |
| -h, --help                | Print help                                                              |
| -V, --version             | Print version                                                           |

## License

Observatory is free software distributed under the **GNU General Public License, version 3.0 or later (GPL-3.0+).**

You are free to use, modify, and share the software under the terms of the GPL.
For full details, see the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html).
