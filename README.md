# lambda

![Static Badge](https://img.shields.io/badge/Built_with-C-blue?logo=c)

An interpreted Lua-like scripting language hobby project. The implementation is **entirely in C**. This language takes a lot of inspiration from Lua in terms of portability *and* implementation style. There is no real *goal* to this project besides seeing if this can even be done when working part-time on it.

Lambda is interpreted, since I can't even imagine how complex writing a compiler would be.

If you would like to contribute to the cause, you're more than welcome to! Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) and [Code of Conduct](./CODE_OF_CONDUCT.md) before you do! Just remember that the only dependencies should come from either the C standard library and to use `#preprocessors` when dealing with OS-specific functions, such as file handling on POSIX over Windows.

C was chosen over C++ since C is simpler and has far fewer features. The smaller feature set of C reduces overall complexity and makes it easier to maintain and audit code, which is especially valuable in this environment, where reliability and long-term support are vital. C’s portability across platforms and compilers also ensures that the codebase can be reused or deployed in diverse systems with minimal mods. By avoiding the overhead of C++’s object-oriented features and additional overhead the resulting libraries can be far more lean.

## Writing Lambda

> [!TIP]
> This is unfinished, and subject to change!

## Using the Interpreter

> [!CAUTION]
> The interpreter itself is finished and likely contains bugs and pitfalls, use this carefully!