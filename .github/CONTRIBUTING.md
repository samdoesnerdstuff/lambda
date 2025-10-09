# Contributing to Lambda

Thanks for taking the time to help improve Lambda!

Whether you’re filing an issue, suggesting a feature, or contributing code, this guide explains how to get started.

---

## Getting Started

### 1. Build the Compiler
Lambda’s bootstrap compiler is written in **Rust**.

```sh
# Clone the repo
git clone https://github.com/samdoesnerdstuff/lambda.git
cd lambda

# Build the compiler
cargo build
```

You'll find the compiler in `target/debug/lamc`

### 2. Run Tests

Tests verify the lexer, parser, and early runtime behavior.

```sh
cargo test
```

### 3. Try out an example

Run one or multiple of the Lambda example files:

```sh
# --run compiles and runs the code automatically!
./target/debug/lamc --run examples/hello.lm
```

## Code Style
**Rust:**
 - Follow rustfmt defaults (cargo fmt).
 - Keep functions small and focused.
 - Use clear naming for modules: lexer, parser, ast, backend.

**Lambda source:**
 - 4-space indentation
 - lowercase keywords
 - no semicolons unless in loops
 - comments start with /!

## Submitting Changes
Fork the repository and create a feature branch (git checkout -b feature/your-idea). Make your changes with clear, atomic commits. Ensure cargo test passes.

Open a Pull Request to main with a descriptive title. If your PR affects the language specification, `include: Updated snippets in spec/` Explanations in the PR body for reviewers.

## Communication

 - *Feature ideas / questions:* Use the [Discussions](https://github.com/samdoesnerdstuff/lambda/discussion)
 - *Specification feedback:* Use the **Specification Feature Request** template.
 - *Compiler bugs:* Use the **Compiler Issue** template.

## Recognition

Contributors are listed in the release notes and, once the project stabilizes, in a future `AUTHORS.md` file.

Thanks again for helping build Lambda — procedural power, C speed, Lua soul.

---