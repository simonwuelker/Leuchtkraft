# Leuchtkraft
Leuchtkraft is my tiny, declarative Programming Language.

A basic Leuchtkraft program might look like this:
```
/* Logic clauses */
forall X {
    bird(X) and healthy(X) <=> canfly(X) // only healthy birds can fly
    wounded(X) and healthy(X) => false // A object can't both be healthy and wounded
}

/* Facts */
true => bird(john) and bird(mary) and bird(jane) // john, mary and jane are birds
true => wounded(john) // john is wounded
true => healthy(jane) // jane is healthy

/* Conclusions */
canfly(john) => ? // False (john is not a healthy bird)
canfly(mary) => ? // Indeterminate (mary's health is unknown)
canfly(jane) => ? // True (jane is a healthy bird)
```

## Syntax Highlighting
I only include vim scripts because if you are not using vim, are you really a programmer?

Copy the highlighting script (`le.vim`) to the `~/.vim/syntax/le.vim`.
Then create `~/.vim/ftdetect/le.vim` and write 
`au BufRead,BufNewFile *.le set filetype=le` to it to associate .le files 
with leuchtkraft scripts.
