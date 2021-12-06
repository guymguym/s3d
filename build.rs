//! # build.rs
//!
//! This build script runs automatically by cargo
//! and generates `$OUT_DIR/s3.rs` based on `models/s3.json` smithy JSON AST
//!
//! See:
//! - https://awslabs.github.io/smithy/1.0/spec/index.html
//! - https://awslabs.github.io/smithy/1.0/spec/core/json-ast.html
//!

use quote::{format_ident, quote};
use serde_json::{Map, Value};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
};

const MODELS_DIR: &str = "models";

// smithy.api traits used in s3.json:
const _SM_PREFIX: &str = "smithy.api#";
const _SM_DOC: &str = "smithy.api#documentation";
const SM_ENUM: &str = "smithy.api#enum";
const _SM_ERROR: &str = "smithy.api#error";
const _SM_REQUIRED: &str = "smithy.api#required";
const _SM_HTTP: &str = "smithy.api#http";
const SM_HTTP_LABEL: &str = "smithy.api#httpLabel";
const SM_HTTP_QUERY: &str = "smithy.api#httpQuery";
const SM_HTTP_HEADER: &str = "smithy.api#httpHeader";
const SM_HTTP_PAYLOAD: &str = "smithy.api#httpPayload";
const SM_HTTP_PREFIX_HEADERS: &str = "smithy.api#httpPrefixHeaders";
const _SM_HTTP_CHECKSUM_REQUIRED: &str = "smithy.api#httpChecksumRequired";
const _SM_XML_NS: &str = "smithy.api#xmlNamespace";
const SM_XML_NAME: &str = "smithy.api#xmlName";
const _SM_XML_ATTR: &str = "smithy.api#xmlAttribute";
const _SM_XML_FLATTENED: &str = "smithy.api#xmlFlattened";
const _SM_SENSITIVE: &str = "smithy.api#sensitive";
const _SM_TIMESTAMP_FORMAT: &str = "smithy.api#timestampFormat";
const _SM_EVENT_PAYLOAD: &str = "smithy.api#eventPayload";
const _SM_STREAMING: &str = "smithy.api#streaming";
const _SM_PAGINATED: &str = "smithy.api#paginated";
const _SM_DEPRECATED: &str = "smithy.api#deprecated";
const _SM_TITLE: &str = "smithy.api#title";
const _SM_PATTERN: &str = "smithy.api#pattern";
const _SM_LENGTH: &str = "smithy.api#length";
const _SM_HOST_LABEL: &str = "smithy.api#hostLabel";
const _SM_ENDPOINT: &str = "smithy.api#endpoint";
const _SM_AUTH: &str = "smithy.api#auth";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let models_path = Path::new(MODELS_DIR);
    let s3_out_path = out_path.join("s3.rs");
    let s3_model_path = models_path.join("s3.json");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", s3_model_path.display());
    let s3_model = SmithyModel::new(&s3_model_path);
    SmithyGen::new(&s3_out_path).generate(&s3_model);
}

pub struct SmithyGen {
    pub writer: CodeWriter,
    // map of xml model names to whether they were already generated
    pub xml_models: HashMap<String, bool>,
}

impl SmithyGen {
    pub fn new(out_path: &Path) -> Self {
        Self {
            writer: CodeWriter::new(out_path),
            xml_models: HashMap::new(),
        }
    }

    pub fn generate(&mut self, model: &SmithyModel) {
        self.write_code(quote! {

            /// This file is generated by build.rs. DO NOT EDIT.

            use crate::{http::*, xml::*};
            use aws_smithy_http::byte_stream::ByteStream;
            use aws_smithy_xml::decode::{Document, ScopedDecoder};
            use hyper::{body::Bytes, header::HeaderValue, Body, StatusCode};

        });

        self.gen_ops_enum(model);
        for (ref name, ref op) in model.get_shapes_by_type("operation") {
            self.gen_server_op(model, name, op);
        }
        for (ref name, ref shape) in model.get_shapes_by_type("structure") {
            if self.xml_models.contains_key(name) {
                self.gen_xml_model(model, name, shape);
            }
        }
        for (ref name, _) in model.get_shapes_with_trait(SM_ENUM) {
            self.gen_enum_type(model, name);
        }
        self.writeln("// build.rs done.");
        self.writer.flush().unwrap();
    }

