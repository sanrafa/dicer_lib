This is a small library intended for use with my Dicer applications.

It's not being 'maintained' in the usual sense, but feel free to fork and use it as a basis for your own work.

## API

Input should be a string written in [dice notation](https://en.wikipedia.org/wiki/Dice_notation) or arithmetic. Supports exploding dice - `2d6!`.

### `roll (input: &str) -> Result<i32>`

Returns summed output based on dice roll/expression.

Supports addition, subtration, multiplication, and division. Expressions can use parentheses.

### `pool (input: &str, base: i32, threshold: f64) -> Result<Vec<(i32, bool)>>`

Returns a vector of numbers based on dice roll/expression.

- `base` represents the type of dice to use when not specified.

  - E.g. `pool("2+2", 10)` is equivalent to `4d10`.

- `threshold` determines the minimum required for a successful dice roll.
  - If less than 1, minimum equals `threshold * die_faces + 1` (rounded down)
  - If greater than 1, `minimum = threshold`

Only supports addition and subtraction. Subtraction truncates the pool by number of dice specificed - type is irrelevant. If more dice are subtracted than exists in the pool, returns `[(0, false)]` (automatic failure).

NOTE: Pools combining dice and integers cannot begin with an integer. `2+1d10` is invalid, but `1d10+2` is allowed.

## License

MIT

## Crates used:

- [pest](https://crates.io/crates/pest)
- [anyhow](https://crates.io/crates/anyhow)
- [rand](https://crates.io/crates/rand)
- [lazy_static](https://crates.io/crates/lazy_static)
