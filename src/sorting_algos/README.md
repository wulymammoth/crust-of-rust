# sorting algorithms

## considerations

- Ord trait
  - `Trait std::cmp::Ord`
  - `Eq` : complete equality
  - `PartialOrd` : partial ordering
    - `u8` and `u16` can be compared even if they aren't of the same type

- sort in Rust is a [Timsort](https://en.wikipedia.org/wiki/Timsort) implementation
