# Wye

Wye is a statically typed, functional language that I decided to build in order
to learn more about functional languages and compilers. Wye features a standard
Hindley-Milner type system with type inference, as well as other things often
found in functional languages such as pattern matching and custom types.

For a detailed description of Wye, check out [the spec](/specification/).

# Current Progress

As of right now, Wye has a parser and a preliminary syntax highlighter for
VSCode. To use the parser, one can run
```sh
cargo run parse <path to wye program>
```
in order to print a very unpretty parse tree of your input Wye program. The
syntax highlighter is stored in the
[wye-syntax-highlighter/](/wye-syntax-highlighter/) directory and can be
installed from the VSCode extensions marketplace.

# A Glimpse of Wye

```ocaml
TODO
```

To see more examples of wye, check out the [examples/](/examples) folder.

# Next Steps

- type checker
- interpreter
- register allocator
- code generator
- compiler
