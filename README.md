[![Crates.io](https://img.shields.io/crates/v/pluck.svg)](https://crates.io/crates/pluck)
[![docs](https://docs.rs/mio/badge.svg)](https://docs.rs/pluck)

# pluck

`pluck!` is a macros that creates a lambda that plucks the provided
property from the argument. Great with iterators.

## Access

`pluck!` provides many different ways to access values.

### Tuple Indices

Provide the index to extract.

```rust
let list = [(0, "a"), (1, "b"), (2, "c")];

let first = list.iter().map(pluck!(.0)).collect::<Vec<_>>();
assert_eq!(first, &[0, 1, 2]);

let second = list.iter().map(pluck!(.1)).collect::<Vec<_>>();
assert_eq!(second, &["a", "b", "c"]);
```

### Struct Properties

Provide the property name to extract.

```rust
struct Person { name: &'static str }
let list = [Person { name: "Alice" }];

let names = list.iter().map(pluck!(.name)).collect::<Vec<_>>();

assert_eq!(names, &["Alice"]);
```

### By Reference

Precede the property name with `&` to pluck by reference.

```rust
let list = [(0, "a"), (1, "b"), (2, "c")];

let first = list.iter().map(pluck!(&.0)).collect::<Vec<_>>();
assert_eq!(first, &[&0, &1, &2]);
```

### Mutable Reference

Precede the property name with `&mut` to pluck by mutable reference.

```rust
let mut list = [(0, "a"), (1, "b"), (2, "c")];

for num in list.iter_mut().map(pluck!(&mut .0)) {
    *num += 1;
}

assert_eq!(list, [(1, "a"), (2, "b"), (3, "c")]);
```

### Index Type

`pluck!` works with types implementing [`Index`](std::ops::Index) and
[`IndexMut`](std::ops::IndexMut).

```rust
let list = [[0], [1], [2]];

let first = list.iter().map(pluck!([0])).collect::<Vec<_>>();
assert_eq!(first, &[0, 1, 2]);

let first_ref = list.iter().map(pluck!(&[0])).collect::<Vec<_>>();
assert_eq!(first_ref, &[&0, &1, &2]);
```

### Deref

`pluck!` works with types implementing [`Deref`](std::ops::Deref) and
[`DerefMut`](std::ops::DerefMut).

```rust
let list = vec![&0, &1, &2];
let derefed = list.into_iter().map(pluck!(*)).collect::<Vec<_>>();
assert_eq!(derefed, &[0, 1, 2]);

let list = vec![&&&0, &&&1, &&&2];
let derefed = list.into_iter().map(pluck!(***)).collect::<Vec<_>>();
assert_eq!(derefed, &[0, 1, 2]);
```

## Combinations

`pluck!` is designed to allow you to arbitrarily combine accessing. You
can specify precedence with `()`.

```rust
struct Person { name: &'static str }
let mut list = vec![[&Person { name: "Alice" }]];
let derefed = list.iter_mut().map(pluck!((*[0]).name)).collect::<Vec<_>>();
assert_eq!(derefed, &["Alice"]);
```

# License: MIT
