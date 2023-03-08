# :computer: Windows pwd / Windows pwd

[Homepage / 主页](https://github.com/xxnuo/pwd)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## :globe_with_meridians: Introduction / 简介

This software is a Windows alternative to the Unix `pwd` command. It can be called using `pwd.exe` to display the current directory, `pwd .\file.txt` to display the full path of a file, or `pwd .\dir\` to display the full path of a directory. The program also copies the output to the clipboard and supports color output, with directories displayed in cyan and files displayed in green.

这个软件是为Windows编写的，可以代替 unix 的 `pwd` 命令。调用例子：`pwd.exe`（表示当前路径）、`pwd .\file.txt`、`pwd .\dir\`。程序会输出文件完整路径并复制到剪切板。并支持颜色输出，目录为青色，文件为绿色。

## :rocket: Getting Started / 入门指南

To use this software, simply download the `pwd.exe` file from the [release](https://github.com/xxnuo/pwd/releases) and save it to a directory in your PATH. Then, open a command prompt and call `pwd.exe` with the desired arguments.

使用本软件，只需从 [Release](https://github.com/xxnuo/pwd/releases) 中下载 `pwd.exe` 文件并保存到 PATH 中的目录中。然后，打开命令提示符并使用所需的参数调用 `pwd.exe`。

## :hammer: Building from Source / 从源代码构建

To build this software from source, you will need to have Rust installed on your system. Once Rust is installed, simply clone the repository and run `cargo build --release`. The resulting `pwd.exe` file will be located in the `target/release` directory.

要从源代码构建此软件，您需要在系统上安装 Rust。安装 Rust 后，只需克隆代码库并运行 `cargo build --release`。生成的 `pwd.exe` 文件将位于 `target/release` 目录中。

## :page_facing_up: License / 许可证

This software is licensed under the MIT License. See the `LICENSE` file for more information.

本软件根据 MIT 许可证授权。有关更多信息，请参见 `LICENSE` 文件。

## :handshake: Contributing / 贡献

Contributions to this project are welcome! If you find a bug or have an idea for a new feature, please open an issue or submit a pull request.

欢迎为此项目做出贡献！如果您发现错误或有新功能的想法，请打开问题或提交拉取请求。

## :heart: Support / 支持

If you find this software useful, please consider giving it a star on GitHub! This helps to spread the word and encourages further development.

如果您发现此软件有用，请在 GitHub 上为其点赞！这有助于传播信息并鼓励进一步开发。
