//! # build.rs
//!
//! This build script runs automatically by cargo
//! and generates `$OUT_DIR/s3.rs` based on `models/s3.json` smithy JSON AST
//!
//! See:
//! - https://awslabs.github.io/smithy/1.0/spec/index.html
//! - https://awslabs.github.io/smithy/1.0/spec/core/json-ast.html
//!

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde_json::{Map, Value};
use std::{
    collections::HashSet,
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
    let out_path = Path::new(out_dir.as_str());
    let model_path = Path::new(MODELS_DIR).join("s3.json");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", model_path.display());
    let model = SmithyModel::new(model_path.as_path());
    SmithyGen::new(&out_path).generate(&model);
}

pub struct SmithyGen {
    pub out_path: PathBuf,
    pub writer: Option<CodeWriter>,
    // map of xml model names to whether they were already generated
    pub xml_models_queue: Vec<String>,
    pub xml_models_done: HashSet<String>,
}

impl SmithyGen {
    pub fn new(out_path: &Path) -> Self {
        Self {
            out_path: out_path.to_path_buf(),
            writer: None,
            xml_models_queue: Vec::new(),
            xml_models_done: HashSet::new(),
        }
    }

    pub fn close_out_file(&mut self) {
        if self.writer.is_some() {
            self.writer.take().unwrap().flush().unwrap();
        }
    }
    pub fn open_out_file(&mut self, fname: &str) {
        self.close_out_file();
        self.writer = Some(CodeWriter::new(&self.out_path.join(fname)));
    }

