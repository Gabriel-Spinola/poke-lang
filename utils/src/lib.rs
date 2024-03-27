extern crate proc_macro;
use proc_macro::TokenStream;

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

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ConvertToTokenRule)]
pub fn convert_to_token_rule(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let ast = parse_macro_input!(input as DeriveInput);

    // Extract the name of the enum
    let enum_name = &ast.ident;

    // Generate conversion code for each variant
    let conversion_code = match &ast.data {
        syn::Data::Enum(data_enum) => {
            let mut tokens = Vec::new();
            for variant in &data_enum.variants {
                let variant_ident = &variant.ident;
                let conversion = match &variant.fields {
                    syn::Fields::Unit => {
                        quote! { Self::#variant_ident => Some(TokenRule::#variant_ident), }
                    }
                    syn::Fields::Named(_) => {
                        quote! { Self::#variant_ident { .. } => Some(TokenRule::#variant_ident), }
                    }
                    syn::Fields::Unnamed(_) => {
                        quote! { Self::#variant_ident(..) => Some(TokenRule::#variant_ident), }
                    }
                };
                tokens.push(conversion);
            }
            quote! { impl #enum_name {
                pub fn to_rule(&self) -> Option<TokenRule> {
                    match self {
                        #(#tokens)*
                        _ => None,
                    }
                }
            }}
        }
        _ => {
            quote! { compile_error!("ConvertToTokenRule can only be derived for enums"); }
        }
    };

    // Return the generated implementation
    conversion_code.into()
}
