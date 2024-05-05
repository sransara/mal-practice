# mal-practice
My attempt of https://github.com/kanaka/mal/

- [x] eval "def!"
- [x] eval "let*"
- [x] eval "macro*"
- [x] eval "quote"
- [x] eval "quasiquote"
- [x] `pr_str` support
- [x] `if`, `fn*`, `do` support
- Rust macro to support builtins in ./src/builtins.rs
- [x] `add`, `sub`, `lte`, `list`, `throw` builtins

Sample mal program that can be executed:
```
(def! fib (fn* (n)
  (if (lte n 1)
    n
    (add (fib (sub n 1)) (fib (sub n 2))))))
(fib 10)
```

Sample run:
```
cat example/fib_test.mal | cargo run

# output
Function(User { params: [Symbol("n")], body: List([Symbol("if"), List([Symbol("lte"), Symbol("n"), Integer(1)]), Symbol("n"), List([Symbol("add"), List([Symbol("fib"), List([Symbol("sub"), Symbol("n"), Integer(1)])]), List([Symbol("fib"), List([Symbol("sub"), Symbol("n"), Integer(2)])])])]) })
Integer(55)
```
