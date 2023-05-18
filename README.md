# rusty_shell
rusty_shell is a simple shell program implemented in Rust. It is the Rust implementation of Pr 2: Shell where I implemented a trivial shell in C/C++ using the fork, exec, and wait system calls. It provides a basic command-line interface where users can enter commands and execute them. The project aims to demonstrate how to interact with the underlying operating system using raw pointers and C interoperability.

The shell is designed to showcase the low-level features of Rust by leveraging raw pointers and interacting with the C programming language. By using raw pointers and C interoperability, rusty_shell demonstrates how to combine Rust's safety guarantees and the more permissive nature of low-level systems programming.

## Features
* **Command Parsing and Tokenization:** The shell parses the user-entered commands and tokenizes them based on whitespace separation. This process involves breaking down the command into individual tokens for further processing. <br>
* **Command Execution:** The shell supports executing commands using the execvp system call. It can execute both system commands (e.g., echo, date, whoami) and basic shell commands (e.g., ls, cd, pwd). <br>
* **Built-in Commands:** The shell provides basic built-in commands such as quit or exit to exit the shell. These commands are handled internally without invoking external processes. <br>
* **Error Handling:** The shell includes basic error handling for command execution failures. It displays an error message if a command fails to execute, allowing users to identify and address the issue. <br>

## Design Decisions
**Minimalistic Approach:** rusty_shell focuses on simplicity using fork, exec, and wait system calls. It aims to provide a minimalistic shell environment without the complexity of advanced features such as input/output redirection or piping. Although the approach is minimalistic, the code required to achieve such a task in Rust is difficult.

**Raw Pointers and C Interoperability:** The decision to use raw pointers and C interoperability was made to demonstrate the low-level capabilities of Rust. By leveraging C APIs and working with raw pointers, rusty_shell showcases how Rust can interact with existing C code and system calls.

**Command Tokenization:** The tokenization process allows the shell to break down user-entered commands into individual tokens. This design decision enables further processing and flexible command execution.

**Separation of Concerns:** rusty_shell follows a modular approach with separate functions for command parsing, execution, and built-in commands. This separation of concerns improves code readability and maintainability.

**Build System:** By choosing Cargo as the build system (instead of rustc) and package manager for rusty_shell, you can take advantage of its robust features, efficient build process, and seamless integration with Rust. It simplifies dependency management, improves code organization, and enhances the overall development experience, allowing you to focus on implementing the shell's functionality without getting bogged down by build-related complexities.

## Prerequisites
* **Rust Programming Language:** rusty_shell is implemented in Rust, so you'll need Rust installed on your system. Visit the Rust website for installation instructions. <br>
* **Cargo Package Manager:** Cargo is the default package manager for Rust. It is bundled with the Rust installation. Cargo simplifies building, managing dependencies, and running Rust projects. <br>

## Usage
Clone the repository: `git clone https://github.com/Anish-Gupta03/rusty_shell.git` <br>
Run the project: `cargo run` <br>
This will start the shell prompt, where you can enter commands. <br>

## Supported Commands
The rusty_shell supports various commands, including: <br> <br>

* Basic shell commands: `ls`, `cd`, `pwd`, etc.
* System commands: `echo`, `date`, `whoami`, etc.
* Built-in commands: `quit` or `exit` to exit the shell <br>

Please note that rusty_shell does not support advanced features such as input/output redirection or piping. It focuses on providing a minimalistic shell environment for basic command execution.

## Known Limitations
* rusty_shell is a simple shell implementation designed for educational purposes. It may not include all the features and robustness of production-grade shells.
* The shell does not support advanced features such as input/output redirection or piping. It focuses on demonstrating the basics of command execution and tokenization.
* Error handling is minimal and may not cover all possible scenarios. The shell may display generic error messages in case of failures.

## Acknowledgments
This project is inspired by Pr 2: Shell

## Presentation Link
**Slides:** https://docs.google.com/presentation/d/1swuYxPgi8jTHWa7ATvPPRex1HR-IhHUqeSDx0xeW-Fo/edit?usp=sharing
<br>
**Video:** [https://drive.google.com/file/d/1P4YTuqyeSm6ZBQ3vXBDpFBTpUYnkisGI/view?usp=sharing](https://drive.google.com/file/d/1P4YTuqyeSm6ZBQ3vXBDpFBTpUYnkisGI/view?usp=sharing)
<br>