    pub fn generate(&mut self, model: &SmithyModel) {
        self.open_out_file("s3.1.rs");
        self.gen_ops_enum(model);

        self.open_out_file("s3.2.rs");
        for (ref name, _) in model.get_shapes_with_trait(SM_ENUM) {
            self.gen_enum_type(model, name);
        }

        self.open_out_file("s3.3.rs");
        for (ref name, ref op) in model.get_shapes_by_type("operation") {
            self.gen_server_op(model, name, op);
        }

        self.open_out_file("s3.4.rs");
        while !self.xml_models_queue.is_empty() {
            let shape_key = self.xml_models_queue.pop().unwrap();
            if self.xml_models_done.insert(shape_key.clone()) {
                self.gen_xml_model(model, &shape_key);
            }
        }

        self.close_out_file();
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

        let (head_inputs, body_inputs) = model.get_members_partition(&input);
        let (head_outputs, body_outputs) = model.get_members_partition(&output);

        let head_decoders = head_inputs
            .iter()
            .map(|(ref field_name, ref field)| self.gen_head_decoder(model, field_name, field))
            .collect::<Vec<_>>();
        let head_encoders = head_outputs
            .iter()
            .map(|(ref field_name, ref field)| self.gen_head_encoder(model, field_name, field))
            .collect::<Vec<_>>();

        let body_decoder = self.gen_body_decoder(model, body_inputs);
        let body_encoder = self.gen_body_encoder(model, body_outputs);

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
                        #( #head_decoders )*
                        #body_decoder
                        Ok(b.build()?)
                    })
                }

                fn encode_output(mut o: Self::Output) -> TraitFuture<'static, HttpResponse, S3Error> {
                    Box::pin(async move {
                        let mut r = responder();
                        let h = r.headers_mut().unwrap();
                        #( #head_encoders )*
                        #body_encoder
                    })
                }

            }

        });
    }

    fn gen_head_decoder(
        &mut self,
        _model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> TokenStream {
        let field_snake = snake(field_name);
        let field_id = format_ident!("{}", field_snake);
        let set_field = format_ident!("set_{}", field_snake);
        let get_field = format_ident!("get_{}", field_snake);
        if SmithyModel::has_trait(field, SM_HTTP_LABEL) {
            return quote! { b = b.#field_id(req.#get_field()); };
        }
        if SmithyModel::has_trait(field, SM_HTTP_QUERY) {
            let param = SmithyModel::get_trait(field, SM_HTTP_QUERY);
            return quote! { b = b.#set_field(req.get_param(#param)); };
        }
        if SmithyModel::has_trait(field, SM_HTTP_HEADER) {
            let header = SmithyModel::get_trait(field, SM_HTTP_HEADER);
            return quote! { b = b.#set_field(req.get_header(#header)); };
        }
        if SmithyModel::has_trait(field, SM_HTTP_PREFIX_HEADERS) {
            let header_map = SmithyModel::get_trait(field, SM_HTTP_PREFIX_HEADERS);
            return quote! { b = b.#set_field(req.get_header_map(#header_map)); };
        }
        panic!("gen_head_decoder {} {:#?}", field_name, field);
    }

    fn gen_head_encoder(
        &mut self,
        _model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> TokenStream {
        let field_snake = snake(field_name);
        let field_id = format_ident!("{}", field_snake);
        if SmithyModel::has_trait(field, SM_HTTP_HEADER) {
            let header = SmithyModel::get_trait(field, SM_HTTP_HEADER);
            return quote! { o.#field_id().set_header(h, #header); };
        }
        if SmithyModel::has_trait(field, SM_HTTP_PREFIX_HEADERS) {
            let header_prefix = SmithyModel::get_trait(field, SM_HTTP_PREFIX_HEADERS);
            return quote! { o.#field_id().set_header(h, #header_prefix); };
        }
        panic!("gen_head_encoder {} {:#?}", field_name, field);
    }

    fn gen_body_decoder(
        &mut self,
        model: &SmithyModel,
        fields: Vec<(String, Value)>,
    ) -> TokenStream {
        let decoders = fields
            .iter()
            .map(|(ref field_name, ref field)| {
                self.gen_body_decoder_field(model, field_name, field)
            })
            .collect::<Vec<_>>();
        return quote! { #( #decoders )* };
    }

    fn gen_body_decoder_field(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> TokenStream {
        let field_snake = snake(field_name);
        let set_field = format_ident!("set_{}", field_snake);
        let (shape_name, shape) = model.get_target_shape(field);
        let shape_ident = format_ident!("{}", shape_name);
        let shape_type = SmithyModel::get_shape_type(&shape);

        if SmithyModel::has_trait(field, SM_HTTP_PAYLOAD) {
            if shape_type == "blob" {
                return quote! {
                    b = b.#set_field(Some(ByteStream::new(req.take_body().into())));
                };
            }
            if SmithyModel::has_trait(field, SM_XML_NAME) {
                let shape_key = field["target"].as_str().unwrap();
                if !self.xml_models_done.contains(shape_key) {
                    self.xml_models_queue.push(shape_key.to_string());
                }
                let xml_name = SmithyModel::get_trait(field, SM_XML_NAME);
                return quote! {
                    b = b.#set_field(Some(#shape_ident::decode_xml(
                        #xml_name, req.take_body_bytes().await?)?));
                };
            }
            if shape_type == "string" {
                if SmithyModel::has_trait(&shape, SM_ENUM) {
                    return quote! {
                    b = b.#set_field(aws_sdk_s3::model::#shape_ident::from_http(
                        req.take_body_string().await?.as_str()));
                    };
                } else {
                    return quote! {
                        b = b.#set_field(Some(req.take_body_string().await?));
                    };
                }
            }
        }

        // TODO gen_body_decoder_field - SelectObjectContent only
        return quote! { b = b.#set_field(None); };
    }

    fn gen_body_encoder(
        &mut self,
        model: &SmithyModel,
        fields: Vec<(String, Value)>,
    ) -> TokenStream {
        return quote! { Ok(r.body(Body::empty())?) };
    }

    fn gen_body_encoder_field(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> TokenStream {
        let field_snake = snake(field_name);
        let field_id = format_ident!("{}", field_snake);

        // TODO gen_body_encoder_field
        return quote! { Body::wrap_stream(o.#field_id) };

        // if SmithyModel::has_trait(field, SM_HTTP_PAYLOAD) {
        //     let (_shape_name, shape) = model.get_target_shape(field);
        //     if SmithyModel::get_shape_type(&shape) == "blob" {
        //         return Some(quote! { Body::wrap_stream(o.#field_id) });
        //     }
        // }
        // let xml_name = if SmithyModel::has_trait(field, SM_XML_NAME) {
        //     SmithyModel::get_trait(field, SM_XML_NAME)
        // } else {
        //     field_name.to_string()
        // };
        // let xml_ident = format_ident!("{}", xml_name);

        // // TODO gen_body_encoder XML
        // let shape_key = field["target"].as_str().unwrap();
        // // if !self.xml_models_done.contains(shape_key) {
        // //     self.xml_models_queue.push(shape_key.to_string());
        // // }
        // Some(quote! {
        //     #xml_ident::encode_xml(&mut w, #xml_name, o.#field_id())
        //     // xml_tag!(#xml_name, w, { });
        // })

        // let output_xml_name = SmithyModel::get_trait(&output, SM_XML_NAME);
        // let body_maker = if output_xml_name.is_empty() {
        //     match body_encoders.len() {
        //         0 => quote! {
        //             Ok(r.body( Body::empty() )?)
        //         },
        //         1 => quote! {
        //             Ok(r.body( #(#body_encoders)* )?)
        //         },
        //         // _ => panic!("bad output shape {} {:#?}", name, body_encoders),
        //         _ => quote! {
        //             Ok(r.body( Body::empty() )?)
        //         },
        //     }
        // } else {
        //     quote! {
        //         // Ok(r.body(
        //         //     Body::from(xml_doc!(output_xml_name, w, {
        //         //         #( #body_encoders )*
        //         //     }))
        //         // )?)
        //         Ok(r.body(Body::empty())?)
        //     }
        // };
    }

    fn gen_xml_model(&mut self, model: &SmithyModel, shape_key: &str) {
        let (name, shape) = model.get_shape_by_key(shape_key);
        let xml_ident = format_ident!("{}", name);
        let error_str = format!("Bad {}", name);
        let field_decoders = model
            .get_members(&shape)
            .map(|(ref field_name, ref field)| self.gen_xml_decoder(model, field_name, field))
            .collect::<Vec<_>>();

        self.write_code(quote! {

            pub struct #xml_ident {}

            impl #xml_ident {

                pub fn decode_xml(xml_name: &str, bytes: Bytes)
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

                pub fn encode_xml(xml_name: &str, w: &mut ScopeWriter)
                    -> Result<(), S3Error> {
                    Ok(())
                }

            }

        });
    }

    fn gen_xml_decoder(
        &mut self,
        model: &SmithyModel,
        field_name: &str,
        field: &Value,
    ) -> TokenStream {
        let (shape_name, shape) = model.get_target_shape(field);
        let field_snake = snake(field_name);
        let _field_id = format_ident!("{}", field_snake);
        let set_field = format_ident!("set_{}", field_snake);

        match SmithyModel::get_shape_type(&shape).as_str() {
            "list" => quote! {
                el if el.matches(#shape_name) => {
                    b = b.#set_field(None); // TODO decode_xml LIST
                    // b = b.#field(None);
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
        let w = self.writer.as_mut().unwrap();
        w.write_all(code.to_string().as_bytes()).unwrap();
        w.write_all(b"\n\n").unwrap();
    }

    fn _writeln<T: AsRef<[u8]>>(&mut self, s: T) {
        let w = self.writer.as_mut().unwrap();
        w.write_all(s.as_ref()).unwrap();
        w.write_all(b"\n").unwrap();
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

    pub fn get_shape_by_key(&self, k: &str) -> (String, Value) {
        (camel(&unprefix(k)), self.json["shapes"][k].to_owned())
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
            .map(|(k, v)| (camel(&unprefix(k)), v.to_owned()))
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
            .map(|(k, v)| (camel(&unprefix(k)), v.to_owned()))
    }

    pub fn get_target_shape(&self, p: &Value) -> (String, Value) {
        match &p["target"] {
            Value::String(s) => (camel(&unprefix(s)), self.json["shapes"][s].to_owned()),
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

    pub fn get_members_partition<'a>(
        &'a self,
        p: &'a Value,
    ) -> (Vec<(String, Value)>, Vec<(String, Value)>) {
        p["members"]
            .as_object()
            .unwrap_or(&self.empty)
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_owned()))
            .partition(|(_, ref field)| {
                SmithyModel::has_trait(field, SM_HTTP_LABEL)
                    || SmithyModel::has_trait(field, SM_HTTP_QUERY)
                    || SmithyModel::has_trait(field, SM_HTTP_HEADER)
                    || SmithyModel::has_trait(field, SM_HTTP_PREFIX_HEADERS)
            })
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

    pub fn get_trait_value(p: &Value, t: &str) -> Value {
        p.pointer(&format!("/traits/{}", t))
            .map_or(Value::Null, |v| v.to_owned())
    }

    pub fn get_trait(p: &Value, t: &str) -> String {
        p.pointer(&format!("/traits/{}", t))
            .map_or("".to_string(), |v| v.as_str().unwrap().to_string())
    }
}

/// CodeWriter pipes generated code through rustfmt and into a file
pub struct CodeWriter {
    path: PathBuf,
    rustfmt: Option<Child>,
    w: Option<BufWriter<ChildStdin>>,
}
impl CodeWriter {
    fn new(file_path: &Path) -> Self {
        println!("CodeWriter file {:?}", file_path);
        let file = File::create(file_path).unwrap();
        let mut rustfmt = Command::new("rustfmt")
            .arg("--edition=2021")
            .stdin(Stdio::piped())
            .stdout(file)
            .spawn()
            .unwrap();
        println!("CodeWriter rustfmt {:?}", rustfmt);
        let w = BufWriter::new(rustfmt.stdin.take().unwrap());
        CodeWriter {
            path: file_path.to_path_buf(),
            rustfmt: Some(rustfmt),
            w: Some(w),
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
        self.rustfmt.take().unwrap().wait()?;
        println!("CodeWriter done {}", self.path.display());
        Ok(())
    }
}

/// unprefix returns just the suffix for `prefix#suffix` strings
fn unprefix(s: &str) -> String {
    s.split_once('#').unwrap().1.to_string()
}

/// camel changes from MIXOfUPPERCaseAndCamelCase to MixOfUpperCaseAndCamelCase
fn camel(s: &str) -> String {
    let mut out = String::new();
    let mut upper_streak = 0;
    for c in s.chars() {
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

/// snake changes from CamelCase to snake_case
fn snake(s: &str) -> String {
    let mut out = String::new();
    let mut upper_streak = 0;
    for mut c in s.chars() {
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
