use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Item, Token, WhereClause, WherePredicate};

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