    ///
    /// Generates a simple enum of operation kinds and macros to generate code for each operation.
    ///
    /// The enum has no attached state to any of the operations.
    /// It might be interesting to consider a more complex enum if needed by the daemon,
    /// or perhaps that would instead go to it's own enum, with auto-generated-mapping to this one.
    ///
    fn gen_ops_enum(&mut self, model: &SmithyModel) {
        let ops_iter = || {
            model
                .get_shapes_by_type("operation")
                .map(|(name, _)| format_ident!("{}", name))
        };
        let ops1 = ops_iter();
        let ops2 = ops_iter();
        let ops3 = ops_iter();
        let ops4 = ops_iter();

        self.write_code(quote! {

            #[derive(Debug, PartialEq, Eq, Clone, Copy)]
            pub enum S3Ops {
                #(#ops1),*
            }

            /// This macro calls a provided $macro for each S3 operation to generate code per op.
            macro_rules! generate_code_for_each_s3_op {
                ($macro:ident) => {
                    #( $macro!(#ops2); )*
                }
            }

            /// This macro matches a variable of S3Ops type and expands the provided $macro
            /// for each S3 operation to generate code handler per op.
            macro_rules! generate_match_for_each_s3_op {
                ($macro:ident, $op:expr) => {
                    match ($op) {
                        #( S3Ops::#ops3 => $macro!(#ops4), )*
                    }
                }
            }

            pub(crate) use generate_code_for_each_s3_op;
            pub(crate) use generate_match_for_each_s3_op;

        });
    }

