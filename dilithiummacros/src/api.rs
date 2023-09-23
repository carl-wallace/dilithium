use crate::mode::Mode;
use quote::quote;

pub(crate) fn mode_api_internal(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mode = syn::parse_macro_input!(input as Mode);
    let mode_name = mode.mode_name;

    let tokens = quote! {
        pub mod api {
            use crate::#mode_name::params::{PUBLICKEYBYTES, SECRETKEYBYTES, SIGNBYTES};
            use crate::#mode_name::sign::*;

            #[derive(Copy, Clone, PartialEq, Eq, Hash)]
            pub struct Keypair {
              pub public: [u8; PUBLICKEYBYTES],
              secret: [u8; SECRETKEYBYTES],
            }

            /// Secret key elided
            impl std::fmt::Debug for Keypair {
              fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "public: {:?}\nsecret: <elided>", self.public)
              }
            }

            pub enum SignError {
              Input,
              Verify,
            }

            impl Keypair {
              /// Explicitly expose secret key
              /// ```
              /// # use pqc_dilithium::mode3::api::*;
              /// let keys = Keypair::generate();
              /// let secret_key = keys.expose_secret();
              /// assert!(secret_key.len() == pqc_dilithium::mode3::params::SECRETKEYBYTES);
              /// ```
              pub fn expose_secret(&self) -> &[u8] {
                &self.secret
              }

              /// Generates a keypair for signing and verification
              ///
              /// Example:
              /// ```
              /// # use pqc_dilithium::mode3::api::*;
              /// let keys = Keypair::generate();
              /// assert!(keys.public.len() == pqc_dilithium::mode3::params::PUBLICKEYBYTES);
              /// assert!(keys.expose_secret().len() == pqc_dilithium::mode3::params::SECRETKEYBYTES);
              /// ```
              pub fn generate() -> Keypair {
                let mut public = [0u8; PUBLICKEYBYTES];
                let mut secret = [0u8; SECRETKEYBYTES];
                crypto_sign_keypair(&mut public, &mut secret, None);
                Keypair { public, secret }
              }

              /// Generates a signature for the given message using a keypair
              ///
              /// Example:
              /// ```
              /// # use pqc_dilithium::mode3::api::*;
              /// # let keys = Keypair::generate();
              /// let msg = "Hello".as_bytes();
              /// let sig = keys.sign(&msg);
              /// assert!(sig.len() == pqc_dilithium::mode3::params::SIGNBYTES);
              /// ```
              pub fn sign(&self, msg: &[u8]) -> [u8; SIGNBYTES] {
                let mut sig = [0u8; SIGNBYTES];
                crypto_sign_signature(&mut sig, msg, &self.secret);
                sig
              }
            }

            /// Verify signature using keypair
            ///
            /// Example:
            /// ```
            /// # use pqc_dilithium::mode3::api::*;
            /// # let keys = Keypair::generate();
            /// # let msg = [0u8; 32];
            /// # let sig = keys.sign(&msg);
            /// let sig_verify = verify(&sig, &msg, &keys.public);
            /// assert!(sig_verify.is_ok());
            pub fn verify(
              sig: &[u8],
              msg: &[u8],
              public_key: &[u8],
            ) -> Result<(), SignError> {
              if sig.len() != SIGNBYTES {
                return Err(SignError::Input);
              }
              crypto_sign_verify(&sig, &msg, public_key)
            }
        }
    };
    tokens.into()
}
