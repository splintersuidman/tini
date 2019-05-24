# tini [![Build Status](https://travis-ci.org/splintah/tini.svg?branch=master)](https://travis-ci.org/splintah/tini)

tini — **T**here **I**s **N**o **I**nfix — is a tiny functional programming language.

The only two data types supported are integers and functions.

## Getting started

This crate contains two binaries: `repl` and `tinii`.
`tini-repl` contains a ‘run - eval - print - loop’ interface that evaluates single lines of
tini.
`tinii` runs the tini code inside the file supplied at the command line.

After the code has been installed (see [Installation]), run `tini-repl` or `tinii <file>` to
evaluate tini code.

## Installation

Prerequisites:

- Nightly [Rust] 1.30 or newer with `cargo`

```bash
cargo install --git https://gitlab.com/splintah/tini
```

## Syntax

Integers are written just like in most major programming languages: `123`.
Function calls are written like this: `(function argument argument argument ...)`.
There are two ‘special’ functions: `if` and `define`:

- `if` expressions are written like so: `(if condition consequence alternative)`.
  If the condition evaluates to `0`, the consequence is evaluated and returned.
  In all other cases, the alternative is evaluated and returned.
- `define` expressions can be written in two ways:
  1. to define a value, write `(define name value)`; `name` will be bound to the value of
  `value`.
  2. to define a function, write `(define (name argument argument argument ...) value)`; when
  `name` is called, the `value` will be evaluated, with the `argument`s in scope.

Lastly, there are comments: every line that begins with ‘;’ is considered to be a comment, and
its value will be discarded. (The comments are not present in the BNF below, because they are
not part of the token tree.)

The syntax can be written in (this non-standard dialect – ‘invented’ by me – of) [BNF]:

```plain
<digit>   ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
<integer> ::= <digit>+

<alphabetic>        ::= 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I'
                      | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R'
                      | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'a'
                      | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j'
                      | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's'
                      | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z'
<special character> ::= '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '-'
                      | '=' | '+' | '|' | ':' | '/' | '?' | ',' | '.' | '<'
                      | '>' | '`' | '~' | '_' | '\''
<identifier begin>  ::= <alphabetic>
                      | <special character>
<identifier middle> ::= <identifier begin>
                      | <digit>
<identifier>        ::= <identifier begin><identifier middle>*

<define value>    ::= '(' 'define' <identifier> <expression> ')'
<define function> ::= '(' 'define' '(' <identifier> <identifier>* ')' <expression> ')'
<define>          ::= <define value>
                    | <define function>

<if> ::= '(' 'if' <expression> <expression> <expression> ')'

<call> ::= '(' <identifier> <expression>* ')'

<expression> ::= <integer>
               | <identifier>
               | <define>
               | <if>
               | <call>
```

## Built-in functions

The current built-in functions are:

- `(+ x y)` adds the integers `x` and `y`.
- `(- x y)` subtracts the integer `y` from the integer `x`.
- `(* x y)` multiplies the integers `x` and `y`.
- `(= x y)` compares two values: integers are equal when `(= (- x y) 0)`, functions are never equal.
- `(> x y)` returns whether integer `x` is greater than integer `y`; if a non-integer is given, the result is _always_ `0`.
- `(> x y)` returns whether integer `x` is less than integer `y`; if a non-integer is given, the result is _always_ `0`.
- `(print argument argument argument ...)` prints its arguments; functions cannot be printed well, so they are printed as ‘`<function>`’.

## Examples

### Factorial

This example calculates n!, the factorial of n.

```plain
; (fac n) calculates the factorial of n.
(define (fac n)
  (if (= n 1)
    1
    (* n (fac (- n 1)))))
(print (fac 6))
```

Output:

```plain
720
```

### More examples

See the directory `examples/` for more examples.

## Inspiration

tini's syntax is mostly inspired by [LISP].

## Licence

This repository is licensed under the MIT licence; see the file `LICENCE`.

[Installation]: #installation
[Rust]: https://rust-lang.org
[BNF]: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
[LISP]: https://en.wikipedia.org/wiki/Lisp_(programming_language)
