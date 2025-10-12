# Lambda Type System
<sub>SPEC VERSION: 0.1-alpha</sub>

Lambda’s type system is **static**, **strong**, and **explicit**.  
All variables, parameters, and function returns have a defined type at compile time.  
No implicit conversions are performed between unrelated types.

---

## 1. Primitive Types

| Type      | Description                                                | Example           |
| --------- | ---------------------------------------------------------- | ----------------- |
| `integer` | 64-bit signed integer.                                     | `42`, `-15`       |
| `float`   | 64-bit floating-point number.                              | `3.14`, `-0.001`  |
| `bool`    | Boolean truth value.                                       | `true`, `false`   |
| `string`  | Immutable UTF-8 encoded text.                              | `"Hello, world!"` |
| `null`    | The absence of value. Only valid as an explicit literal.   | `null`            |

---

## 2. Composite Types

### 2.1 `list`

A mutable, dynamically sized sequence of dynamic values.  
Lists can store mixed types, and elements are indexed from **0**.
<!-- indexing by anything different than zero feels like an offense -->

**Example:**

```lm
let data: list = [1, "two", 3.0]
data.append("four")
write(to_str(data[0]))
```

**Supported Operations:**

| Operator   | Description                  |
| ---------- | ---------------------------- |
| `+`        | Concatenates two lists.      |
| `[]`       | Index access.                |

---

### 2.2 `tuple`

An immutable, ordered collection of fixed length.  
Used mainly for grouping values and returning multiple results.

**Example:**

```lm
fn split() -> tuple
    return ("foo", "bar")
end

let (a, b) = split()
```

**Supported Operations:**

| Operator   | Description              |
| ---------- | ------------------------ |
| `[]`       | Index access.            |
| `==`, `!=` | Element-wise comparison. |

---

## 3. Type Inference

Type inference may occur when the compiler can **unambiguously** determine the type of a value.

```lm
let x = 42        /! inferred as integer
let y = 3.14      /! inferred as float
let s = "hello"   /! inferred as string
```

If the compiler cannot infer a type, an explicit type annotation is required.

---

## 4. Type Conversion

Lambda does **not** perform implicit coercion.
All conversions must be explicit via intrinsic functions:

* `to_str(value)`
* `to_int(value)`
* `to_float(value)`
* `to_bool(value)`

---

## 5. Null Handling

`null` is a valid literal but cannot be assigned to variables unless explicitly initialized that way.  
Operations on `null` values are invalid and result in runtime errors.  
All types can be coerced from their actual type into `null`.

```lm
let x = null
x = 3     /! OK
x = null  /! still allowed but not implicit
```

```lm
if some_var != null then
  io.write("%some_var isn't null!")
else
  io.write("%some_var is null! D:")
end
```

Lambda treats `null` as a **sentinel** rather than a nullable type modifier — it is the programmer’s responsibility to guard against it.

---

