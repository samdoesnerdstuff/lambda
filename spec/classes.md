# Classes in Lambda

Lambda supports **object-oriented programming** via the `class` keyword.
Classes encapsulate data (fields) and behavior (methods).

## 1. Defining Classes

Classes are declared using the `class` keyword, followed by a name and an indented body:

```lambda
require 'std/io.lm'

class Dog
    fn __init(self, name: string)
        self.name: string = name
    end

    fn bark(self)
        io.print("%self.name says woof!")
    end
end
```

Here:

* `__init` is the **constructor**, automatically called when a class is instantiated.
* The special variable `self` refers to the instance being created.

## 2. Creating Instances

Instances are created by calling the class name like a function:

```lambda
mydog = Dog("Buddy")
mydog.bark()
```

## 3. Fields and Methods

Fields are attached to `self` within class methods:

```lambda
self.fieldname: type = value
```

Fields are dynamically added to instances during initialization.

Methods are simply functions declared within a class body.
They are bound to instances automatically, so `self` always refers to the current object.

## 4. Inheritance

Classes may inherit from one parent class:

```lambda
require 'std/io.lm'

class Cat extends Animal
    fn __init(self, name: string)
        super.__init(name)
    end

    fn speak(self)
        io.print("%self.name says meow!")
    end
end
```

Use `extends` to declare inheritance.
Use `super.method()` to call methods from the base class.

## 5. Special Methods

| Name       | Called When             | Purpose                          |
| ---------- | ----------------------- | -------------------------------- |
| `__init`   | Object creation         | Constructor logic.               |
| `__str`    | Used in string contexts | Returns a string representation. |
| `__del`    | Before object deletion  | Cleanup logic (optional).        |

Example:

```lambda
fn __str(self) -> string
    return "Dog(" + self.name + ")"
end
