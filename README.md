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


"Crab" is a reference to the mascot of the Rust Programming Language, which was used to write the first implementation of the dialect.

## Language features

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

### Variable initialisation

Using "let" to initialise variables is standard for Crab, like most versions of BASIC.

Although, unlike most dialects of BASIC

- all identifiers are placeholders for strings that do not start or finish with spaces
- some strings also represent integers
- there are no floating point types

```
# This string is not an integer
let name = Mary

# This string is also an integer
let num = 5
```

<img src="example.png" width="100%" height="auto" />

### Assignment operators

Crab BASIC has no concept of nested expressions. Variables are mutated by assignment.

#### Reassigned to

```
x = y
x = 5
```

#### Increased by

```
x += y
x += 5
```

#### Reduced by

```
x -= y
x -= 5
```

#### Multiplied by

```
x *= y
x *= 5
```

#### Divided by

```
x /= y
x /= 5
```

#### Modulus assignment

```
x %= y
x %= 5
```

#### Random assignment

```
x ?= y
x ?= 5
```

#### Concatenated to

```
x $= y
x $= 5
```
