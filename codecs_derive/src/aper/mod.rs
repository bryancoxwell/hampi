//! Actual Implementation for Codec `impl` generation for different ASN Types.

use super::attrs::TyCodecParams;

mod choice;
mod enumerated;
mod integer;

pub(crate) fn generate_decode(
    ast: &syn::DeriveInput,
    params: &TyCodecParams,
) -> proc_macro::TokenStream {
    let ty = params.ty.as_ref().unwrap();
    match ty.value().as_str() {
        "CHOICE" => choice::generate_aper_decode_for_asn_choice(ast, params),
        "INTEGER" => integer::generate_aper_decode_for_asn_integer(ast, params),
        "ENUMERATED" => enumerated::generate_aper_decode_for_asn_enumerated(ast, params),
        _ => syn::Error::new_spanned(ty.clone(), "This ASN.1 Type is not supported.")
            .to_compile_error()
            .into(),
    }
}