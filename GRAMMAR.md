# TungLang Grammar and Function Documentation

## Grammar Overview

- **Variables:**
  - `var name = value`
  - Example: `var age = 10`

- **Input:**
  - `input(prompt)`
  - Example: `var name = input("Enter name: ")`

- **If Statement:**
  - `if condition { ... } else { ... }`
  - Example:
  
    ```tung
    if name == "kaiden" {
        print("Welcome Kaiden")
    } else {
        print("Go away, " + name)
    }
    ```

- **Print:**
  - `print(value)`
  - Example: `print("Hello")`

- **String Concatenation:**
  - Use `+` to join strings: `"Hello, " + name`

## Functions

- **input(prompt: String) -> String**
  - Prompts the user and returns input as a string.
  - Example: `var name = input("Enter name: ")`

- **print(value: String)**
  - Prints a string to the output.
  - Example: `print("Hello")`

## Example Program

```tung
var name = input("Input name: ")
if name == "kaiden" {
    print("Welcome Kaiden")
} else {
    print("Go away, " + name)
}
```