    ///
    /// Generates an impl of ServerOperation per operation.
    ///
    fn gen_server_op(&mut self, model: &SmithyModel, name: &str, op: &Value) {
        let (_, input) = model.get_target_shape(&op["input"]);
        let (_, output) = model.get_target_shape(&op["output"]);
        assert_eq!(SmithyModel::get_shape_type(&op), "operation");
        if !input.is_null() {
            assert_eq!(SmithyModel::get_shape_type(&input), "structure");
        }
        if !output.is_null() {
            assert_eq!(SmithyModel::get_shape_type(&output), "structure");
        }
        let name_ident = format_ident!("{}", name);
        let name_input = format_ident!("{}Input", name);
        let name_output = format_ident!("{}Output", name);
        let name_error = format_ident!("{}Error", name);
        let field_decoders = model
            .get_members(&input)
            .map(|(ref field_name, ref field)| self.decode_input_field(model, field_name, field))
            .collect::<Vec<_>>();
        let field_encoders = model
            .get_members(&output)
            .map(|(ref field_name, ref field)| self.encode_output_field(model, field_name, field))
            .collect::<Vec<_>>();

        self.write_code(quote! {

            pub struct #name_ident {}
            impl ServerOperation for #name_ident {
                type Input = aws_sdk_s3::input::#name_input;
                type Output = aws_sdk_s3::output::#name_output;
                type Error = aws_sdk_s3::error::#name_error;
                const OP: S3Ops = S3Ops::#name_ident;

                fn decode_input(req: &mut S3Request) -> TraitFuture<Self::Input, S3Error> {
                    Box::pin(async move {
                        let mut b = Self::Input::builder();
                        #( #field_decoders )*
                        Ok(b.build()?)
                    })
                }

                fn encode_output(o: Self::Output) -> TraitFuture<'static, HttpResponse, S3Error> {
                    Box::pin(async move {
                        let mut r = responder();
                        let mut body = Body::empty();
                        let h = r.headers_mut().unwrap();
                        #( #field_encoders )*
                        Ok(r.body(body)?)
                    })
                }
            }

        });
    }

    fn decode_input_field(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> proc_macro2::TokenStream {
        let field_snake = field_name.snake();
        let field_id = format_ident!("{}", field_snake);
        let set_field = format_ident!("set_{}", field_snake);
        let get_field = format_ident!("get_{}", field_snake);
        if SmithyModel::has_trait(field, SM_HTTP_LABEL) {
            quote! { b = b.#field_id(req.#get_field()); }
        } else if SmithyModel::has_trait(field, SM_HTTP_QUERY) {
            let param = SmithyModel::get_trait_str(field, SM_HTTP_QUERY);
            quote! { b = b.#set_field(req.get_param(#param)); }
        } else if SmithyModel::has_trait(field, SM_HTTP_HEADER) {
            let header = SmithyModel::get_trait_str(field, SM_HTTP_HEADER);
            quote! { b = b.#set_field(req.get_header(#header)); }
        } else if SmithyModel::has_trait(field, SM_HTTP_PREFIX_HEADERS) {
            let header_map = SmithyModel::get_trait_str(field, SM_HTTP_PREFIX_HEADERS);
            quote! { b = b.#set_field(req.get_header_map(#header_map)); }
        } else if SmithyModel::has_trait(field, SM_HTTP_PAYLOAD) {
            let (shape_name, shape) = model.get_target_shape(field);
            if SmithyModel::get_shape_type(&shape) == "blob" {
                quote! { b = b.#set_field(Some(ByteStream::new(req.take_body().into()))); }
            } else if SmithyModel::has_trait(field, SM_XML_NAME) {
                if !self.xml_models.contains_key(&shape_name) {
                    self.xml_models.insert(shape_name.to_owned(), false);
                }
                let xml_ident = format_ident!("{}", shape_name);
                let xml_name = SmithyModel::get_trait_str(field, SM_XML_NAME);
                quote! {
                    b = b.#set_field(Some(#xml_ident::decode_xml(
                        req.take_body_bytes().await?, #xml_name)?));
                }
            } else {
                quote! {
                    // TODO decode_input JSON ???
                    b = b.#set_field(None);
                }
            }
        } else {
            quote! {
                // TODO decode_input XML ???
                b = b.#set_field(None);
            }
        }
    }

    fn encode_output_field(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> proc_macro2::TokenStream {
        let field_snake = field_name.snake();
        let field_id = format_ident!("{}", field_snake);
        if SmithyModel::has_trait(field, SM_HTTP_HEADER) {
            let header = SmithyModel::get_trait_str(field, SM_HTTP_HEADER);
            quote! { o.#field_id().set_header(h, #header); }
        } else if SmithyModel::has_trait(field, SM_HTTP_PAYLOAD) {
            let (_shape_name, shape) = model.get_target_shape(field);
            if SmithyModel::get_shape_type(&shape) == "blob" {
                quote! { body = Body::wrap_stream(o.body); }
            } else {
                quote! {
                    // TODO encode_output XML PAYLOAD #field_id
                }
            }
        } else if SmithyModel::has_trait(field, SM_XML_NAME) {
            quote! {
                // TODO encode_output XML !!! #field_id
            }
        } else {
            quote! {
                // TODO encode_output XML ??? #field_id
            }
        }
    }

    fn gen_xml_model(&mut self, model: &SmithyModel, name: &str, shape: &Value) {
        let xml_ident = format_ident!("{}", name);
        let error_str = format!("Bad {}", name);
        let field_decoders = model
            .get_members(&shape)
            .map(|(ref field_name, ref field)| self.decode_xml_field(model, field_name, field))
            .collect::<Vec<_>>();

        self.write_code(quote! {

            pub struct #xml_ident {}
            impl #xml_ident {
                pub fn decode_xml(bytes: Bytes, xml_name: &str)
                    -> Result<aws_sdk_s3::model::#xml_ident, S3Error> {
                    let mut b = aws_sdk_s3::model::#xml_ident::builder();
                    let mut doc = Document::try_from(&bytes[..]).unwrap();
                    let mut d = doc.root_element().unwrap();
                    if !d.start_el().matches(xml_name) {
                        Err(S3Error::bad_request(#error_str))?;
                    }
                    while let Some(mut d) = d.next_tag() {
                        match d.start_el() {
                            #( #field_decoders )*
                             _ => Err(S3Error::bad_request(#error_str))?,
                        }
                    }
                    Ok(b.build())
                }
            }

        });
    }

    fn decode_xml_field(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> proc_macro2::TokenStream {
        let (shape_name, shape) = model.get_target_shape(field);
        let field_snake = field_name.snake();
        let field_id = format_ident!("{}", field_snake);
        let set_field = format_ident!("set_{}", field_snake);

        match SmithyModel::get_shape_type(&shape).as_str() {
            "list" => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(None); // TODO decode_xml LIST
                },
            },
            "structure" => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(None); // TODO decode_xml STRUCTURE
                },
            },
            "union" => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(None); // TODO decode_xml UNION
                },
            },
            "timestamp" => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(xml_to_date(&mut d));
                },
            },
            _ => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(xml_to_data(&mut d));
                },
            },
        }
    }

    fn gen_enum_type(&mut self, _model: &SmithyModel, name: &str) {
        let enum_ident = format_ident!("{}", name);
        self.write_code(quote! {
            impl FromHttp for aws_sdk_s3::model::#enum_ident {
                fn from_http(v: &str) -> Option<Self> {
                    Some(Self::from(v))
                }
            }
            impl ToHeader for &aws_sdk_s3::model::#enum_ident {
                fn to_header(self) -> Option<HeaderValue> {
                    self.as_str().to_header()
                }
            }
        });
    }

    fn write_code<T: ToString>(&mut self, code: T) {
        self.writer.write_all(code.to_string().as_bytes()).unwrap();
        self.writer.write_all(b"\n\n").unwrap();
    }

    fn writeln<T: AsRef<[u8]>>(&mut self, s: T) {
        self.writer.write_all(s.as_ref()).unwrap();
        self.writer.write_all(b"\n").unwrap();
    }
}

