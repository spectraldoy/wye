# Wye

Wye is a statically typed, functional language that I decided to build in order to learn more about functional languages and compilers. Wye features a standard Hindley-Milner type system with type inference, as well as other things often found in functional languages such as pattern matching and custom types.

For a detailed description of Wye, check out [the spec](/specification/).

# Current Progress

As of right now, the only operational aspect of the Wye language is its parser. One can run
```sh
cargo run parse <path to wye program>
```
in order to print a very unpretty parse tree of your input Wye program.

What I want to do next:
- syntax highlighter
- type checker
- interpreter
- register allocator
- code generator
- compiler

# A glimpse of Wye

Below should be an implementation of functionality to obtain the Collatz sequence starting at a given integer.

```rust
let collatz_sequence (x: int) -> [int] = match x {
    _ if x < 0 => [],
    1 => [1],
    _ if (rem x 2) == 0 => {
        let half_x = x / 2;
        half_x :: (collatz_sequence half_x)
    },
    _ => {
        let three_x_plus_1 = (3 * x) + 1;
        three_x_plus_1 :: (collatz_sequence three_x_plus_1)
    }
}

let Main = print (collatz_sequence 81);
```
