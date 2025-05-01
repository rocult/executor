use crate::type_to_vm;

use quote::ToTokens;

fn handle_struct(ident: syn::Ident, fields: &mut syn::FieldsNamed) {
    fields.named.iter_mut().for_each(|x| {
        if let Some(ty) = type_to_vm(
            &format!("{}{}", ident, x.ident.as_ref().unwrap()),
            &x.ty.clone().into_token_stream().to_string(),
        ) {
            x.ty = ty;
        }
    });
}

pub fn do_encryptions(syntax_tree: &mut syn::File) {
    syntax_tree.items.iter_mut().for_each(|x| match x {
        syn::Item::Struct(x) => match &mut x.fields {
            syn::Fields::Named(fields) => handle_struct(x.ident.clone(), fields),
            _ => {}
        },
        _ => {}
    });
}
