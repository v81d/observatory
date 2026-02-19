# Observatory

![GitHub commit activity](https://img.shields.io/github/commit-activity/w/v81d/observatory?style=for-the-badge)
![GitHub top language](https://img.shields.io/github/languages/top/v81d/observatory?style=for-the-badge)
![GitHub issues or pull requests](https://img.shields.io/github/issues/v81d/observatory?style=for-the-badge)
![GitHub license](https://img.shields.io/github/license/v81d/observatory?style=for-the-badge)

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

## Contributing

### Reporting Issues

To report an issue or bug, visit Observatory's [issue tracker](https://github.com/v81d/observatory/issues) on GitHub.

### Pull Requests

To push your features or fixes into this official repository:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/my-feature`) or a fix branch (`git checkout -b fix/my-fix`).
3. Commit your changes (`git commit -m "feat: add new feature"`). **Please follow the [Conventional Commits](https://www.conventionalcommits.org) guideline when doing so!**
4. Push the branch (`git push origin feature/my-feature`).
5. Open a pull request with `contrib` as the base branch. Make sure to create a detailed title and description of your change.

Please follow the [GitHub flow](https://guides.github.com/introduction/flow) and Observatory's [Code of Conduct](CODE_OF_CONDUCT.md) when submitting a pull request.

## License

Observatory is free software distributed under the **GNU General Public License, version 3.0 or later (GPL-3.0+).**

You are free to use, modify, and share the software under the terms of the GPL.
For full details, see the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.html).
