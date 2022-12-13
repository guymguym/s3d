use crate::codegen::smithy_model::*;
use crate::codegen::utils::CodeWriter;
use quote::quote;
use std::path::Path;

pub struct GenOps<'a> {
    pub model: &'a SmithyModel,
    pub writer: CodeWriter,
}

impl<'a> GenOps<'a> {
    pub fn new(model: &'a SmithyModel, out_path: &Path) -> Self {
        Self {
            model,
            writer: CodeWriter::new(out_path),
        }
    }

    pub fn generate(mut self) {
        self.write_header();
        self.write_ops_enum_and_macros();
        self.writer.done();
    }

    fn write_header(&mut self) {
        self.writer.write_code(quote! {
            #![allow(unused)]
            #![allow(non_camel_case_types)]
        });
    }

    fn write_ops_enum_and_macros(&mut self) {
        // Generate the basic enum of operation kinds + macros to quickly generate code for each operation.
        // The enum is flat - meaning it defines no attached state to any of the operations.
        // It might be interesting to consider a more complex enum if needed by the daemon,
        // or perhaps that would instead go to it's own enum, with auto-generated-mapping to this one.
        let ops_names: Vec<_> = self.model.iter_ops().map(|op| op.ident()).collect();

        self.writer.write_code(quote! {

            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            pub enum Ops {
                #(#ops_names, )*
            }

            /// This macro calls a provided $macro for each operation to generate code per op.
            macro_rules! generate_ops_code {
                ($macro:ident) => {
                    #( $macro!(#ops_names); )*
                }
            }

            /// This macro calls a provided $macro for each operation to generate item per op.
            macro_rules! generate_ops_items {
                ($macro:ident) => {
                    #( $macro!(#ops_names), )*
                }
            }

            /// This macro matches a variable of Ops type and expands the provided $macro
            /// for each operation to generate code handler per op.
            macro_rules! generate_ops_match {
                ($macro:ident, $op:expr) => {
                    match ($op) {
                        #( ops::Ops::#ops_names => $macro!(#ops_names), )*
                    }
                }
            }

            pub(crate) use generate_ops_code;
            pub(crate) use generate_ops_items;
            pub(crate) use generate_ops_match;
        });
    }
}
