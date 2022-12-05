use crate::codegen::smithy_model::*;
use crate::codegen::utils::CodeWriter;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub struct GenHttp<'a> {
    pub model: &'a SmithyModel,
    pub writer: CodeWriter,
}

impl<'a> GenHttp<'a> {
    pub fn new(model: &'a SmithyModel, out_path: &Path) -> Self {
        Self {
            model,
            writer: CodeWriter::new(out_path),
        }
    }

    pub fn generate(mut self) {
        self.writer.write_code(quote! {
            #![allow(unused)]
            #![allow(non_camel_case_types)]
            use std::str::FromStr;
        });
        self.writer.done();
    }
}
