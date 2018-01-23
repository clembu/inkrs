# inkrs

`inkrs` is a port of [Inkle's ink](https://github.com/inkle/ink)
in [rust](https://www.rust-lang.org),
not the compiler.

## the parsing tree

When given a story `.json` file,
inkrs will build a `Story` structure
that contains a tree structure
corresponding to a typed version of the json file.

This parsed tree will contain **unsafe** values.
For example, a `variable reference` to
an ink variable that has not yet been given a value.

Due to the non-linear nature of an `ink` story,
variables can be initialized much later in the file
than its first reference, or even outside of the story flow.

## Todo : Deserialization

### Numbers
[x] Int Value
[x] Float Value

### Strings
[x] String Value
[x] Glue
[x] ControlCommand
[ ] Native Function Calls
[x] Void

### Maps
[x] Variable Pointer Value
[ ] Divert
[ ] ChoicePoint
[x] Variable Reference
[ ] Variable Assignment
[x] Tag
[x] List Value
[ ] Saved Choice

### Sequence
[ ] Container
