# Project Structure

The project is organized as follows:

```dir
/TungLang/
├── src/
│   ├── main.rs             # Entry point for the interpreter
│   ├── parser/             # Grammar definition and parsing
│   │   ├── mod.rs          # Parser module definition
│   │   └── parser.pest     # PEG grammar rules
│   ├── interpreter/        # Code execution logic
│   │   ├── mod.rs          # Main interpreter structure
│   │   ├── assignment.rs   # Variable assignments
│   │   ├── binary_expr.rs  # Mathematical operations
│   │   ├── control_flow.rs # If/elif/else statements
│   │   ├── expression.rs   # Expression evaluation
│   │   └── function_call.rs# Function handling
│   └── value.rs            # Value types and operations
```

## Key Components

### 1. Grammar Definition (`/src/parser/parser.pest`)

The language syntax is defined using Pest grammar rules. This is where you define what the language accepts as valid syntax.

* `file` - The top-level rule for a TungLang file
* `statement` - Individual actions like assignments or function calls
* `expression` - Values, variables, and operations
* `if_statement` - Control flow constructs

### 2. Interpreter (`/src/interpreter/`)

The interpreter executes the parsed syntax tree:

* `mod.rs` - Contains the main `Interpreter` struct and entry points
* `assignment.rs` - Handles variable assignments
* `expression.rs` - Evaluates expressions and values
* `binary_expr.rs` - Processes mathematical operations
* `function_call.rs` - Handles function calls like `print()`
* `control_flow.rs` - Manages if/elif/else statements

### 3. Values (`/src/value.rs`)

Defines the types of values supported in the language and operations on them.

## Extending the Language

### Adding New Value Types

To add a new value type (e.g., Boolean):

1. Modify `value.rs` to add a new variant to the `Value` enum:

   ```rust
   pub enum Value {
       Number(i64),
       String(String),
       Boolean(bool),
   }
   ```

2. Update the `Display` and `type_name` implementations.

### Adding New Operators

To add a new operator (e.g., modulo `%`):

1. Add the operator to the grammar in `parser.pest`:

   ```pest
   operator = { "+" | "-" | "*" | "/" | "%" }
   ```

2. Update `binary_expr.rs` to handle the new operator:

   ```rust
   // In evaluate_binary_expr
   "%" => Value::Number(left_num % right_num),
   ```

### Adding New Control Structures

To add a new control structure (e.g., `while` loop):

1. Add the syntax to `parser.pest`:

   ```pest
   statement = { assignment | function_call | if_statement | while_loop }
   
   while_loop = {
       "while" ~ "(" ~ expression ~ ")" ~ ":" ~ NEWLINE* ~
       statement* ~
       "endwhile"
   }
   ```

2. Create a handler in `control_flow.rs`:

   ```rust
   pub(super) fn handle_while_loop(&mut self, mut pairs: Pairs<Rule>) {
       // Implementation here
   }
   ```

3. Update `handle_statement` in `mod.rs` to call your new handler.

### Adding Built-in Functions

To add a built-in function (e.g., `input()`):

1. Update the `handle_function_call` method in `function_call.rs`:

   ```rust
   if function_name == "print" {
       // Existing print logic
   } else if function_name == "input" {
       // New input function logic
   } else {
       eprintln!("Unknown function: {}", function_name);
   }
   ```

## Example TungLang Code

```tung
# Variable assignment
x = 5
y = "Hello"

# If statement
if (x == 5):
    print("x is 5")
elif (x == 4):
    print("x is 4")
else:
    print("x is something else")
endif

# Function call
print(x)
print(y)
print(x + 3)
```

## Running TungLang

To run a TungLang script, use:

```sh
cargo run your_script.tung
```

## Future Enhancements

Some ideas for future language features:

1. User-defined functions
2. Loops (while, for)
3. More data structures (arrays, maps)
4. Import system for modularization
5. Standard library with common functions

## Contributing

To contribute to TungLang:

1. Make sure to understand the component you're modifying
2. Add tests for new features
3. Keep the language syntax consistent
4. Document your changes

Happy coding with TungLang!
