//! An attribute macro to stipulate bounds.
//!
//! The attribute applies bounds to `struct`s, `enum`s, `union`s, `trait`s, `fn`s, and `impl` blocks.
//!
//! ```rust
//! use attr_bounds::bounds;
//!
//! #[bounds(T: Copy)]
//! pub struct Wrapper<T>(T);
//!
//! let var = Wrapper(42);
//! ```
//!
//! ```compile_fail
//! use attr_bounds::bounds;
//!
//! #[bounds(T: Copy)]
//! pub struct Wrapper<T>(T);
//!
//! let var = Wrapper(Vec::<i32>::new());
//! //                ^^^^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `Vec<i32>`
//! ```
//!
//! # Usage notes
//!
//! Basically, the attribute is designed to be used for conditional compilation and otherwise you will not need the attribute.
//!
//! ```rust
//! use attr_bounds::bounds;
//!
//! #[cfg(feature = "unstable_feature_a")]
//! pub trait UnstableA {}
//! #[cfg(feature = "unstable_feature_b")]
//! pub trait UnstableB {}
//!
//! #[cfg_attr(feature = "unstable_feature_a", bounds(Self: UnstableA))]
//! #[cfg_attr(feature = "unstable_feature_b", bounds(Self: UnstableB))]
//! pub trait Trait {}
//!
//! #[cfg(feature = "unstable_feature_a")]
//! impl UnstableA for () {}
//! #[cfg(feature = "unstable_feature_b")]
//! impl UnstableB for () {}
//!
//! impl Trait for () {}
//! ```

use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Item, Token, WhereClause, WherePredicate};

/// Applies bounds to an item.
///
/// You can specify bounds with <i>[WhereClauseItem]</i>s.
///
/// # Examples
/// ```rust
/// use attr_bounds::bounds;
///
/// #[bounds(
///     A: Clone,
///     for<'a> &'a A: std::ops::Add<&'a A, Output = A>,
///     B: Clone,
/// )]
/// pub struct Pair<A, B>(A, B);
///
/// let pair = Pair(42, vec!['a', 'b', 'c']);
/// ```
///
/// [WhereClauseItem]: https://doc.rust-lang.org/reference/items/generics.html#where-clauses
#[proc_macro_attribute]
pub fn bounds(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
    let attr = parse_macro_input!(attr with parser);
    let mut item = parse_macro_input!(item as Item);

    let item = match try_make_where_clause(&mut item) {
        Ok(where_clause) => {
            where_clause.predicates.extend(attr);
            item.into_token_stream()
        }
        Err(_) => {
            quote! {
                compile_error!("The attribute may only be applied to `struct`s, `enum`s, `union`s, `trait`s, `fn`s, and `impl`s.");
                #item
            }
        }
    };

    item.into()
}

fn try_make_where_clause(item: &mut Item) -> Result<&mut WhereClause, &str> {
    let generics = match item {
        Item::Enum(item) => &mut item.generics,
        Item::Fn(item) => &mut item.sig.generics,
        Item::Impl(item) => &mut item.generics,
        Item::Struct(item) => &mut item.generics,
        Item::Trait(item) => &mut item.generics,
        Item::Union(item) => &mut item.generics,
        _ => return Err("unsupported item type."),
    };
    Ok(generics.make_where_clause())
}
