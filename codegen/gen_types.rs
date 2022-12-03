use crate::codegen::smithy_model::*;
use crate::codegen::utils::CodeWriter;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::path::Path;

pub struct GenTypes<'a> {
    pub model: &'a SmithyModel,
    pub writer: CodeWriter,
}

impl<'a> GenTypes<'a> {
    pub fn new(model: &'a SmithyModel, out_path: &Path) -> Self {
        Self {
            model,
            writer: CodeWriter::new(out_path),
        }
    }

    pub fn generate(mut self) {
        self.writer.write_code(quote! {
            use std::str::FromStr;
            use std::collections::HashMap;
            use std::collections::HashSet;

            trait Op {
                type Input;
                type Output;
            }
        });

        // Generate the basic enum of operation kinds + macros to quickly generate code for each operation.
        // The enum is flat - meaning it defines no attached state to any of the operations.
        // It might be interesting to consider a more complex enum if needed by the daemon,
        // or perhaps that would instead go to it's own enum, with auto-generated-mapping to this one.
        let ops_names: Vec<_> = self.model.iter_ops().map(|op| op.ident()).collect();

        self.writer.write_code(quote! {

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum S3Ops {
                #(#ops_names),*
            }

            /// This macro calls a provided $macro for each S3 operation to generate code per op.
            macro_rules! generate_ops_code {
                ($macro:ident) => {
                    #( $macro!(#ops_names); )*
                }
            }

            /// This macro calls a provided $macro for each S3 operation to generate item per op.
            macro_rules! generate_ops_items {
                ($macro:ident) => {
                    #( $macro!(#ops_names), )*
                }
            }

            /// This macro matches a variable of S3Ops type and expands the provided $macro
            /// for each S3 operation to generate code handler per op.
            macro_rules! generate_ops_match {
                ($macro:ident, $op:expr) => {
                    match ($op) {
                        #( S3Ops::#ops_names => $macro!(#ops_names), )*
                    }
                }
            }

            pub(crate) use generate_ops_code;
            pub(crate) use generate_ops_items;
            pub(crate) use generate_ops_match;
        });

        for (key, shape) in self.model.shapes.iter() {
            let ident = shape.ident();
            match shape.typ {
                SmithyType::Service => {}
                // SmithyType::Document => {}
                // SmithyType::Resource => {}
                SmithyType::Operation => {
                    let input_shape = shape
                        .members
                        .get("input")
                        .map(|m| self.model.get_shape_of(m));
                    let output_shape = shape
                        .members
                        .get("output")
                        .map(|m| self.model.get_shape_of(m));
                    let input_ident = format_ident!(
                        "{}",
                        input_shape.map_or("Destination".to_string(), |m| m.name.to_owned())
                    );
                    let output_ident = format_ident!(
                        "{}",
                        output_shape.map_or("Destination".to_string(), |m| m.name.to_owned())
                    );
                    self.writer.write_code(quote! {
                        pub struct #ident;
                        impl Op for #ident {
                            type Input = #input_ident;
                            type Output = #output_ident;
                        }
                    });
                }

                SmithyType::Structure => {
                    let members = shape.members.values().map(|k| k.ident());
                    let types = shape
                        .members
                        .values()
                        .map(|m| self.model.get_shape_of(m).ident());
                    self.writer.write_code(quote! {
                        pub struct #ident {
                            #(#members: Option<#types>,)*
                        }
                    });
                }

                SmithyType::String => {
                    if shape.has_trait(SM_ENUM) {
                        let enum_spec = shape.get_trait_value(SM_ENUM);
                        let mut i = 0;
                        let names: Vec<_> = enum_spec
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|e| {
                                i += 1;
                                format_ident!(
                                    "{}",
                                    e["name"].as_str().unwrap_or(&format!("_{}", i - 1))
                                )
                            })
                            .collect();
                        let values: Vec<_> = enum_spec
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|i| i["value"].as_str().unwrap_or(""))
                            .collect();
                        self.writer.write_code(quote! {
                            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
                            pub enum #ident {
                                #(#names,)*
                            }
                            impl AsRef<str> for #ident {
                                fn as_ref(&self) -> &str {
                                    match self {
                                        #(Self::#names => #values,)*
                                    }
                                }
                            }
                            impl TryFrom<&str> for #ident {
                                type Error = String;
                                fn try_from(s: &str) -> Result<Self, Self::Error> {
                                    match s {
                                        #(#values => Ok(Self::#names),)*
                                        _ => Err(format!("unknown enum value {}", s)),
                                    }
                                }
                            }
                        });
                    } else {
                        self.writer.write_code(quote! {
                            pub type #ident = String;
                        });
                    }
                }

                SmithyType::List => {
                    let item_type = shape
                        .members
                        .get("member")
                        .map(|m| self.model.get_shape_of(m).ident())
                        .unwrap();
                    self.writer.write_code(quote! {
                        pub type #ident = Vec<#item_type>;
                    });
                }

                SmithyType::Set => {
                    let item_type = shape
                        .members
                        .get("member")
                        .map(|m| self.model.get_shape_of(m).ident())
                        .unwrap();
                    self.writer.write_code(quote! {
                        pub type #ident = HashSet<#item_type>;
                    });
                }

                SmithyType::Map => {
                    let key_type = shape
                        .members
                        .get("key")
                        .map(|m| self.model.get_shape_of(m).ident())
                        .unwrap();
                    let value_type = shape
                        .members
                        .get("key")
                        .map(|m| self.model.get_shape_of(m).ident())
                        .unwrap();
                    self.writer.write_code(quote! {
                        pub type #ident = HashMap<#key_type, #value_type>;
                    });
                }

                _ => {
                    let type_name = to_rust_type(shape);
                    self.writer.write_code(quote! {
                        pub type #ident = #type_name;
                    });
                }
            }
        }

        self.writer.done();
    }
}

fn to_rust_type(shape: &SmithyShape) -> TokenStream {
    match shape.typ {
        SmithyType::Boolean => quote! { bool },
        SmithyType::Byte => quote! { i8 },
        SmithyType::Short => quote! { i16 },
        SmithyType::Integer => quote! { i32 },
        SmithyType::Long => quote! { i64 },
        SmithyType::Float => quote! { f32 },
        SmithyType::Double => quote! { f64 },
        SmithyType::String => quote! { String },
        SmithyType::Timestamp => quote! { String },
        SmithyType::Blob => quote! { Vec<u8> },
        SmithyType::Union => quote! { Vec<String> },
        _ => todo!(
            "unsupported input shape type {:?} shape name {}",
            shape.typ,
            shape.name,
        ),
    }
}