pub struct SmithyModel {
    pub json: Value,
    empty: Map<String, Value>,
}

impl SmithyModel {
    pub fn new(model_path: &Path) -> Self {
        let json: Value = serde_json::from_reader(File::open(model_path).unwrap()).unwrap();
        SmithyModel {
            json,
            empty: Map::new(),
        }
    }

    pub fn get_shapes_by_type<'a>(
        &'a self,
        t: &'a str,
    ) -> impl Iterator<Item = (String, Value)> + 'a {
        self.json["shapes"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_k, v)| v["type"].as_str() == Some(t))
            .map(|(k, v)| (k.unprefix().uncaps(), v.to_owned()))
    }

    pub fn get_shapes_with_trait<'a>(
        &'a self,
        t: &'a str,
    ) -> impl Iterator<Item = (String, Value)> + 'a {
        self.json["shapes"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_k, v)| SmithyModel::has_trait(&v, t))
            .map(|(k, v)| (k.unprefix().uncaps(), v.to_owned()))
    }

    pub fn get_target_shape(&self, p: &Value) -> (String, Value) {
        match &p["target"] {
            Value::String(s) => (s.unprefix().uncaps(), self.json["shapes"][s].to_owned()),
            _ => ("".to_string(), Value::Null),
        }
    }

    pub fn get_members<'a>(&'a self, p: &'a Value) -> impl Iterator<Item = (String, Value)> + 'a {
        p["members"]
            .as_object()
            .unwrap_or(&self.empty)
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_owned()))
    }

    pub fn get_shape_type(p: &Value) -> String {
        match &p["type"] {
            Value::String(s) => s.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn has_trait(p: &Value, t: &str) -> bool {
        p.pointer(&format!("/traits/{}", t)).is_some()
    }

    pub fn get_trait(p: &Value, t: &str) -> Value {
        p.pointer(&format!("/traits/{}", t))
            .map_or(Value::Null, |v| v.to_owned())
    }

    pub fn get_trait_str(p: &Value, t: &str) -> String {
        p.pointer(&format!("/traits/{}", t))
            .map_or("".to_string(), |v| v.as_str().unwrap().to_string())
    }
}

///
/// CodeWriter
///
pub struct CodeWriter {
    path: PathBuf,
    rustfmt: Child,
    w: Option<BufWriter<ChildStdin>>,
}
impl CodeWriter {
    fn new(file_path: &Path) -> Self {
        let file = File::create(file_path).unwrap();
        println!("CodeWriter file {:?}", file);
        let mut rustfmt = Command::new("rustfmt")
            .arg("--edition=2021")
            .stdin(Stdio::piped())
            .stdout(file)
            .spawn()
            .unwrap();
        println!("CodeWriter rustfmt {:?}", rustfmt);
        let w = Some(BufWriter::new(rustfmt.stdin.take().unwrap()));
        CodeWriter {
            path: file_path.to_path_buf(),
            rustfmt,
            w,
        }
    }
}
impl Write for CodeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.as_mut().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        println!("CodeWriter flush buffers {}", self.path.display());
        self.w.take().unwrap().flush()?;
        println!("CodeWriter wait rustfmt {}", self.path.display());
        self.rustfmt.wait()?;
        println!("CodeWriter done {}", self.path.display());
        Ok(())
    }
}

