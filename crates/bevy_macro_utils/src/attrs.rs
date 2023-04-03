use syn::DeriveInput;

use crate::symbol::Symbol;

pub fn parse_attrs(ast: &DeriveInput, attr_name: Symbol) -> syn::Result<Vec<&syn::MetaList>> {
    let mut list = Vec::new();
    for attr in ast.attrs.iter().filter(|a| a.path().is_ident(&attr_name.to_string())) {
        let meta_res = attr.meta.require_list();
        match meta_res {
            Ok(meta_list) => list.push(meta_list),
            _ => continue,
        }
    }
    Ok(list)
}

pub fn get_lit_str(attr_name: Symbol, lit: &syn::Lit) -> syn::Result<&syn::LitStr> {
    if let syn::Lit::Str(lit) = lit {
        Ok(lit)
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!("expected {attr_name} attribute to be a string: `{attr_name} = \"...\"`"),
        ))
    }
}

pub fn get_lit_bool(attr_name: Symbol, lit: &syn::Lit) -> syn::Result<bool> {
    if let syn::Lit::Bool(lit) = lit {
        Ok(lit.value())
    } else {
        Err(syn::Error::new_spanned(
            lit,
            format!("expected {attr_name} attribute to be a bool value, `true` or `false`: `{attr_name} = ...`"),
        ))
    }
}
