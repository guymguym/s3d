use crate::codegen::smithy_model::*;
use crate::codegen::utils::CodeWriter;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
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
        self.write_header();
        for (_key, shape) in self.model.shapes.iter() {
            self.write_shape_type(shape);
        }
        self.writer.done();
    }

    fn write_header(&mut self) {
        self.writer.write_code(quote! {

            #![allow(unused)]
            #![allow(non_camel_case_types)]
            use crate::common::OpService;
            use std::str::FromStr;
            use std::collections::HashMap;
            use std::collections::HashSet;
            use std::sync::Arc;
            use aws_smithy_http_server::operation::{
                Operation, OperationShape, OperationService, IntoService, Handler};
            use aws_smithy_http_server::response::IntoResponse;
            use aws_smithy_http_server::proto::rest_xml::RestXml;
            use aws_smithy_http_server::body::BoxBody;

        });
    }

    fn write_shape_type(&mut self, shape: &SmithyShape) {
        let ident = shape.ident();
        match shape.typ {
            SmithyType::Service => {
                let members = shape.members.values().map(|m| m.ident());
                let ops = shape
                    .members
                    .values()
                    .map(|m| self.model.get_shape_of(m).ident());
                self.writer.write_code(quote! {

                    #[derive(Debug, Default, Clone)]
                    pub struct #ident {
                        #(pub #members: Option<OpService<#ops>>, )*
                    }

                });
            }

            SmithyType::Operation => {
                let input = self.get_member_type(shape, "input");
                let output = self.get_member_type(shape, "output");
                let name = &shape.name;
                self.writer.write_code(quote! {

                    #[derive(Debug, Default, Clone)]
                    pub struct #ident;

                    impl OperationShape for #ident {
                        const NAME: &'static str = #name;
                        type Input = #input;
                        type Output = #output;
                        type Error = ();
                    }

                    // impl IntoResponse<RestXml> for <#ident as OperationShape>::Output {
                    //     fn into_response(self) -> hyper::Response<BoxBody> {
                    //         hyper::Response::new(BoxBody::default())
                    //     }
                    // }

                });
            }

            SmithyType::Structure => {
                let members = shape.members.values().map(|m| m.ident());
                let types = shape
                    .members
                    .values()
                    .map(|m| self.model.get_shape_of(m).ident());
                self.writer.write_code(quote! {

                    #[derive(Debug, Default, Clone)]
                    pub struct #ident {
                        #(pub #members: Option<#types>,)*
                    }

                    impl #ident {
                        pub fn to_http_response(self) -> anyhow::Result<hyper::Response<hyper::Body>> {
                            anyhow::bail!("todo")
                        }
                    }

                });
            }

            SmithyType::List => {
                let item_type = self.get_member_type(shape, "member");
                self.writer.write_code(quote! {
                    pub type #ident = Vec<#item_type>;
                });
            }
            SmithyType::Set => {
                let item_type = self.get_member_type(shape, "member");
                self.writer.write_code(quote! {
                    pub type #ident = HashSet<#item_type>;
                });
            }
            SmithyType::Map => {
                let key_type = self.get_member_type(shape, "key");
                let value_type = self.get_member_type(shape, "value");
                self.writer.write_code(quote! {
                    pub type #ident = HashMap<#key_type, #value_type>;
                });
            }

            SmithyType::Union => {
                self.write_shape_type_union(shape);
            }

            SmithyType::Enum => {
                self.write_shape_type_enum(shape);
            }

            SmithyType::Blob => {
                let type_name = if shape.has_trait(SM_STREAMING) {
                    quote! { Arc<hyper::Body> }
                } else {
                    quote! { Vec<u8> }
                };
                self.writer.write_code(quote! {
                    pub type #ident = #type_name;
                });
            }

            // TODO handle SmithyType::Timestamp
            SmithyType::Timestamp => {
                self.writer.write_code(quote! {
                    pub type #ident = String;
                });
            }

            SmithyType::String => {
                if shape.has_trait(SM_ENUM_TRAIT) {
                    self.write_shape_string_enum(shape);
                } else {
                    self.write_shape_type_primitive(shape);
                }
            }

            // SmithyType::Document => {}
            // SmithyType::Resource => {}
            _ => {
                self.write_shape_type_primitive(shape);
            }
        }
    }

    fn write_shape_type_primitive(&mut self, shape: &SmithyShape) {
        let ident = shape.ident();
        let type_name = self.get_primitive_type(shape);
        self.writer.write_code(quote! {
            pub type #ident = #type_name;
        });
    }

    fn write_shape_type_union(&mut self, shape: &SmithyShape) {
        let ident = shape.ident();
        let names = shape.members.keys().map(|k| format_ident!("{}", k));
        let values = shape
            .members
            .values()
            .map(|m| self.model.get_shape_of(m).ident());
        self.writer.write_code(quote! {
            #[derive(Debug, Clone)]
            pub enum #ident {
                #(#names(#values),)*
            }
        });
    }

    fn write_shape_type_enum(&mut self, shape: &SmithyShape) {
        let ident = shape.ident();
        let names: Vec<_> = shape
            .members
            .keys()
            .map(|k| format_ident!("{}", k))
            .collect();
        let values: Vec<_> = shape
            .members
            .values()
            .map(|m| m.get_trait(SM_ENUM_VALUE))
            .collect();
        self._write_enum(ident, names, values);
    }

    fn write_shape_string_enum(&mut self, shape: &SmithyShape) {
        let mut i = 0;
        let enum_spec = shape.get_trait_value(SM_ENUM_TRAIT);
        let ident = shape.ident();
        let names: Vec<_> = enum_spec
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                i += 1;
                format_ident!("{}", e["name"].as_str().unwrap_or(&format!("_{}", i - 1)))
            })
            .collect();
        let values: Vec<_> = enum_spec
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i["value"].as_str().unwrap_or(""))
            .collect();
        self._write_enum(ident, names, values);
    }

    fn _write_enum(&mut self, ident: Ident, names: Vec<Ident>, values: Vec<&str>) {
        self.writer.write_code(quote! {
            #[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
    }

    fn get_member_type(&mut self, shape: &SmithyShape, member_name: &str) -> TokenStream {
        if let Some(m) = shape.members.get(member_name) {
            if m.target != SM_UNIT_TYPE {
                return self.model.get_shape_of(m).ident().to_token_stream();
            }
        }
        quote! { () }
    }

    fn get_primitive_type(&mut self, shape: &SmithyShape) -> TokenStream {
        match shape.typ {
            SmithyType::Boolean => quote! { bool },
            SmithyType::Byte => quote! { i8 },
            SmithyType::Short => quote! { i16 },
            SmithyType::Integer => quote! { i32 },
            SmithyType::Long => quote! { i64 },
            SmithyType::Float => quote! { f32 },
            SmithyType::Double => quote! { f64 },
            SmithyType::String => quote! { String },
            _ => panic!("non primitive shape {} type {:?}", shape.name, shape.typ,),
        }
    }
}
