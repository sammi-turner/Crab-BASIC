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

While most of the classic BASICs employ upper case reserved words, this dialect uses 11 all lower case reserved words.

```
if
or
cls
let
end
goto
input
print
iprint
true
false
```

### Command line arguments

Crab BASIC has no REPL functionality and can only run scripts. Scripts can have any file extension.

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

### Clear screen

The "cls" command, on its own, clears the console for most terminal emulators. There are some notable exeptions, such as the built in IntelliJ terminal.

### Variable initialisation

Using "let" to initialise variables is standard for Crab, like most versions of BASIC. Although, unlike most dialects of BASIC

- all identifiers are placeholders for strings
- some strings also represent integers
- there are no floating point types
- input statements require prior initialisation with "let"

### Assignment operators

Crab BASIC has no bracketed expressions or nesting. Variables are mutated by assignment.

Examples of the eight assignment operators are shown below.

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

### Conditionals

Crab BASIC does not use the "then" keyword in conditional statements.

Instead, conditonal code blocks are delimited by "if" and "end if" statements, as in the following example.

```
if x == 3
print some text output
print more text output
end if
```

Examples of the eight conditional operators are shown below.

#### Identity

```
x == true
x == 7
```

#### Non-identity

```
x != true
x != 7
```

#### Less than

```
x < y
x < 7
```

#### Greater than

```
x > y
x > 7
```

#### Less than or equal to

```
x <= y
x <= 7
```

#### Greater than or equal to

```
x >= y
x >= 7
```

#### Conjunction

```
x and y
```

#### Disjunction

```
x or y
```

### Branching

Although [Edgar Dijkstra](https://en.wikipedia.org/wiki/Considered_harmful) wasn't a fan, "goto" statements are standard both for the older BASICs, and this dialect as well.

The goto command takes one argument, which should be either an integer, or an identifier representing one.

For example

```
goto 25
```

or

```
let main = 100
goto main
```

### Subroutines

Unlike most BASICS, this dialect does not have a call stack for subroutines.

Instead, the "gosub" command only tracks the last position where gosub was called.

The "return" keyword will either move program execution to the line after the most recent gosub call, or default to the last line of the program, where no gosub call was made.

### End

As with other BASICs, the "end" command in this dialect stops the execution of the program.