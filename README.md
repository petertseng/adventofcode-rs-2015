# Advent of Code

These are my solutions to http://adventofcode.com

All solutions are written in Rust.

[![Build Status](https://travis-ci.org/petertseng/adventofcode-rs-2015.svg?branch=master)](https://travis-ci.org/petertseng/adventofcode-rs-2015)

## Input

In general, all solutions can be invoked in both of the following ways:

* Without command-line arguments, takes input on standard input.
* With 1+ command-line arguments, reads input from the first, which must be the path to an input file.
  Arguments beyond the first are ignored.

Some may additionally support other ways:

* 4 (Advent Coins): Pass the secret key in ARGV.
* 10 (Look and Say): Pass the seed sequence in ARGV.
* 11 (Passwords): Pass the initial password in ARGV.
* 20 (Factors): Pass the target number of gifts in ARGV.
* 21 (RPG): Pass the Boss's HP, damage, and armor in ARGV, in that order.
* 22 (Wizard): Pass the Boss's HP and damage in ARGV, in that order.
* 25 (Triangular): Pass the row and column number in ARGV, in that order.

## Highlights

### Regex

Most commonly-used implementation, https://github.com/rust-lang/regex, doesn't support backreferences, so that it can make performance guarantees.
As a result, regex can't be used on day 5, the day they are most useful.
As a cascading effect of this, I also choose not to use them for 8 and 11 (other days when you'd think they'd be useful)

### Itertools

Would have been useful for combinations and permutations.
Instead, they would be provided by:

* Combinations from https://github.com/bluss/rust-itertools
* Permutations from https://github.com/bluss/permutohedron

For educational purposes, I implemented these myself.

### Performance

In most cases, at least on par with the D and Haskell implementations.
Most notable exception is on day 10 because the D and Haskell implementations use the Cosmological Theorem and the Rust implementation doesn't.

Inexplicably significantly better at:

* Day 06 (Light Grid): 6x over Haskell, 2x over D
* Day 10 (Look and Say) if Haskell and D do *not* use the Cosmological Theorem: 16x over Haskell, 11x over D
* Day 15 (Cookies): 4x over Haskell, 3x over D
* Day 18 (Game of Life): 3x over Haskell, 2x over D
* Day 22 (Wizard): 3x over Haskell, 3x over D