///
/// Unprefix returns just the suffix for `prefix#suffix` strings
///
trait Unprefix {
    fn unprefix(&self) -> String;
}
impl Unprefix for String {
    fn unprefix(&self) -> String {
        self.as_str().unprefix()
    }
}
impl Unprefix for &str {
    fn unprefix(&self) -> String {
        self.split_once('#').unwrap().1.to_string()
    }
}

///
/// Uncaps changes upper case words to camel case
///
trait Uncaps {
    fn uncaps(&self) -> String;
}
impl Uncaps for String {
    fn uncaps(&self) -> String {
        self.as_str().uncaps()
    }
}
impl Uncaps for &str {
    fn uncaps(&self) -> String {
        let mut out = String::new();
        let mut upper_streak = 0;
        for c in self.chars() {
            if c.is_uppercase() || c.is_numeric() {
                if upper_streak == 0 {
                    out.push(c);
                } else {
                    out.push(c.to_lowercase().next().unwrap());
                }
                upper_streak += 1;
            } else {
                if upper_streak > 1 && out.len() > 1 {
                    let c = out.pop().unwrap();
                    out.push(c.to_uppercase().next().unwrap());
                }
                out.push(c);
                upper_streak = 0;
            }
        }
        out
    }
}

///
/// Snake case from camel case
///
trait Snake {
    fn snake(&self) -> String;
}
impl Snake for String {
    fn snake(&self) -> String {
        self.as_str().snake()
    }
}
impl Snake for &str {
    fn snake(&self) -> String {
        let mut out = String::new();
        let mut upper_streak = 0;
        for mut c in self.chars() {
            if c.is_uppercase() || c.is_numeric() {
                if upper_streak == 0 && out.len() > 0 && out.chars().last().unwrap() != '_' {
                    out.push('_');
                }
                out.push(c.to_lowercase().next().unwrap());
                upper_streak += 1;
            } else {
                if !c.is_alphanumeric() {
                    c = '_';
                }
                if upper_streak > 1 && out.len() > 1 && c != '_' {
                    let c = out.pop().unwrap();
                    out.push('_');
                    out.push(c);
                }
                out.push(c);
                upper_streak = 0;
            }
        }
        out
    }
}

