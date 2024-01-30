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

/// _If you haven't read the [`Cell`] section, I recommend you do so before reading this
/// section._
///
/// `RefCell` is very similar to `Cell` in that it provides _interior mutability_ for the value.
/// The main difference is that `RefCell` allows us to get a reference to the inner value without
/// having to move it out.
///
/// This is possible because `RefCell` keeps track of the number of active references to the inner
/// value during runtime. This is done by using a `borrow` field of type i32. The value of this
/// field can be one of the following:
/// ```ignore
/// match borrow {
///     0          => no active references (UNUSED),
///     x if x > 0 => number of shared references,
///     x if x < 0 => number of exclusive references (can only be -1),
/// }
/// ```
/// This is a simple way to count references and in this case, we can get away without any race or
/// deadlock conditions. This is because `RefCell` is not marked as Sync (or Send). Without the
/// Sync marker, the compiler guarentees that `RefCell` cannot be passed to a different thread
/// which in turn guarentees that the `borrow` field can only be updated by one thread.
///
/// To demonstrate this, we will crate the same **immutable** struct as in the [`Cell`] section but
/// this time we will use `RefCell` instead of `Cell`.
/// - `regular`: just a regular i32
/// - `special`: an i32 (which implements Copy) wrapped in RefCell
/// - `special_nocopy`: a String (which does **not** implement Copy) wrapped in RefCell
/// ```
/// use std::cell::RefCell;
///
/// struct Immutable {
///    regular: i32,
///    special: RefCell<i32>,
///    special_nocopy: RefCell<String>,
/// }
///
/// let a = Immutable {
///    regular: 1,
///    special: RefCell::new(42),
///    special_nocopy: RefCell::new("hi".to_string())
/// };
///```
/// Once again, without marking `a` as `mut`, it is not possible to mutate any of the fields.
/// ```compile_fail
/// # use std::cell::RefCell;
/// # let a = learning_cell::RefCell::Immutable::default();
/// // Error: cannot mutate immutable variable `a`
/// a.regular += 2;
/// a.special = RefCell::new(24);
/// ```
/// But now we will start seeing some differences. First, we cannot use methods like `get` or `set`
/// on `RefCell`. While we still have access to `swap`, `replace` and `take`, we will see that they
/// can panic at runtime if we try to use them while the value is being borrowed.
///
/// Before we get into that, let's look at the `borrow` and `borrow_mut` methods. These methods
/// allow us to get a reference to the inner value. The difference between the two is that
/// `borrow_mut` returns a mutable reference while `borrow` returns an immutable reference.
/// ```
/// # use std::cell::RefCell;
/// # let a = learning_cell::RefCell::Immutable::default();
/// let refer = a.special.borrow();
/// assert_eq!(*refer, 42);
///
/// let mut refer = a.special_nocopy.borrow_mut();
/// *refer = "bye".to_string();
/// ```
///
/// Now what happens if we try to mutuably borrow the value while it is already borrowed? Well, we
/// will get a panic at runtime.
/// ```should_panic
/// # use std::cell::RefCell;
/// # let a = learning_cell::RefCell::Immutable::default();
/// let refer = a.special.borrow();
/// // Panic: already borrowed: BorrowMutError
/// let refer2 = a.special.borrow_mut();
/// ```
/// In fact this is the case for all the methods that try to take ownership of the inner value like
/// `swap`, `replace` and `take`. If we try to use any of these methods while the value is already
/// borrowed, we will get a panic at runtime.
///
/// And I want to emphasize that this is a **runtime panic** and not a compile time error. This
/// code will compile just fine but if you try to run it, it will panic at runtime.
/// This is because the compiler cannot know at compile time whether the value is already borrowed
/// or not. This is why we need to be careful when using `RefCell`.
///
/// Thankfully, `RefCell` provides us with a way to check whether the value is already borrowed or
/// not. This is done by using the `try_borrow` and `try_borrow_mut` methods. These methods return
/// a `Result` which is either `Ok` if the value is not borrowed or `Err` if it is.
/// ```
/// # use std::cell::RefCell;
/// # let a = learning_cell::RefCell::Immutable::default();
/// let refer = a.special.try_borrow();
/// assert!(refer.is_ok());
/// let refer2 = a.special.try_borrow_mut();
/// assert!(refer2.is_err());
/// ```
/// Funny enough, when you call `borrow` or `borrow_mut` on a `RefCell`, those methods actually
/// call `try_borrow` and `try_borrow_mut` under the hood and panic if the result is `Err`.
pub mod RefCell {
    use std::cell::RefCell;

    #[doc(hidden)]
    pub struct Immutable {
        pub regular: i32,
        pub special: RefCell<i32>,
        pub special_nocopy: RefCell<String>,
    }

    impl Default for Immutable {
        fn default() -> Self {
            Self {
                regular: 1,
                special: RefCell::new(42),
                special_nocopy: RefCell::new("hi".to_string())
            }
        }
    }
}
