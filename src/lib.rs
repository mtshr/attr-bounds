//! An attribute macro to stipulate bounds.
//!
//! The attribute applies bounds to `struct`s, `enum`s, `union`s, `trait`s, `fn`s, associated `type`s, and `impl` blocks.
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

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse},
    parse_macro_input,
    punctuated::Punctuated,
    ItemEnum, ItemFn, ItemImpl, ItemStruct, ItemTrait, ItemType, ItemUnion, Signature, Token,
    TraitItemFn, TraitItemType, WhereClause, WherePredicate,
};

enum Item {
    Enum(ItemEnum),
    Fn(ItemFn),
    Impl(ItemImpl),
    Struct(ItemStruct),
    Trait(ItemTrait),
    Type(ItemType),
    Union(ItemUnion),
    AssocType(TraitItemType),
    FnDecl(TraitItemFn),
}

impl Item {
    fn make_where_clause(&mut self) -> &mut WhereClause {
        let generics = match self {
            Item::Enum(ItemEnum { generics, .. })
            | Item::Fn(ItemFn {
                sig: Signature { generics, .. },
                ..
            })
            | Item::Impl(ItemImpl { generics, .. })
            | Item::Struct(ItemStruct { generics, .. })
            | Item::Trait(ItemTrait { generics, .. })
            | Item::Type(ItemType { generics, .. })
            | Item::Union(ItemUnion { generics, .. })
            | Item::AssocType(TraitItemType { generics, .. })
            | Item::FnDecl(TraitItemFn {
                sig: Signature { generics, .. },
                ..
            }) => generics,
        };
        generics.make_where_clause()
    }
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if let Ok(item) = fork
            .parse::<syn::Item>()
            .map_or(Err(()), |item| match item {
                syn::Item::Enum(item) => Ok(Item::Enum(item)),
                syn::Item::Fn(item) => Ok(Item::Fn(item)),
                syn::Item::Impl(item) => Ok(Item::Impl(item)),
                syn::Item::Struct(item) => Ok(Item::Struct(item)),
                syn::Item::Trait(item) => Ok(Item::Trait(item)),
                syn::Item::Type(item) => Ok(Item::Type(item)),
                syn::Item::Union(item) => Ok(Item::Union(item)),
                _ => Err(()),
            })
        {
            input.advance_to(&fork);
            return Ok(item);
        }

        if let Ok(item) = input
            .parse::<syn::TraitItem>()
            .map_or(Err(()), |item| match item {
                syn::TraitItem::Fn(item) => Ok(Item::FnDecl(item)),
                syn::TraitItem::Type(item) => Ok(Item::AssocType(item)),
                _ => Err(()),
            })
        {
            return Ok(item);
        }

        Err(input.error("Unexpected item."))
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Item::Enum(item) => item.to_tokens(tokens),
            Item::Fn(item) => item.to_tokens(tokens),
            Item::Impl(item) => item.to_tokens(tokens),
            Item::Struct(item) => item.to_tokens(tokens),
            Item::Trait(item) => item.to_tokens(tokens),
            Item::Type(item) => item.to_tokens(tokens),
            Item::Union(item) => item.to_tokens(tokens),
            Item::AssocType(item) => item.to_tokens(tokens),
            Item::FnDecl(item) => item.to_tokens(tokens),
        }
    }
}

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
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
    let attr = parse_macro_input!(attr with parser);

    match syn::parse::<Item>(input) {
        Ok(mut item) => {
            let where_clause = item.make_where_clause();
            where_clause.predicates.extend(attr);
            item.into_token_stream().into()
        }
        Err(_) => {
            // Using the compile_error!() macro to highlight the attribute in reporting an error.
            quote! {
                compile_error!("The attribute may only be applied to `struct`s, `enum`s, `union`s, `trait`s, `fn`s, `type`s, and `impl` blocks.");
            }.into()
        }
    }
}