// match type_name {
//     "service" => {}
//     "operation" => {}
//     "structure" => {
//         gen_decode_fn_def(w, &sname, &format!("aws_sdk_s3::model::{}", name), None);
//     }
//     "union" => {
//         gen_decode_fn_def(w, &sname, "String", None);
//     }
//     "list" | "set" => {
//         gen_decode_fn_def(w, &sname, "Vec<String>", None);
//     }
//     "map" => {
//         gen_decode_fn_def(w, &sname, "HashMap<String,String>", None);
//     }
//     "string" => {
//         gen_decode_fn_def(w, &sname, "String", None);
//     }
//     "boolean" => {
//         gen_decode_fn_def(w, &sname, "bool", None);
//     }
//     "timestamp" => {
//         gen_decode_fn_def(w, &sname, "aws_smithy_types::date_time::DateTime", None);
//     }
//     "document" => {
//         gen_decode_fn_def(w, &sname, "String", None);
//     }
//     "blob" => {
//         gen_decode_fn_def(w, &sname, "Vec<u8>", None);
//     }
//     "integer" => {
//         gen_decode_fn_def(w, &sname, "i32", None);
//     }
//     "byte" => {
//         gen_decode_fn_def(w, &sname, "u8", None);
//     }
//     "short" => {
//         gen_decode_fn_def(w, &sname, "i16", None);
//     }
//     "long" => {
//         gen_decode_fn_def(w, &sname, "i64", None);
//     }
//     "float" => {
//         gen_decode_fn_def(w, &sname, "f32", None);
//     }
//     "double" => {
//         gen_decode_fn_def(w, &sname, "f64", None);
//     }
//     "bigInteger" => {
//         gen_decode_fn_def(w, &sname, "i64", None);
//     }
//     "bigDecimal" => {
//         gen_decode_fn_def(w, &sname, "f64", None);
//     }
//     _ => {
//         panic!("Unsupported type {}", type_name);
//     }
// }

// let srv = model["shapes"]
// .as_object()
// .unwrap()
// .iter()
// .find(|(_k, v)| v["type"].as_str() == Some("service"))
// .unwrap()
// .1
// .clone();

// let operations: Vec<Op> = srv["operations"]
// .as_array()
// .unwrap()
// .iter()
// .map(|it| {
//     let op = get_target_shape(&model, it);
//     assert_eq!(op.shape_type, ShapeType::Operation);
//     let input = get_target_shape(&model, &op.json["input"]);
//     let output = get_target_shape(&model, &op.json["output"]);
//     let inputs = get_shape_fields(&model, &input);
//     let outputs = get_shape_fields(&model, &output);
//     let is_input_body_stream = input.traits.get(_SM_STREAMING).is_some();
//     let is_output_body_stream = output.traits.get(_SM_STREAMING).is_some();
//     Op {
//         name: op.name,
//         shape: op,
//         inputs,
//         outputs,
//         is_input_body_stream,
//         is_output_body_stream,
//     }
// })
// .collect();

// let enum_types: Obj = model["shapes"]
// .as_object()
// .unwrap()
// .iter()
// .filter(|(_k, v)| v["traits"][SM_ENUM].is_array())
// .map(|(k, v)| (k.unprefix().uncaps(), v.clone()))
// .collect();

// let xml_models: Obj = operations
// .iter()
// .map(|(name, op)| {
//     let (input_name, input_shape) = target_pair(&model, &op["input"]);
//     if input_shape["type"].as_str().unwrap() != "structure" {
//         return Value::Null;
//     }
//     println!("xml_models op name {} input_name {}", name, input_name);
//     for (field_name, member_shape) in
//         input_shape["members"].as_object().unwrap().iter()
//     {
//         let xml_name = member_shape.pointer(&format!("/traits/{}", SM_XML_NAME));
//         if xml_name.is_null() {
//             continue;
//         }
//     }
// })
// .map(|(k, v)| (k.unprefix().uncaps(), v.clone()))
// .collect();

// #[derive(Debug, PartialEq, Eq, Clone, Copy)]
// enum ShapeType {
//     Service,
//     Operation,
//     Structure,
//     Union,
//     List,
//     Set,
//     Map,
//     String,
//     Boolean,
//     Timestamp,
//     Document,
//     Blob,
//     Integer,
//     Byte,
//     Short,
//     Long,
//     Float,
//     Double,
//     BigInteger,
//     BigDecimal,
// }

