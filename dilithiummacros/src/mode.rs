use syn::parse::ParseStream;
use syn::parse::{Parse, Result};
use syn::Ident;

pub(crate) struct Mode {
    pub mode_name: Ident,
}

impl Parse for Mode {
    fn parse(stream: ParseStream) -> Result<Self> {
        if stream.is_empty() {
            panic!("Mode is not specified.");
        }

        Ok(Mode {
            mode_name: stream.parse().unwrap(),
        })
    }
}
