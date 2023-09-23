//! Procedural macros used in the definition and implementation of getters and setters for CertificationPathSettings

mod api;
mod mode;
mod ntt;
mod packing;
mod params;
mod poly;
mod polyvec;
mod reduce;
mod rounding;
mod sign;
mod symmetric;

use api::*;
use ntt::*;
use packing::*;
use params::*;
use poly::*;
use polyvec::*;
use reduce::*;
use rounding::*;
use sign::*;
use symmetric::*;

#[proc_macro]
pub fn mode_params(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_params_internal(input)
}

#[proc_macro]
pub fn mode_poly(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_poly_internal(input)
}

#[proc_macro]
pub fn mode_polyvec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_polyvec_internal(input)
}

#[proc_macro]
pub fn mode_rounding(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_rounding_internal(input)
}

#[proc_macro]
pub fn mode_sign(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_sign_internal(input)
}

#[proc_macro]
pub fn mode_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_api_internal(input)
}

#[proc_macro]
pub fn mode_ntt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_ntt_internal(input)
}

#[proc_macro]
pub fn mode_packing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_packing_internal(input)
}

#[proc_macro]
pub fn mode_reduce(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_reduce_internal(input)
}

#[proc_macro]
pub fn mode_symmetric(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    mode_symmetric_internal(input)
}