// impl From<&str> for ShapeType {
//     fn from(s: &str) -> Self {
//         match s {
//             "service" => ShapeType::Service,
//             "operation" => ShapeType::Operation,
//             "structure" => ShapeType::Structure,
//             "union" => ShapeType::Union,
//             "list" => ShapeType::List,
//             "set" => ShapeType::Set,
//             "map" => ShapeType::Map,
//             "string" => ShapeType::String,
//             "boolean" => ShapeType::Boolean,
//             "timestamp" => ShapeType::Timestamp,
//             "document" => ShapeType::Document,
//             "blob" => ShapeType::Blob,
//             "integer" => ShapeType::Integer,
//             "byte" => ShapeType::Byte,
//             "short" => ShapeType::Short,
//             "long" => ShapeType::Long,
//             "float" => ShapeType::Float,
//             "double" => ShapeType::Double,
//             "bigInteger" => ShapeType::BigInteger,
//             "bigDecimal" => ShapeType::BigDecimal,
//             _ => panic!("Unknown shape type: {}", s),
//         }
//     }
// }

// struct Shape {
//     name: String,
//     full_name: String,
//     shape_type: ShapeType,
//     traits: Traits,
//     json: Value,
// }

// struct Field {
//     name: String,
//     shape: Shape,
//     traits: Traits,
// }

// struct Op {
//     name: String,
//     shape: Shape,
//     inputs: Vec<Field>,
//     outputs: Vec<Field>,
//     is_input_body_stream: bool,
//     is_output_body_stream: bool,
// }

// trait JQ {
//     fn jq(&self, keys: &str) -> Value;
// }

// impl JQ for Value {
//     fn jq(&self, q: &str) -> Value {
//         match self {
//             Value::Null => Value::Null,
//             Value::Bool(_) => {
//                 if q.is_empty() {
//                     self.clone()
//                 } else {
//                     Value::Null
//                 }
//             }
//             Value::Number(_) => {
//                 if q.is_empty() {
//                     self.clone()
//                 } else {
//                     Value::Null
//                 }
//             }
//             Value::String(_) => {
//                 if q.is_empty() {
//                     self.clone()
//                 } else {
//                     Value::Null
//                 }
//             }
//             Value::Array(a) => a.iter().map(|it| it.jq(q)).collect(),
//             Value::Object(o) => {
//                 let qs: Vec<&str> = q.splitn(2, '.').collect();
//                 o[qs[0]].jq(qs[1])
//             }
//         }
//     }
// }

// trait SM {
//     fn get_type(&self) -> ShapeType;
//     fn get_target(&self, model: &Value) -> Obj;
//     fn get_trait(&self, key: &str) -> Value;
//     fn get_members(&self) -> Obj;
// }

// impl SM for Value {
//     fn get_type(&self) -> ShapeType {
//         self["type"].as_str().unwrap().into()
//     }

//     fn get_target(&self, model: &Value) -> Obj {
//         match self["target"].as_str() {
//             Some(s) => model["shapes"][s].as_object().unwrap().clone(),
//             None => self["target"].as_object().unwrap().clone(),
//         }
//     }

//     fn get_trait(&self, key: &str) -> Value {
//         self["traits"][key].clone()
//     }

//     fn get_members(&self) -> Obj {
//         self["members"].as_object().unwrap().clone()
//     }
// }

// writeln!(w, "pub struct {} {{}}", name).unwrap();
// writeln!(w, "impl ServerOperation for {} {{", name).unwrap();
// writeln!(w, "    type Input = aws_sdk_s3::input::{}Input;", name).unwrap();
// writeln!(w, "    type Output = aws_sdk_s3::output::{}Output;", name).unwrap();
// writeln!(w, "    type Error = aws_sdk_s3::error::{}Error;", name).unwrap();
// writeln!(w, "    const OP: S3Ops = S3Ops::{};", name).unwrap();

