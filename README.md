# Gotoo

Gotoo is a command-line project manager for developers who want to efficiently manage and access their project directories using their preferred text editors. With Gotoo, you can easily open saved projects in your chosen editor and add new projects to the list for quick access.

## Table of Contents

- [Supported Operating Systems](#supported-operating-systems)
- [Installation](#installation)
- [Usage](#usage)
  - [Open a Saved Project](#open-a-saved-project)
  - [Add a New Project](#add-a-new-project)
- [TODO](#todo)
- [Contributing](#contributing)
- [License](#license)

## Supported Operating Systems

✅ = Yes,
⚠️ = Not Yet

| Operating System | Supported |
| ---------------- | --------- |
| Windows          | ✅        |
| MacOS            | ⚠️        |
| Linux            | ⚠️        |

## Installation

### Install via npm

Install the package globally with npm or yarn.

```shell
npm install --global gotoo
```

This will download the Javascript installer from npm, which will download the relevant pre-compiled binary from the corresponding release on Github.

Checkout [gotoo on npm](https://www.npmjs.com/package/gotoo)

### Download manually

Download the binary for your platform from the [latest release](https://github.com/rajabilal555/gotoo/releases) on Github.

### Install via source

You'll need to have Rust installed on your system. If you haven't already, you can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once Rust is installed, you can build and install Gotoo by running the following commands:

```shell
git clone https://github.com/rajabilal555/gotoo.git
cd gotoo
cargo install --path .
```

### Install via cargo

If you would like to install it The Rust Way™. Run the following command:

```shell
cargo install gotoo
```

## Usage

### Open a Saved Project

To open a saved project in your chosen editor, use the following command:

```shell
gotoo -o
```

You will be prompted to choose a project from your list, and then select an editor to open the project.

### Add a New Project

To add a new project to your list, use the following command:

```shell
gotoo -a
```

You will be prompted to provide the project name and directory path. Additionally, you'll choose your preferred text editor for this project.
When adding a new project, it will be saved in the Gotoo project list for future access.

## TODO

- [x] Add support for windows.
- [ ] Add support for linux.
- [ ] Add support for MacOS.
- [ ] Add support for adding custom editor commands.
- [ ] Add a default editor config.
- [ ] A fuzzy search in the project selector

## Contributing

Contributions are welcome from the community. If you'd like to contribute to Gotoo, please follow these steps:

1. Fork the repository.
1. Create a new branch for your feature or bug fix: git checkout -b feature/your-feature-name.
1. Make your changes and commit them.
1. Push your changes to your fork: git push origin feature/your-feature-name.
1. Create a pull request to the this repository's main branch.
1. Please ensure that your code follows best practices, includes tests if applicable, and adheres to the project's coding style.

## License

This project is licensed under the MIT License (with Attribution). See the [LICENSE](./LICENSE) file for details.
