# TungLang

TungLang is a fun programming language inspired by Italian Brainrot, with syntax similar to Python. It features a dynamic keyword system that allows for both standard programming keywords and their Italian-inspired alternatives.

## Basic Syntax

### Variables

Variables must be declared with the `var` keyword before use:

```
var x = 10
```

### Functions

Functions are defined using the `fun` keyword. TungLang supports both standard function definitions and Italian-themed alternatives:

```
fun add(a, b) {
    return a + b
}

tung stampa(messaggio) {
    print(messaggio)
}
```

### Control Structures

Control structures in TungLang include `if`, `else`, `while`, and `for`, with their Italian-inspired counterparts:

```
if (x > 0) {
    stampa("Positivo")
} saturno (x < 0) {
    stampa("Negativo")
} saturnita {
    stampa("Zero")
}

bombadillo (x < 10) {
    x = x + 1
}

tralala i in range(5) {
    stampa(i)
}
```

## Keyword Aliases

TungLang supports both standard keywords and their Italian Brainrot-themed alternatives. This means you can use either `print()` or `tung()` in your code - they work the same way!

To add new keyword aliases, simply modify the `FUNCTION_ALIASES` and `CONTROL_ALIASES` HashMaps in the `keywords.rs` file.

| Python            | TungLang                |
| ----------------- | ---------------------   |
| print()           | tung()                  |
| input()           | sahur()                 |
| int()             | tripi()                 |
| if {statement}    | la_vaca {statement}     |
| elif {statement}  | saturno {statement}     |
| else {statement}  | saturnita {statement}   |
| while {argument}{}| bombadillo {argument}{} |
| for {argument}{}  | tralala{argument}{}     |