// writeln!(w, "    fn decode_input(req: &mut S3Request)").unwrap();
// writeln!(w, "        -> TraitFuture<Self::Input, S3Error> {{").unwrap();
// writeln!(w, "        Box::pin(async move {{").unwrap();
// writeln!(w, "            let mut b = Self::Input::builder();").unwrap();
// for (field_name, field) in self.get_members(&input).iter() {
//     is_input_body_stream =
//         self.decode_input_field(w, field_name, field) || is_input_body_stream;
// }
// writeln!(w, "           Ok(b.build()?)").unwrap();
// writeln!(w, "       }})").unwrap();
// writeln!(w, "    }}").unwrap();

// writeln!(w, "    fn encode_output(o: Self::Output)").unwrap();
// writeln!(
//     w,
//     "        -> TraitFuture<'static, HttpResponse, S3Error> {{"
// )
// .unwrap();
// writeln!(w, "        Box::pin(async move {{").unwrap();
// writeln!(w, "            let mut r = responder();").unwrap();
// writeln!(w, "            let h = r.headers_mut().unwrap();").unwrap();
// for (field_name, field) in self.get_members(&output).iter() {
//     is_output_body_stream =
//         self.decode_output_field(w, field_name, field) || is_output_body_stream;
// }
// if is_output_body_stream {
//     writeln!(w, "            Ok(r.body(Body::wrap_stream(o.body))?)").unwrap();
// } else {
//     writeln!(w, "            Ok(r.body(Body::empty())?)").unwrap();
// }
// writeln!(w, "        }})").unwrap();
// writeln!(w, "    }}").unwrap();

// writeln!(
//     w,
//     "    const IS_INPUT_BODY_STREAMING: bool = {};",
//     is_input_body_stream
// )
// .unwrap();
// writeln!(
//     w,
//     "    const IS_OUTPUT_BODY_STREAMING: bool = {};",
//     is_output_body_stream
// )
// .unwrap();
// writeln!(w, "}}").unwrap();

// pub struct {name} {{}}
// impl {name} {{
//     pub fn decode_xml(bytes: Bytes, xml_name: &str)
//         -> Result<aws_sdk_s3::model::{name}, S3Error> {{
//         let mut b = aws_sdk_s3::model::{name}::builder();
//         let mut doc = Document::try_from(&bytes[..]).unwrap();
//         let mut d = doc.root_element().unwrap();
//         if !d.start_el().matches(xml_name) {{
//             Err(S3Error::bad_request(\"Bad {name}\"))?;
//         }}
//         while let Some(mut d) = d.next_tag() {{
//             match d.start_el() {{",
//                 name = name
//             )
//             .unwrap();

//             let indent = "                ";
//             for (field_name, field) in SmithyModel::get_members(&shape).iter() {
//                 let (shape_name, shape) = model.get_target_shape(field);
//                 let field_snake = field_name.snake();
//                 writeln!(w, "{}el if el.matches(\"{}\") => {{", indent, field_name,).unwrap();
//                 match SmithyModel::get_shape_type(&shape).as_str() {
//                     "list" => writeln!(
//                         w,
//                         "{}    b = b.set_{}(None); // TODO decode_xml LIST",
//                         indent, field_snake,
//                     )
//                     .unwrap(),
//                     "structure" => writeln!(
//                         w,
//                         "{}    b = b.set_{}(None); // TODO decode_xml STRUCTURE",
//                         indent, field_snake,
//                     )
//                     .unwrap(),
//                     _ => writeln!(
//                         w,
//                         "{}    b = b.set_{}(xml_to_data(&mut d));",
//                         indent, field_snake,
//                     )
//                     .unwrap(),
//                 }
//                 writeln!(w, "{}}}", indent).unwrap();
//             }

//             writeln!(
//                 w,
//                 "\
//                 {}_ => Err(S3Error::bad_request(\"Bad {}\"))?,
//             }}
//         }}
//         Ok(b.build())
//     }}
// }}",
//                 indent, name,
//             )
//             .unwrap();
//         }
