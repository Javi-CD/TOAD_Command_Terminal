# Toad Terminal - Simulated Command-Line in Rust

Toad Terminal is a simulated interactive command-line application written in Rust, designed for learning and experimentation with system-like commands.

## Features

- `cd <path>` – Change the current working directory
- `ls` – List files and directories
- `pwd` – Print current working directory
- `echo <text>` – Print text to the terminal
- `clear` – Clear the terminal screen
- `help` – Display list of available commands
- `exit` – Exit the Toad terminal

## Getting Started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Run the terminal

```bash
cargo run
```

### Example usage

```
Toad> pwd
/home/user/WorkSpace/terminal_simulada

Toad> echo Hello Rust!
Hello Rust!

Toad> help
```

## 📦 Project Structure

```
src/
├── main.rs            # Main REPL loop
└── commands/          # Modular commands
    ├── mod.rs
    ├── cd.rs
    ├── ls.rs
    ├── pwd.rs
    ├── echo.rs
    ├── clear.rs
    └── help.rs
```

## 🧠 Educational Goal

This project is designed to help you:
- Learn how to work with Rust modules
- Handle user input/output
- Work with file systems (`std::fs`, `std::env`)
- Build modular and maintainable CLI applications

---

> Toad Terminal is not a shell replacement — it's an educational sandbox.

