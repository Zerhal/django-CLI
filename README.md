# Django Boilerplate CLI

![GitHub top language](https://img.shields.io/github/languages/top/Zerhal/Django-CLI?style=for-the-badge)
![GitHub Release Date](https://img.shields.io/github/release-date/Zerhal/Django-CLI?style=for-the-badge)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/Zerhal/Django-CLI?style=for-the-badge)
![Contributors](https://img.shields.io/github/contributors/Zerhal/Django-CLI?style=for-the-badge)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/Zerhal/Django-CLI/total?style=for-the-badge)
![GitHub stars](https://img.shields.io/github/stars/Zerhal/Django-CLI?style=for-the-badge)


## Context

This project aims to simplify the creation and initial configuration of a Django project by providing a command-line interface (CLI) written in Rust. The goal is to automate the setup steps of a Django project, allowing developers to save time by avoiding repetitive tasks. The project is designed to be modular, enabling users to select the specific features they need during the project setup.

## Objective

The primary objective of this project is to provide a comprehensive and flexible CLI for creating Django projects. Users can configure their projects according to their specific needs, such as choosing the type of frontend, database, payment system integration, and more. In the future, the CLI could be extended with additional commands to perform tasks like health checks and other administrative functions.

## Features

- **Cross-platform support:** Works on Windows, MacOS, and Linux.
- **Modular Configuration:** Choose the components and features you want to include in your Django project.
- **Automation:** Automate repetitive tasks during Django project setup.
- **Extensibility:** Easily add new modules or features to the CLI as your needs grow.

## Installation

To install this CLI tool, follow these steps:

```bash
# Clone the repository
git clone https://github.com/Zerhal/django-CLI.git

# Navigate to the project directory
cd Django-CLI

# Build and install the CLI tool
cargo install --path .
```

## Usage

Here’s how to use this CLI tool to initialize a new Django project:

```bash
cd target/release
# Command to initialize a new Django project
django-cli
```

In the future, additional commands will be added to manage tasks like health checks and more.

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.