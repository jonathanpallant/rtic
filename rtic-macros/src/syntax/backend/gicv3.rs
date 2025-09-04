use syn::{
    parse::{Parse, ParseStream},
    Result,
};

#[derive(Debug)]
pub struct BackendArgs {
    pub gicd_base_addr: syn::Ident,
    pub gicr_base_addr: syn::Ident,
}

impl Parse for BackendArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let gicd_base_addr = input.parse()?;
        let gicr_base_addr = input.parse()?;
        Ok(BackendArgs { gicd_base_addr, gicr_base_addr })
    }
}
