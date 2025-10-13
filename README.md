# Lambda

![Static Badge](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust&logoColor=%23FFFFFF)

<!-- Licenses -->
![Static Badge](https://img.shields.io/badge/Compiler-Apache--2.0-red)
![Static Badge](https://img.shields.io/badge/Spec-CC--BY--ND--4.0-white?logo=creativecommons&logoColor=%23FFFFFF)

<!-- Quality code badge :3 -->
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/03886551730540f8ad5a39914370587a)](https://app.codacy.com/gh/samdoesnerdstuff/lambda/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade)

Lambda is a lightweight, procedural programming language with the simplicity of Lua and the performance of C.

```
    ⣶⣶⣶⣶⡆               lambda:
    ⠛⠛⢻⣿⣿⡀                * simple & elegant
    ⠀⠀⢀⣿⣿⣷                 * expressive
    ⠀⢀⣾⣿⣿⣿⣇                 * powerful
⠀ ⠀⠀⢠⣿⣿⡟⢹⣿⣿⡆
⠀   ⣰⣿⣿⠏⠀⠀⢻⣿⣿⡄
⠀  ⣴⣿⡿⠃⠀⠀⠀⠈⢿⣿⣷⣤⣤⡆⠀
⠀⠀⠾⠿⠿⠁⠀⠀⠀⠀⠀⠘⣿⣿⡿⠿⠛
```

## What *is* Lambda?

At its core, Lambda is a compiled, statically-typed procedural programming language with features for Object-Oriented Programming. It's been intentionally designed for rapid development, ease of use and expressive programming.

**Sample, the fibonacci sequence as a Lambda function:**
```
fn fib(n: integer) -> integer
    if n <= 1 then return n end
    return fib(n - 1) + fib(n - 2)
end

/! It's assumed that this is an integer so no types are needed
/! This may change in future revisions however
for i = 0; i < 10; i++ do
    print(fib(i))
end
```

This language has been heavily inspired by languages like Lua, C and type-strict languages. 

## What makes Lambda different?

Lambda is unlike any language in its ecosystem. Designed to be elegant, expressive, and powerful. It brings together the compiled performance of native applications with the DX and the rapid prototyping of Lua. Its strong type system is derived from languages like C++ and Java where types are always required. Its OOP model is directly based off of Python's OOP systems.

Lambda also plans on having an FFI for C libraries, immediately opening up a **vast library** of software from all corners of the programming space.

Lambda also has a simple, but powerful buildsystem. Much like other compiler projects before it, Lambda is purely CLI and is planned on having a vast array of build flags, optimizations, inclusions, everything a developer would need.

## Who is Lambda *for?*

Lambda is for the developer who just wants a language that's very fast to write, can handle its own memory and just *works* for general purpose development. Lambda was not designed with one specific paradigm in mind 24/7, it was designed to be a strongly typed language that almost anybody can pick up and learn in a few weeks, like Python!

## Getting Started

Right now, installing Lambda globally is not possible, it can only be run locally from source. To do so, you'll need the Rust programming language installed on your machine:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have Rust, you can go ahead and clone this repository:

```sh
git clone https://github.com/samdoesnerdstuff/lambda.git --depth 1
```

Once you have the repository cloned, it's as easy as running `cargo build` and you're off. Due to the size of this project, compilation times may be a bit long.

**Example:**
```sh
cargo build --release
./target/release/lamc -s ./examples/hello.lm
```

## Licenses

**Compiler and Standard Library**  
Licensed under [Apache 2.0](./LICENSE).

**Language Specification**  
Licensed under [CC BY-ND 4.0](./spec/LICENSE).  
You may read and share it freely, but not distribute modified versions.

## Contributing / Security

If you'd like to contribute to Lambda and its specification, amazing! ✨ \
Check out the [Contributing](./.github/CONTRIBUTING.md) document for more info!

If you're here to report a security issue, refer to [Security](./.github/SECURITY.md) for relevant information.
