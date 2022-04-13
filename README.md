# Leuchtkraft
Leuchtkraft is my tiny, declarative Programming Language with absolutely minimal
dependencies.
A basic Leuchtkraft program might look like this:
```
// Logic Clauses
forall X
    bird(X) and healthy(X) => canfly(X) // Healthy birds can fly
    canfly(X) => healthy(X)             // Things that fly are healthy
    wounded(X) and healthy(X) => false  // Things can't both be healthy and wounded

// Facts
true => bird(john) and bird(mary) and bird(jane) // john, mary and jane are birds
true => wounded(john) // john is wounded
true => healthy(jane) // jane is healthy

// Conclusions
X? => canfly(john) => X? // False (john is not a healthy bird)
Y? => canfly(mary) => Y? // Indeterminate (mary's health is unknown)
Z? => canfly(jane) => Z? // True (jane is a healthy bird)
```

Check out [my blog post](https://wuelle.dev/blog/posts/leuchtkraft/) for more
information on how to use leuchtkraft.

## Installation
### From crates.io
```
cargo install leuchtkraft
```
### From source
```
git clone https://github.com/Wuelle/Leuchtkraft
cd Leuchtkraft
cargo install --path .
```

Execute `leuchtkraft --help` for basic usage information.

## Road Map
In order or priority: 
- [x] Custom zero-clone parser
- [x] Logic resolver
- [x] Awesome build warnings/errors
- [ ] WASM app for testing
- [x] REPL
- [ ] Var-level unknowns (`canfly(X?)`)
- [ ] Compiler and (if we feel really fancy) JIT


## Syntax Highlighting
I only include vim scripts because if you are not using vim, are you really a programmer?

Copy the highlighting script (`le.vim`) to the `~/.vim/syntax/le.vim`.
Then create `~/.vim/ftdetect/le.vim` and write 
`au BufRead,BufNewFile *.le set filetype=le` to it to associate .le files 
with leuchtkraft scripts.
