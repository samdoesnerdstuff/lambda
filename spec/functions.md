# Lambda Standard Library and Intrinsic Functions
<sub>SPEC VERSION: 0.1-alpha</sub>

This document defines the **built-in functions** and **standard library modules** available in Lambda.

Lambda’s function system is statically typed. Function declarations use the `fn` keyword, with optional parameters and explicit return types:

```lm
fn name(param: type, other: type) -> return_type
    /! Function body
    /! Very cool
end
```

## 1. Intrinsic Functions

These are functions that you can use without calling on any modules.

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `write()`    | `fn write(content: string) -> null`  | Writes string content to stdout without formatting.  |
| `read()`     | `fn read(prompt: string) -> string`  | Takes in user input from stdin with a prompt.        |
| `to_str()`   | `fn to_str(value: any) -> string`    | Type coercion from `any` to a `string`.              |
| `to_int()`   | `fn to_int(value: any) -> integer`   | Attempt to coerce `value` into an `integer`.         |
| `to_float()` | `fn to_float(value: any) -> float`   | Attempt to coerce `value` into a `float`.            |
| `to_bool()`  | `fn to_bool(value: any) -> bool`     | Converts any `value` into a `bool`.                  |
| `typeof()`   | `fn typeof(value: any) -> string`      | Returns the type of the `value` in a `string`.       |
| `length()`   | `fn len(content: string) -> integer` | Returns the length of the `content` as an `integer`. |
| `assert()`   | `fn assert(expression: bool, message: string) -> null` | Asserts `expression`, stops execution if it fails and produces `message`. |

> [!NOTE]
> When passing a variable to `write()` or `io.print()` to display, it **has** to be prefixed with **%**.

## 2. Standard Library

The **Lambda Standard Library** (often abbreviated as *stdlib* or *std*) provides a core set of modules that extend the language’s capabilities. These modules are loaded using the `require` directive and form the foundation of I/O, mathematics, and system operations.

Unless otherwise specified, all standard library modules are statically linked and available at compile time. Each module exports its public functions under a namespaced prefix (for example, `io.write()` from `std/io.lm`).

---

### 2.1 std/io.lm - Input/Output

Provides basic text input and output.

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `io.print()` | `fn print(data: string) -> null`                | Writes text to stdout, a newline is inserted at the end of the data. |
| `io.warn()`  | `fn warn(message: string) -> null`              | Writes a warning message to stdout.                                  |
| `io.error()` | `fn error(message: string, stop: bool) -> null` | Writes an error to stderr, stops exec if `stop` is true.             |

---

### 2.2 std/math.lm - Math Utilities

Provides utilities and constants for math outside arithmetic.

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `math.abs()`   | `fn abs(x: float) -> float`                           | Absolute value of `x`.               |
| `math.sqrt()`  | `fn sqrt(x: float) -> float`                          | Square root of `x`.                  |
| `math.pow()`   | `fn pow(base: float, exp: float) -> float`            | Exponentiation.                      |
| `math.sin()`   | `fn sin(radians: float) -> float`                     | Sine of `radians`.                   |
| `math.cos()`   | `fn cos(radians: float) -> float`                     | Cosine of `radians`.                 |
| `math.tan()`   | `fn tan(radians: float) -> float`                     | Tangent of `radians`.                |
| `math.min()`   | `fn min(a: float, b: float) -> float`                 | Returns the smaller of `a` and `b`.  |
| `math.max()`   | `fn max(a: float, b: float) -> float`                 | Returns the larger of `a` and `b`.   |
| `math.clamp()` | `fn clamp(x: float, min: float, max: float) -> float` | Restricts `x` within [`min`, `max`]. |
| `math.log()`   | `fn log(x: float, base: float) -> float`              | Logarithm function.                  |
| `math.ln()`    | `fn ln(x: float) -> float`                            | Natural log function.                |

**Constants:**
 - math.pi  = 3.14159265358973
 - math.phi = 1.61803399874984
 - math.e   = 2.71828182845904

**Notes:**
- All trigonometric functions assume input in **radians**.
- All logarithmic functions return results in **natural base e** unless a base is explicitly provided.


---

### 2.3 std/sys.lm - System and Environment

Provides system-level utilities and runtime functionality. Also provides time-related functions.

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `sys.exit()`      | `fn exit(code: integer) -> null`    | Exits program with exit code.                 |
| `sys.uptime()`    | `fn uptime() -> float`              | Returns program uptime in seconds.            |
| `sys.timesince()` | `fn timesince() -> float`           | Returns seconds since the UNIX epoch started. |
| `sys.sleep()`     | `fn sleep(duration: float) -> null` | Pauses execution for `duration` seconds.      |

---

### 2.4 std/string.lm - String Utilities

Utilities for string manipulation.

| Function | Signature | Description |
| -------- | --------- | ----------- |
| `string.upper()` | `fn upper(s: string) -> string`                        | Converts `s` to uppercase.                        |
| `string.lower()` | `fn lower(s: string) -> string`                        | Converts `s` to lowercase.                        |
| `string.find()`  | `fn find(haystack: string, needle: string) -> integer` | Returns index of `needle` or `-1`.                |
| `string.trim()`  | `fn trim(s: string) -> string`                         | Trims whitespace out of `s`.                      |
| `string.sub()`   | `fn sub(s: string, old: string, new: string) -> string`| Returns new string where all `new` replace `old`. |
| `string.split()` | `fn split(s: string) -> list`                          | Splits a string into a list of individual characters. |

---
