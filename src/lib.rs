//! Contains [`fluid`], a macro that allows you to write long method call chains as a series
//! of steps instead.
//!
//! # Basic Usage
//!
//! ```rust
//! # #[macro_use] extern crate fluid_macro;
//! # use fluid_macro::fluid;
//!
//! # fn main() {
//!
//! let x = fluid!(Some(123), {
//!     unwrap_or_default();
//!     clamp(5, 100);
//!     to_string();
//! });
//!
//! assert_eq!(x, "100");
//!
//! # }
//! ```
//!
//! This is equivalent to writing:
//!
//! ```rust
//! let x = Some(123).unwrap_or_default().clamp(5, 100).to_string();
//!
//! assert_eq!(x, "100");
//! ```
//!
//! # Nested blocks
//!
//! Methods that have a final argument that is a one-argument closure support an alternate
//! block syntax:
//!
//! ```rust
//! # #[macro_use] extern crate fluid_macro;
//! # use fluid_macro::fluid;
//!
//! struct Example(i32);
//!
//! impl Example {
//!     fn add(mut self, arg: i32) -> Self {
//!         self.0 += arg;
//!         self
//!     }
//!
//!     fn modify(mut self, f: impl FnOnce(i32) -> i32) -> Self {
//!         self.0 = f(self.0);
//!         self
//!     }
//! }
//!
//! # fn main() {
//!     let x = fluid!(Example(0), {
//!         add(15);
//!         modify() {
//!             clamp(20, 50);
//!         }
//!         add(15);
//!     });
//!
//!     assert_eq!(x.0, 35);
//! # }
//! ```
//!
//! The above is equivalent to writing:
//!
//! ```rust
//! # struct Example(i32);
//! #
//! # impl Example {
//! #     fn add(mut self, arg: i32) -> Self {
//! #         self.0 += arg;
//! #         self
//! #     }
//! #
//! #     fn modify(mut self, f: impl FnOnce(i32) -> i32) -> Self {
//! #         self.0 = f(self.0);
//! #         self
//! #     }
//! # }
//! #
//! let x = Example(0).add(15).modify(|b| b.clamp(20, 50)).add(15);
//!
//! assert_eq!(x.0, 35)
//! ```
//!
//! # Chaining in non-method things
//!
//! Although it looks a bit weird, you can write casts and operations with this syntax:
//!
//! ```rust
//! # #[macro_use] extern crate fluid_macro;
//! # use fluid_macro::fluid;
//!
//! # fn main() {
//!     let x = fluid!(5i32, {
//!         [+ 5];
//!         [as u8];
//!         to_string();
//!     });
//!
//!     assert_eq!(x, "10");
//! # }
//! ```
//!
//! The entire chained expression so far will be placed in the first position of the partial
//! expression in square brackets.
//!
//! The above expands to:
//!
//! ```rust
//! let x = ((5i32 + 5) as u8).to_string();
//!
//! assert_eq!(x, "10");
//! ```
//!
//! # Known limitations
//!
//! You can't turbofish.
//!
//! ```ignore
//! let x = fluid!("123", {
//!     parse::<i32>(); // will not compile!
//!     unwrap_or_default();
//!     clamp(5, 100);
//!     to_string();
//! })
//! ```
//!
//! It's not very friendly to the IDE whilst writing. You will have to already know the names
//! of methods you want to use. After compilation, however, symbol lookup and the like works fine.

/// General-purpose macro that allows you to write long method chains as a series of
/// statements. See the crate documentation for more details.
#[macro_export]
macro_rules! fluid {
    // Base case: There's no more calls to combine, so just resolve to the final builder.
    ($expr:expr, {}) => { $expr };
    // Nesting case: Use this macro recursively in order to handle each nested branch.
    ($expr:expr, { $block:ident($($args:expr),*) { $($children:tt)+ } $($next:tt)* }) => {
        $crate::fluid!(
            $expr.$block($($args,)* |b| $crate::fluid!(b, { $($children)+ })),
            { $($next)* }
        )
    };
    // Expression-shaped calls.
    ($expr:expr, { [$($items:tt)+]; $($next:tt)* }) => {
        $crate::fluid!(($expr $($items)+), { $($next)*} )
    };
    // Default case: Take one line and turn it into a chained call.
    ($expr:expr, { $command:ident($($args:expr),*); $($next:tt)* }) => {
        $crate::fluid!($expr.$command($($args),*), { $($next)* })
    };
}
