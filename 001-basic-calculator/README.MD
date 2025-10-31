````markdown
# Basic Calculator in Rust

This project is a simple calculator written in Rust. It takes two numbers and an operand from user input, then prints the calculated result. It is a beginner-friendly example that demonstrates common Rust patterns such as reading from standard input, parsing values, using control flow, and returning values from functions.

If you see references to **“The Book”**, it means the official Rust book included in the documentation you can open with:

```bash
rustup doc
````

---

## What this program teaches

By reading through the code you can learn:

* How to read user input with `io::stdin().read_line()`
* Converting `String` input to numeric types using `.trim().parse()`
* Why the `String` type is growable and owned
* How Rust returns values from expressions without semicolons
* How `if` expressions work
* How to write and call functions
* Basic ownership transfer when passing values to functions

All of these topics are well covered in **The Book**.

---

## Supported operations

This calculator currently supports:

* `+` addition
* `-` subtraction
* `*` multiplication
* `/` division
* `%` remainder (modulos)

Division by zero is handled by returning `0` as a fallback.

---

## How the program works

The program runs as follows:

1. Prompts for a first number
2. Reads it as a `String`
3. Trims whitespace and parses it into `u64`
4. Prompts for an operand symbol
5. Reads and trims the operator
6. Prompts for a second number
7. Parses it the same way as the first
8. Prints the expression for clarity
9. Calls the `calculate` function
10. Prints the result

---

## The calculation function

The logic is handled inside:

```rust
fn calculate(first_number: u64, operand: String, second_number: u64) -> u64
```

This function:

* Accepts two numbers and an operand
* Compares the operand with string literals
* Uses `if` and `else if` chains
* Returns a `u64` based on the operation

Rust treats `if` as an expression, so the final value of each branch is used as the return value. A missing semicolon means “return this”.

All branches must return the same type, which is why the fallback `else` returns `0`.

---

## Handling invalid input

This code uses:

```rust
.expect("")
```

This will cause the program to panic if parsing fails.

For a real application, you would handle errors more gracefully. For now, this keeps the example simple.

---

## Running the project

Make sure you have Rust and Cargo installed. If not, install using rustup:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Open the project folder from a terminal and run:

```bash
cargo run
```

Follow the prompts.

---

## Example interaction

```
Hello there this is a basic calculator application, usage instructions are below
Type in the first number then pick the operand then pick the second number…
First number:
12
Type in the operand…
+
Second number:
30
12 + 30
Result: 42
```

---

## Learning resources

Open offline docs:

```bash
rustup doc
```

Recommended reading sections from **The Book**:

* “Common Programming Concepts”
* “Understanding Ownership”
* “Functions”
* “Control Flow”
* “Common Collections”
* “Error Handling”

---

## Next steps

Ideas to extend this project:

* Loop so the user can calculate repeatedly
* Better error messages
* Support floating-point numbers
* Use a `match` expression instead of many `if` branches

These improvements are also discussed in **The Book**.

---
