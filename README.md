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

"Crab" is a reference to the mascot of the Rust Programming Language, which was used to write the first implementation of this BASIC.

### No nested expressions!

Crab places strict limits on the number of tokens on a single line and does not allow nested expressions.

The motivations for this austere design are as follows.

- Crab is designed as a teaching tool rather than a production language.
- Shorter lines are faster to parse for the simplest variety of interpreter.
- Shoter lines are easier to read for humans who may be unfamiliar with programming.
- A simpler language also allows for an implementation that is more approachable for curious programmers.

### Comments

Crab uses a single octothorpe symbol in place of the "rem" keyword used in most BASICs.

```
# This is an example of a code comment
```

### Emoji support

Unicode emojis are supported.

```
# Welcome to 🦀 BASIC!
```

### Reserved words

Crab BASIC uses 13 reserved words, which are all lower case.

```
if
or
cls
let
end
goto
gosub
input
print
iprint
return
true
false
```

### Command line arguments

Crab has no REPL functionality and can only run scripts. Scripts can have any file extension.

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

Unlike most BASICs, in Crab

- string literals are not delimited by quotes
- the "print" command does not recognise identifiers
- the "iprint" command does recognise identifiers and will interpolate strings containing them
- print and iprint commands with no string literal will add a blank line

### Cls

The cls command, on its own, clears the console for most terminal emulators. 

However, there are some notable exeptions, such as the built-in IntelliJ terminal.

### Identifiers

In Crab, all identifiers must begin with a lower case letter.

"Let" commands are used to initialise variables, which is standard for most versions of BASIC.

Although, unlike other BASICs, in Crab

- all identifiers are placeholders for strings
- some strings also represent integers
- there are no floating point types
- input statements require prior initialisation with "let"

### Assignment operators

Crab has no bracketed expressions or nesting. Variables are mutated by assignment.

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

### Goto

Although [Edgar Dijkstra](https://en.wikipedia.org/wiki/Considered_harmful) wasn't a fan, goto statements are standard for most of the older BASIC implementations.

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

Gosub and return work in a similar fashion to most BASICs, where each gosub statements pushes the current line onto a call stack before program execution jumps to the line indicated.

When return is called, the last line is popped off the stack and execution returns to that line.

For example, this program

```
gosub 4
print second line
end

print first line
return
```

has this output

```
first line
second line

On line 3, the program ends.
```
### Loops

Although Crab has no specific keywords for looping, it is still simple to implement looping structures.

For example, the following code

```
let n = 0

if n < 10
iprint n
n += 1
goto 2
end if
```

has the following output

```
0
1
2
3
4
5
6
7
8
9

On line 7, the program ends.
```

### End

As with other BASICs, the "end" command on its own stops the execution of the program.
