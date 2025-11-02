use proc_macro2::Span;
use syn::{Generics, Ident};

use crate::extendr_options::ExtendrOptions;

/// Determine the R-side class name for a Rust `struct`/`enum`.
///
/// If `opts.r_class_name` is present, return that. Otherwise construct a
/// default name from the Rust type identifier and its generic parameter
/// names (joined with underscores) — preserving the previous behavior.
pub fn r_class_name(ident: &Ident, generics: &Generics, opts: &ExtendrOptions) -> syn::LitStr {
    if let Some(name) = opts.r_class_name.as_ref() {
        syn::LitStr::new(name, Span::call_site())
    } else {
        let mut self_ty_name = ident.to_string();
        for gen in generics.type_params() {
            self_ty_name.push('_');
            self_ty_name.push_str(&gen.ident.to_string());
        }
        syn::LitStr::new(&self_ty_name, Span::call_site())
    }
}
