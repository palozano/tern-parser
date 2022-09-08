# Rust Developer Task - Arithmetic Parser

Requirements for the solution:
* provide your solution in **Rust**
* you may not use any external libraries to solve this problem
* you should provide sufficient evidence that your solution is complete by, as a minimum, indicating that it works correctly against the supplied test data.


## The test

Implement a parser to take a string and compute its numerical value using the given rules.

Operators should be applied in order of precedence from left to right. An exception to this is brackets which are used to explicitly denote precedence by grouping parts of an expression that should be evaluated first.

### Rules

a = ‘+’, 
b = ‘-’, 
c = ‘*’, 
d = ‘/’, 
e = ‘(’, 
f = ‘)’

### Acceptance Criteria

Input: “3a2c4” ->
Result: 20

Input: “32a2d2" ->
Result: 17

Input: “500a10b66c32” ->
Result: 14208

Input: “3ae4c66fb32" ->
Result: 235

Input: “3c4d2aee2a4c41fc4f” ->
Result: 990

## Solution

Submit the solution via a personal github link and please be sure to document how to run everything.

## How to run the code

If you use `cargo`, you can run `cargo run` and the output will be printed for the provided data. Running `cargo test` runs the tests asserting that the solution matches the actual result.

You can also use `rustc` to compile the code and then run the binary.