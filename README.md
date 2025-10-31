# Rust Zero to Mastery

This repository contains 100 progressively challenging Rust projects. The goal is to help learners move past passive tutorials and gain real experience with memory safety, ownership, lifetimes, concurrency, and systems-level thinking. Each project is organized in its own numbered folder and includes a detailed `README.md` for clarity.

Projects start with basics like variables, pattern matching, and simple command-line tools. They grow into topics such as file parsing, multi-threading, async runtime design, networking, serialization, and performance tuning. Along the way, you will see Rust’s strengths and tradeoffs in real contexts.

All code in this repository is heavily commented and annotated to support problem-solving and deep understanding. The intent is to teach reasoning, not guesswork, and to help break out of “tutorial hell” by showing how real Rust programs evolve.

Video walkthroughs may be added when a project benefits from visual explanation or involves complex mental models.

## Structure

rust-zero-to-mastery/
001-basic-calculator-cli/
002-temperature-converter/
003-uuid-generator/
...
100-advanced-grid-simulation/


Each project includes:
- A problem description
- Requirements and constraints
- Implementation notes
- Common pitfalls
- Fully commented source code
- Suggested extensions

## Who This Is For

- Beginners who want hands-on practice
- Developers moving into systems programming
- Anyone curious about ownership, lifetimes, and memory safety
- Learners who want production-grade habits

## Contributing

Contributions are welcome. Suggested improvements:
- Ownership and borrowing clarifications
- Refactoring ideas
- Tests
- README enhancements

Open a pull request or file an issue if you find bugs, typos, or broken examples.

## License

This repository is free and open source for personal, educational, and commercial use. Content may be copied, modified, and redistributed as long as attribution to the repository is maintained.

Permission is granted to use, share, and adapt the material without restriction. No warranties are provided. Software is provided “as-is.”

## Disclaimer

This repository is for educational purposes only. It does not provide guarantees of correctness, performance, or security. Use your own judgment when applying these examples in production environments.

## Roadmap

- Add optional video walkthroughs for complex systems
- Introduce testing and benchmarking
- Highlight unsafe code boundaries
- Add prompts for self-reflection after each build

## Goals

- Build by doing, not watching
- Understand ownership through real examples
- Develop confidence in reading, writing, and refactoring Rust
- Learn systems-level tradeoffs
- Gain technical range from simple utilities to full applications

Start at the first folder. Move forward only when the concepts are comfortable. Consistency beats speed.



**Rust install steps for three platforms. Use rustup for toolchain and cargo.**

````markdown
# Rust Setup Guide

This guide helps you install Rust on Windows, macOS, or Linux. It uses `rustup`, which manages Rust versions and includes `cargo`.

## Windows

1. Go to https://www.rust-lang.org/tools/install
2. Download the Windows installer
3. Run it and follow the prompts
4. Restart your terminal

## macOS

Open Terminal and run:

```bash
curl https://sh.rustup.rs -sSf | sh
````

Follow the prompts. When finished:

```bash
source $HOME/.cargo/env
```

## Linux

Open your shell and run:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Follow the prompts. When finished:

```bash
source $HOME/.cargo/env
```

## Confirm the installation

Run:

```bash
rustc --version
cargo --version
```

You should see version numbers.

## Using cargo

`cargo` builds, runs, and manages Rust projects.

To create a new project:

```bash
cargo new my_project
```

To run it:

```bash
cd my_project
cargo run
```

## Local documentation

Rust installs offline docs with examples.

Open them with:

```bash
rustup doc
```

This launches your browser. The documentation contains many references and links.

## The Book

When this guide says **“The Book”**, it means the official Rust book included in the documentation you open through `rustup doc`. It explains ownership, borrowing, lifetimes, and more from beginner level.

Read it often.


