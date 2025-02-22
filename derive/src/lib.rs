use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

const SHUFFLE_ORDER: [&[u8]; 7] = [
    &[0, 1, 2],
    &[0, 1, 2, 3],
    &[0, 1, 2, 3, 4],
    &[0, 1, 2, 3, 4, 5],
    &[0, 1, 2, 3, 4, 5, 6],
    &[0, 1, 2, 3, 4, 5, 6, 7],
    &[0, 1, 2, 3, 4, 5, 6, 7, 8]
];

#[proc_macro_attribute]
pub fn vm_shuffle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Extract fields from the struct
    let fields = if let syn::Fields::Named(fields) = &input.fields {
        &fields.named
    } else {
        panic!("Expected named fields");
    };

    let field_count = fields.iter().count();
    if field_count < 3 || field_count > 9 {
        panic!("unexpected field count")
    }

    // Select the appropriate shuffle order
    let shuffle_order = SHUFFLE_ORDER[field_count - 3];

    // Shuffle the fields according to the shuffle order
    let shuffled_fields: Vec<_> = shuffle_order.iter()
        .map(|&i| fields.iter().nth(i as usize).unwrap())
        .collect();

    // Generate the new struct with shuffled fields
    let struct_name = &input.ident;
    let vis = &input.vis;
    let shuffled_fields_tokens = shuffled_fields.iter();

    let expanded = quote! {
        #[repr(C)]
        #vis struct #struct_name {
            #(#shuffled_fields_tokens),*
        }
    };

    TokenStream::from(expanded)
}