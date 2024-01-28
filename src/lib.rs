#![allow(non_snake_case)]
/// If we have an immutable struct but we want to mutate one of the fields, then we need to use
/// a `Cell`. `Cell` provides _interior mutability_ for the value.
///
/// To demonstrate this, we will crate an **immutable** struct with the following fields:
/// - `regular`: just a regular i32
/// - `special`: an i32 (which implements Copy) wrapped in Cell
/// - `special_nocopy`: a String (which does **not** implement Copy) wrapped in Cell
/// ```
/// use std::cell::Cell;
///
/// struct Immutable {
///     regular: i32,
///     special: Cell<i32>,
///     special_nocopy: Cell<String>,
/// }
///
/// let a = Immutable {
///     regular: 1,
///     special: Cell::new(42),
///     special_nocopy: Cell::new("hi".to_string())
/// };
/// ```
///
/// Without marking `a` as `mut`, it is not possible to mutate any of the fields.
/// ```compile_fail
/// # use std::cell::Cell;
/// # let a = learning_cell::Cell::Immutable::default();
/// // Error: cannot mutate immutable variable `a`
/// a.regular += 2;
/// a.special = Cell::new(24);
/// ```
///
/// Fortunately, `Cell` gives us some options for special and special_nocopy.
/// Namely we can use methods like `get`, `set`, `swap`, `replace`, etc. on the `Cell` values.
///
/// If we look at the impls of Cell, we can see that `set` is defined for any T without any
/// restrictions.
///
/// i.e. `impl<T> Cell<T>`
///
/// This means we can use `set` for both special (i32) and special_nocopy (String).
/// ```
/// # use std::cell::Cell;
/// # let a = learning_cell::Cell::Immutable::default();
/// a.special.set(2);
/// a.special_nocopy.set("bye".to_string());
/// ```
///
/// On the other hand, `get` is defined for any T that implements the `Copy` trait.
///
/// i.e. `impl<T: Copy> Cell<T>`
///
/// Since `non-Copy` types would require us to move the value out of the `Cell` (which would leave
/// nothing inside), taking overship of the inner value without replacing it would lead to undefined
/// behaviour. For this reason, we can only use `get` for special and not for special_nocopy.
///
/// ```compile_fail
/// # use std::cell::Cell;
/// # let a = learning_cell::Cell::Immutable::default();
/// // OK
/// let _ = a.special.get();
/// // Error: trait bound `Copy` is not satisfied for String
/// let _ = a.special_nocopy.get();
/// ```
///
/// We can mitigate this issue by using `replace` or `swap` to make sure we put something back
/// inside the Cell.
/// ```
/// # use std::cell::Cell;
/// # let a = learning_cell::Cell::Immutable::default();
/// let _ = a.special_nocopy.replace("HI!".to_string());
/// ```
///
/// The only way we can directly modify the value inside the `Cell` is by using `get_mut` to get a
/// mutable reference to the inner value. However, this function comes with a caveat.
///
/// In order for compiler to ensure that we have the sole-ownership (exclusive reference) of the
/// `Cell` (and therefore, its inner value), `get_mut` function requires us to provide a mutable
/// refence to self.
///
/// i.e. `pub fn get_mut(&mut self) -> &mut T`
///
/// Unfortunately, to get a mutuable refence, we would have to mark our struct as mutable which
/// defeats the whole purpose of this exercise.
/// ```compile_fail
/// # use std::cell::Cell;
/// # let a = learning_cell::Cell::Immutable::default();
/// // Error: cannot borrow `a.special` as mutable, as `a` is not declared as mutable
/// let _ = a.special.get_mut();
/// ```
/// As also suggested by the official documentation, using `get_mut` for `Cell` generaly does not
/// make a sense. Instead, we will look into the [`RefCell`] struct.
pub mod Cell {
    use std::cell::Cell;

    #[doc(hidden)]
    pub struct Immutable {
        pub regular: i32,
        pub special: Cell<i32>,
        pub special_nocopy: Cell<String>,
    }

    impl Default for Immutable {
        fn default() -> Self {
            Self { regular: 1, special: Cell::new(42), special_nocopy: Cell::new("hi".to_string()) }
        }
    }
}
