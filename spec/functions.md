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
| `typeof()`   | `fn type(value: any) -> string`      | Returns the type of the `value` in a `string`.       |
| `length()`   | `fn len(content: string) -> integer` | Returns the length of the `content` as an `integer`. |
| `assert()`   | `fn assert(expression: bool, message: string) -> null` | Asserts `expression`, stops execution if it fails and produces `message`. |

## 2. Standard Library
<sub>Coming soon...</sub>