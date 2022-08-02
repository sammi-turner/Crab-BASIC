# Crab BASIC

An interpreter for the Crab dialect of the BASIC programming language.

## Motivation

Although [BASIC is an anachronism](https://en.wikipedia.org/wiki/Timeline_of_programming_languages#1960s), it has a long history as a [didactic tool](https://time.com/69316/basic/).

```
Once upon a time, knowing how to use a computer was virtually synonymous with knowing how to
program one. And the thing that made it possible was a programming language called BASIC.

-Harry McCracken in Time Magazine (2014)
```

As well as being a teaching aid, BASIC interpreters are much simpler to implement than the complex runtimes required for modern scripting languages such as JavaScript, PHP or Python.


"Crab" is a reference to the mascot of the Rust Programming Language, which was used to write the first implementation of the Crab BASIC dialect.

## Language features

### No nesting

This dialect has no use for parentheses and no concept of nested expressions.

### Comments

Crab BASIC uses a single octothorpe symbol in place of the "rem" keyword that if used in most BASIC dialects.

```
# This is an example of a code comment
```

### Emoji support

Unicode emojis are supported.

```
# Welcome to 🦀 BASIC!
```

### Reserved words

While most of the classic BASICs employ upper case reserved words, this dialect uses 11 lower case reserved words.

```
if
or
cls
let
end
goto
true
false
input
print
iprint
```

### Command line arguments

This implementation of Crab BASIC has no REPL functionality and can only run scripts. 

Scripts can have any file extension.

To run the source file "example.txt", we could enter

```
cargo run example.txt
```

from the rust project root folder or

```
./crab_basic example.txt
```

to run the script any directory containing both the binary and the source folder.

### Print and Iprint

Unlike most dialects of BASIC

- string literals are not delimited by quotes
- the "print" command does not recognise identifiers
- the "iprint" command does recognise identifiers and will interpolate strings containing them
- print and iprint commands with no string literal will add a blank line

<img src="print-iprint.png" width="100%" height="auto" />

### Variables

All identifiers in the dialect are placeholders for strings that do not start or finish with spaces.

A subset of those strings also represent integers.

There are no floating point types in Crab BASIC.

All variables are initialised with a non-empty value using the "let" keyword.

```
# This string is not an integer
let name = Mary

# This string is also an integer
let num = 5
```

### Re-assignment

In this dialect, all operations are performed by re-assignment.
