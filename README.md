# Risp

A simple Scheme-like programming language interpreter implemented in Rust:

```scheme
(define fib (lambda (n) (begin
  ; A recursive function
  (define go (lambda (n a0 a1) (if
    (<= n 0)
    a0 
    (go (- n 1) a1 (+ a0 a1))
  )))
  (go n 0 1)
)))

(display (fib 80))(newline)

(define mutual (lambda (n) (begin
  ; Mutually recursive functions
  (define dec (lambda (n) (check (- n 1))))
  (define check (lambda (n) (if (<= n 0) 'done (dec n))))
  (check n)
)))

(display (mutual 100000))(newline)
```

```sh
cat examples/recursion.rsp | ./risp
23416728348467685
'done
```

This is a project for learning purposes that does not follow any Scheme standard. The goal is to offer lexical scope, tail-call optimization, sharing of symbols, evaluate data as code and mostly efficient memory management in a simple way. The implementation is generic to allow selecting at compile time the types of booleans and numbers, as in [this main program](./src/main.rs):

```rust
parser.parse_all_exps::<bool, i64>(input)
```

It will also work for example with any other type that implements `From<bool>` and `Into<bool>` for booleans and `i32` for numbers.

## Implemented special forms

`define`, `if`, `begin`, `lambda`, `quote`, `eval`, `and`, and `or`.

The absence of `set!` allows to easily clean up the environment in some cases during evaluation.

## Built-in procedures

`=`, `>`, `<`, `>=`, `<=`, `+`, `*`, `-`, `eq?`, `display`, `newline`, `true` and `false`.
