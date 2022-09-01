# Risp

A simple Scheme-like programming language interpreter implemented in Rust:

```scheme
(define fib (lambda (n) (begin
  (define go (lambda (n a0 a1) (if
    (<= n 0)
    a0 
    (go (- n 1) a1 (+ a0 a1))
  )))
  (go n 0 1)
)))

(display (fib 80))(newline)

(define mutual (lambda (n) (begin
  (define dec (lambda (n) (check (- n 1))))
  (define check (lambda (n) (if (<= n 0) 'done (dec n))))
  (dec n)
)))

(display (mutual 100000))(newline)
```

```sh
cat examples/recursion.rsp | cargo run -r
    Finished release [optimized] target(s) in 0.03s
      Running `target/release/risp`
23416728348467685
'done
```
