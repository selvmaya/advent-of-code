# aoc2025

## Current architecture
Every day is a crate member in a cargo workspace with two binaries (one for each part in the day).
A template generates a day with a type-based parser and solver.
We use winnow for parsing.


## Ideas for improvements

### `serde`-based serialization
Use `serde` on top of parsing, such that the parsing (serialization) can be "incremental".
Steps:
- If missing, parse the "real input" and save in optimized serialized format
- If present, load and reuse optimized serialized input

### `chumsky` for parsing
Use `chumsky` instead of winnow for a move fully declarative parser, since that is how I generally write my parsers if I can.
I feel comfortable with winnow, and think it is a very idiomatic rust parsing solution,
but find `chumsky` fascinating since it is at the forefront for parsers of what can be done in rust using the type system.
I've read the tutorial in the documentation, and looked at the API, and it makes a lot of sense,
though I think I prefer the names of certain combinators in winnow.

### `aoc-runner` as infrastructure
`cargo-aoc` and its infra crate `aoc-runner` are recommended for convenience of working specifically for AoC problems.
I find it a little lacking in clarity and scope, and kind of like doing my own infra every year,
but it is an option that can be adopted piecemeal by just using `aoc-runner`,
since it seems to do basically the same thing I am doing myself with my `Puzzle`/`Solution` types.