# Wye

Wye is a statically typed, functional language that I decided to build in order to learn more about functional languages and compilers. Wye features a standard Hindley-Milner type system with type inference, as well as other things often found in functional languages such as pattern matching and custom types.

For a detailed description of Wye, check out [the spec](/specification/).

# Current Progress

As of right now, Wye has a parser and a preliminary syntax highlighter for VSCode. To use the parser, one can run
```sh
cargo run parse <path to wye program>
```
in order to print a very unpretty parse tree of your input Wye program. The syntax highlighter is stored in the [wye-syntax-highlighter/](/wye-syntax-highlighter/) directory and can be installed from the VSCode extensions marketplace.

# A Glimpse of Wye

```ocaml
(* return remainder of dividing x by y *)
let rem (x: int) -> (y: int) -> int = x - ((x // y) * y);

(*
Generate the Collatz sequence starting at the input x
*)
let collatz_sequence (x: int) -> [int] = match x {
    (* ignore negative numbers *)
    _ if x < 0 => [],
    (* if we're at 1, terminate *)
    1 => [1],
    (* otherwise we add x to the sequence and recursively enumerate the rest;
    if x is even, divide by 2 *)
    _ if (rem x 2) == 0 => x :: (collatz_sequence (x // 2)),
    (* if we reach here, x is odd, so we multiply by 3 and add 1 *)
    _ => x :: (collatz_sequence ((3 * x) + 1))
};

(*
currently Wye does not take user input, so we specify the input manually
*)
let Main = print (collatz_sequence 97);
```

To see more examples of wye, check out the [examples/](/examples) folder.

# Next Steps

- type checker
- interpreter
- register allocator
- code generator
- compiler
