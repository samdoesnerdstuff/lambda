# Lambda Syntax Specification
<sub>SPEC VERSION: 0.1-alpha</sub>

This document defines the **syntax rules**, keywords, operators, and identifiers for the Lambda programming language.

---

## 1. Comments

Lambda supports single-line comments with `/!`:

```
/! This is a comment, woah!
```

At this time, multi-line comments are not supported.

## 2. Identifiers

 * Must start with a letter (`A-Z` or `a-z`) or underscores `_`
 * Can contain letters, digits (`0-9`) and underscores `_`
 * Case-sensitive

**Samples:**
```
let myVar = null
local fn _my_private_func() -> null
const count2 = 700
```

## 3. Keywords

Reserved keywords cannot be used as identifiers:

| Keyword  | Description                     |
| -------- | --------------------------------|
| `fn`     | Function declaration            |
| `class`  | Class declaration               |
| `let`    | Variable decl (scoped)          |
| `const`  | Constant declaration            |
| `if`     | Conditional                     |
| `else`   | Conditional alternative         |
| `for`    | Loop                            |
| `while`  | Loop                            |
| `return` | Return from function            |
| `end`    | Block terminator                |
| `then`   | Conditional continuation        |
| `do`     | Loop continuation               |
| `require`| Import module                   |
| `local`  | Module-local declaration        |
| `in`     | Used in for loops               |
| `and`    | Logical AND                     |
| `or`     | Logical O                       |
| `not`    | Logical NOT                     |
| `break`  | Break out of loop immediately   |

## 4. Operators

| Operator | Description              |
| -------- | ------------------------ |
| `+`      | Addition                 |
| `-`      | Subtraction              |
| `*`      | Multiplication           |
| `/`      | Division                 |
| `==`     | Equal to                 |
| `!=`     | Not equal to             |
| `<`      | Less than                |
| `>`      | Greater than             |
| `<=`     | Less than or equal       |
| `>=`     | Greater than or equal    |
| `=`      | Assignment               |
| `++`     | Increment                |
| `--`     | Decrement                |
| `:`      | Type annotation / labels |
| `->`     | Return type annotation   |
| `.`      | Member access            |

## 5. Block Structures

Lambda uses explicit end keywords to close blocks.

```
fn greet(name: string) -> null
    if name == "Alice" then
        print("Hello, Alice!")
    else
        print("Hello, stranger!")
    end
end
```

Functions, classes, loops and conditionals are all terminated by an `end` keyword. \
Indentation is technically optional but absolutely recommended.

## 6. String Literals

Single or double quotes supported, escape sequences exist with `\`:

```
let message: string = "Hello, World!\n"
let character: string = 'a'
```

## 7. Numbers and Integers

* Integers: `42`, `-9`
* Floats: `3.14`, `-0.00002`

## 8. Sample Code

```
/! Fibonacci sequence in Lambda
fn fib(n: integer) -> integer
    if n <= 1 then return n end
    return fib(n - 1) + fib(n - 2)
end

for i = 0; i < 100; i++ do
    print(fib(i))
end
```

---

That concludes core syntax rules. See [types.md](./types.md) for the type system, [functions.md](./functions.md) for standard library functions, and [modules.md](./modules.md) for rules on Modules.

---