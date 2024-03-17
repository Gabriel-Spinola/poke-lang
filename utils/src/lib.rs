extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// LINK - https://stackoverflow.com/questions/68025264/how-to-get-all-the-variants-of-an-enum-in-a-vect-with-a-proc-macro
///
/// Get all variants of an given enum
/// # Examples
/// ```rust
/// use all_variants::AllVariants;
//
/// #[derive(AllVariants, Debug)]
/// enum Direction {
///     Left,
///     Top,
///     Right,
///     Bottom,
/// }
///
/// fn main() {
///     println!("{:?}", Direction::all_variants());
/// }
/// ```
/// Output:
///```
/// [Left, Top, Right, Bottom]
/// ```
#[proc_macro_derive(AllVariants)]
pub fn derive_all_variants(input: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(input).unwrap();

    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => enum_item.variants.into_iter().map(|v| v.ident),
        _ => panic!("AllVariants only works on enums"),
    };

    let enum_name = syn_item.ident;

    let expanded = quote! {
        impl #enum_name {
            pub fn all_variants() -> &'static[#enum_name] {
                &[ #(#enum_name::#variants), * ]
            }
        }
    };

    expanded.into()
}