//! `pluck!` is a macro that creates a lambda that plucks the provided
//! property from the argument. Great with iterators.
//! 
//! # Access
//! 
//! `pluck!` provides many different ways to access values.
//!
//! ## Tuple Indices
//!
//! Provide the index to extract.
//!
//! ```
//! # use pluck::*;
//! let list = [(0, "a"), (1, "b"), (2, "c")];
//!
//! let first = list.iter().map(pluck!(.0)).collect::<Vec<_>>();
//! assert_eq!(first, &[0, 1, 2]);
//!
//! let second = list.iter().map(pluck!(.1)).collect::<Vec<_>>();
//! assert_eq!(second, &["a", "b", "c"]);
//! ```
//!
//! ## Struct Properties
//!
//! Provide the property name to extract.
//!
//! ```
//! # use pluck::*;
//! struct Person { name: &'static str }
//! let list = [Person { name: "Alice" }];
//!
//! let names = list.iter().map(pluck!(.name)).collect::<Vec<_>>();
//!
//! assert_eq!(names, &["Alice"]);
//! ```
//!
//! ## By Reference
//!
//! Precede the property name with `&` to pluck by reference.
//!
//! ```
//! # use pluck::*;
//! let list = [(0, "a"), (1, "b"), (2, "c")];
//!
//! let first = list.iter().map(pluck!(&.0)).collect::<Vec<_>>();
//! assert_eq!(first, &[&0, &1, &2]);
//! ```
//!
//! ## Mutable Reference
//!
//! Precede the property name with `&mut` to pluck by mutable reference.
//!
//! ```
//! # use pluck::*;
//! let mut list = [(0, "a"), (1, "b"), (2, "c")];
//!
//! for num in list.iter_mut().map(pluck!(&mut .0)) {
//!     *num += 1;
//! }
//!
//! assert_eq!(list, [(1, "a"), (2, "b"), (3, "c")]);
//! ```
//!
//! ## Index Type
//!
//! `pluck!` works with types implementing [`Index`](std::ops::Index) and
//! [`IndexMut`](std::ops::IndexMut).
//!
//! ```
//! # use pluck::*;
//! let list = [[0], [1], [2]];
//!
//! let first = list.iter().map(pluck!([0])).collect::<Vec<_>>();
//! assert_eq!(first, &[0, 1, 2]);
//!
//! let first_ref = list.iter().map(pluck!(&[0])).collect::<Vec<_>>();
//! assert_eq!(first_ref, &[&0, &1, &2]);
//! ```
//!
//! ## Deref
//!
//! `pluck!` works with types implementing [`Deref`](std::ops::Deref) and
//! [`DerefMut`](std::ops::DerefMut).
//! 
//! ```
//! # use pluck::*;
//! let list = vec![&0, &1, &2];
//! let derefed = list.into_iter().map(pluck!(*)).collect::<Vec<_>>();
//! assert_eq!(derefed, &[0, 1, 2]);
//!
//! let list = vec![&&&0, &&&1, &&&2];
//! let derefed = list.into_iter().map(pluck!(***)).collect::<Vec<_>>();
//! assert_eq!(derefed, &[0, 1, 2]);
//! ```
//! 
//! # Combinations
//! 
//! `pluck!` is designed to allow you to arbitrarily combine accessing. You
//! can specify precedence with `()`.
//! 
//! ```
//! # use pluck::*;
//! struct Person { name: &'static str }
//! let mut list = vec![[&Person { name: "Alice" }]];
//! let derefed = list.iter_mut().map(pluck!((*[0]).name)).collect::<Vec<_>>();
//! assert_eq!(derefed, &["Alice"]);
//! ```

#[doc(hidden)]
#[macro_export]
macro_rules! do_expression {
    ($var:expr, ($($exprs:tt)*)$($tail:tt)*) => {
        $crate::do_expression!($crate::do_expression!($var, $($exprs)*), $($tail)*)
    };
    ($var:expr, [$expr:tt]$($tail:tt)*) => {
        $crate::do_expression!($var[$expr], $($tail)*)
    };
    ($var:expr, *$($tail:tt)*) => {
        *$crate::do_expression!($var, $($tail)*)
    };
    ($var:expr, .$expr:tt$($tail:tt)*) => {
        $crate::do_expression!($var.$expr, $($tail)*)
    };
    ($var:expr,) => {
        $var
    }
}

/// Create a lambda that extracts the provided property from the argument.
///
/// See [crate level documentation](crate) for detailed usage.
#[macro_export]
macro_rules! pluck {
    (&mut $($expr:tt)+) => {
        |value| &mut $crate::do_expression!(value, $( $expr )+)
    };
    (&$($expr:tt)+) => {
        |value| & $crate::do_expression!(value, $( $expr )+)
    };
    ($($expr:tt)+) => {
        |value| $crate::do_expression!(value, $( $expr )+)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ref_to_triple_deref() {
        let list = vec![&&&0, &&&1, &&&2];
        let derefed = list.into_iter().map(pluck!(&***)).collect::<Vec<_>>();
        assert_eq!(derefed, &[&0, &1, &2]);
    }

    #[test]
    fn mut_ref_deref() {
        let mut list = vec![0, 1, 2];
        let derefed = list.iter_mut().map(pluck!(&mut *)).collect::<Vec<_>>();
        assert_eq!(derefed, &[&0, &1, &2]);
    }

    #[test]
    fn deref_index() {
        let mut list = vec![[&0], [&1], [&2]];
        let derefed = list.iter_mut().map(pluck!(*[0])).collect::<Vec<_>>();
        assert_eq!(derefed, &[0, 1, 2]);
    }

    #[test]
    fn nested_index() {
        let mut list = vec![[[0]], [[1]], [[2]]];
        let derefed = list.iter_mut().map(pluck!([0][0])).collect::<Vec<_>>();
        assert_eq!(derefed, &[0, 1, 2]);
    }

    #[test]
    fn nested_property() {
        struct Person { address: Address }
        struct Address { street: &'static str }

        let mut list = [Person { address: Address { street: "foo" } }];
        let streets = list.iter_mut().map(pluck!(.address.street)).collect::<Vec<_>>();
        assert_eq!(streets, &["foo"]);
    }

    #[test]
    fn index_deref() {
        let mut list = vec![[&[0]], [&[1]], [&[2]]];
        let derefed = list.iter_mut().map(pluck!((*[0])[0])).collect::<Vec<_>>();
        assert_eq!(derefed, &[0, 1, 2]);
    }

    #[test]
    fn index_deref_property() {
        struct Person { name: &'static str }
        let mut list = vec![[&Person { name: "Alice" }]];
        let derefed = list.iter_mut().map(pluck!((*[0]).name)).collect::<Vec<_>>();
        assert_eq!(derefed, &["Alice"]);
    }
}
