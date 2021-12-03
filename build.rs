//! # build.rs
//!
//! This build script runs automatically by cargo
//! and generates `$OUT_DIR/s3.rs` based on `models/s3.json` smithy JSON AST
//!
//! See:
//! - https://awslabs.github.io/smithy/1.0/spec/index.html
//! - https://awslabs.github.io/smithy/1.0/spec/core/json-ast.html
//!
use serde_json::{Map, Value};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
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

const HEADER: &'static str = "\
/// This file is generated by build.rs. DO NOT EDIT.

use crate::{http::*, xml::xml_to_data};
use hyper::{Body, StatusCode, body::Bytes, header::HeaderValue};
use aws_smithy_http::byte_stream::ByteStream;
use aws_smithy_xml::decode::{Document, ScopedDecoder};
";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir);
    let models_path = Path::new(MODELS_DIR);
    let s3_out_path = out_path.join("s3.rs");
    let s3_model_path = models_path.join("s3.json");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", s3_model_path.display());
    let s3_model: Value = serde_json::from_reader(File::open(s3_model_path).unwrap()).unwrap();
    let mut s3_out = BufWriter::new(File::create(s3_out_path).unwrap());
    Smithy::new(s3_model).generate_service_code(&mut s3_out);
    s3_out.flush().unwrap();
}

pub struct Smithy {
    pub model: Value,
    // map of xml model names to whether they were already generated
    xml_models: HashMap<String, bool>,
}

impl Smithy {
    pub fn new(model: Value) -> Self {
        Self {
            model,
            xml_models: HashMap::new(),
        }
    }

    pub fn generate_service_code(&mut self, w: &mut dyn Write) {
        writeln!(w, "{}", HEADER).unwrap();
        self.gen_ops_enum(w);
        self.gen_ops_macro(w);
        self.gen_server_ops(w);
        self.gen_xml_models(w);
        self.gen_enum_types(w);
        writeln!(w, "").unwrap();
        writeln!(w, "// build.rs done.").unwrap();
    }

    ///
    /// Generates a simple enum of operation kinds.
    ///
    /// The enum has no attached state to any of the operations.
    /// It might be interesting to consider a more complex enum if needed by the daemon,
    /// or perhaps that would instead go to it's own enum, with auto-generated-mapping to this one.
    ///
    fn gen_ops_enum(&self, w: &mut dyn Write) {
        writeln!(w, "#[derive(Debug, PartialEq, Eq, Clone, Copy)]").unwrap();
        writeln!(w, "pub enum S3Ops {{").unwrap();
        for (name, _op) in self.get_shapes_by_type("operation").iter() {
            writeln!(w, "    {},", name).unwrap();
        }
        writeln!(w, "}}\n").unwrap();
    }

    ///
    /// Generates a macro that expands a second macro per operation
    ///
    fn gen_ops_macro(&self, w: &mut dyn Write) {
        writeln!(
            w,
            "/// This macro calls a provided $macro for each S3 operation to generate code per op."
        )
        .unwrap();
        writeln!(w, "macro_rules! generate_code_for_each_s3_op {{").unwrap();
        writeln!(w, "    ($macro:ident) => {{").unwrap();
        for (name, _op) in self.get_shapes_by_type("operation").iter() {
            writeln!(w, "        $macro!({});", name).unwrap();
        }
        writeln!(w, "    }};").unwrap();
        writeln!(w, "}}").unwrap();
        writeln!(w, "").unwrap();
        writeln!(w, "pub(crate) use generate_code_for_each_s3_op;").unwrap();
        writeln!(w, "").unwrap();

        writeln!(w, "macro_rules! generate_match_for_each_s3_op {{").unwrap();
        writeln!(w, "    ($macro:ident, $op:expr) => {{").unwrap();
        writeln!(w, "        match ($op) {{").unwrap();
        for (name, _op) in self.get_shapes_by_type("operation").iter() {
            writeln!(w, "            S3Ops::{0} => $macro!({0}),", name).unwrap();
        }
        writeln!(w, "        }}").unwrap();
        writeln!(w, "    }};").unwrap();
        writeln!(w, "}}").unwrap();
        writeln!(w, "").unwrap();
        writeln!(w, "pub(crate) use generate_match_for_each_s3_op;").unwrap();
        writeln!(w, "").unwrap();
    }

    ///
    /// Generates an impl of ServerOperation per operation.
    ///
    fn gen_server_ops(&mut self, w: &mut dyn Write) {
        for (name, op) in self.get_shapes_by_type("operation").iter() {
            let (_, input) = self.get_target_shape(&op["input"]);
            let (_, output) = self.get_target_shape(&op["output"]);
            assert_eq!(self.get_shape_type(&op), "operation");
            if !input.is_null() {
                assert_eq!(self.get_shape_type(&input), "structure");
            }
            if !output.is_null() {
                assert_eq!(self.get_shape_type(&output), "structure");
            }

            let mut is_input_body_stream = false;
            let mut is_output_body_stream = false;

            writeln!(w, "pub struct {} {{}}", name).unwrap();
            writeln!(w, "impl ServerOperation for {} {{", name).unwrap();
            writeln!(w, "    type Input = aws_sdk_s3::input::{}Input;", name).unwrap();
            writeln!(w, "    type Output = aws_sdk_s3::output::{}Output;", name).unwrap();
            writeln!(w, "    type Error = aws_sdk_s3::error::{}Error;", name).unwrap();
            writeln!(w, "    const OP: S3Ops = S3Ops::{};", name).unwrap();

            writeln!(w, "    fn decode_input(req: &mut S3Request)").unwrap();
            writeln!(w, "        -> TraitFuture<Self::Input, S3Error> {{").unwrap();
            writeln!(w, "        Box::pin(async move {{").unwrap();
            writeln!(w, "            let mut b = Self::Input::builder();").unwrap();
            for (field_name, field) in self.get_members(&input).iter() {
                is_input_body_stream =
                    self.decode_input_field(w, field_name, field) || is_input_body_stream;
            }
            writeln!(w, "           Ok(b.build()?)").unwrap();
            writeln!(w, "       }})").unwrap();
            writeln!(w, "    }}").unwrap();

            writeln!(w, "    fn encode_output(o: Self::Output)").unwrap();
            writeln!(
                w,
                "        -> TraitFuture<'static, HttpResponse, S3Error> {{"
            )
            .unwrap();
            writeln!(w, "        Box::pin(async move {{").unwrap();
            writeln!(w, "            let mut r = responder();").unwrap();
            writeln!(w, "            let h = r.headers_mut().unwrap();").unwrap();
            for (field_name, field) in self.get_members(&output).iter() {
                is_output_body_stream =
                    self.decode_output_field(w, field_name, field) || is_output_body_stream;
            }
            if is_output_body_stream {
                writeln!(w, "            Ok(r.body(Body::wrap_stream(o.body))?)").unwrap();
            } else {
                writeln!(w, "            Ok(r.body(Body::empty())?)").unwrap();
            }
            writeln!(w, "        }})").unwrap();
            writeln!(w, "    }}").unwrap();

            writeln!(
                w,
                "    const IS_INPUT_BODY_STREAMING: bool = {};",
                is_input_body_stream
            )
            .unwrap();
            writeln!(
                w,
                "    const IS_OUTPUT_BODY_STREAMING: bool = {};",
                is_output_body_stream
            )
            .unwrap();
            writeln!(w, "}}").unwrap();
        }
        writeln!(w, "\n").unwrap();
    }

    fn decode_input_field(&mut self, w: &mut dyn Write, field_name: &str, field: &Value) -> bool {
        let mut is_input_body_stream = false;
        let field_snake = field_name.snake();
        let indent = "            ";
        if self.has_trait(field, SM_HTTP_LABEL) {
            writeln!(w, "{}b = b.{}(req.get_{1}());", indent, field_snake).unwrap();
        } else if self.has_trait(field, SM_HTTP_QUERY) {
            writeln!(
                w,
                "{}b = b.set_{}(req.get_param(\"{}\"));",
                indent,
                field_snake,
                self.get_trait_str(field, SM_HTTP_QUERY)
            )
            .unwrap();
        } else if self.has_trait(field, SM_HTTP_HEADER) {
            writeln!(
                w,
                "{}b = b.set_{}(req.get_header(\"{}\"));",
                indent,
                field_snake,
                self.get_trait_str(field, SM_HTTP_HEADER)
            )
            .unwrap();
        } else if self.has_trait(field, SM_HTTP_PREFIX_HEADERS) {
            writeln!(
                w,
                "{}b = b.set_{}(req.get_header_map(\"{}\"));",
                indent,
                field_snake,
                self.get_trait_str(field, SM_HTTP_PREFIX_HEADERS)
            )
            .unwrap();
        } else if self.has_trait(field, SM_HTTP_PAYLOAD) {
            let (shape_name, shape) = self.get_target_shape(field);
            if self.get_shape_type(&shape) == "blob" {
                is_input_body_stream = true;
                writeln!(
                    w,
                    "{}b = b.set_{}(Some(ByteStream::new(req.take_body().into())));",
                    indent, field_snake
                )
                .unwrap();
            } else if self.has_trait(field, SM_XML_NAME) {
                if !self.xml_models.contains_key(&shape_name) {
                    self.xml_models.insert(shape_name.to_owned(), false);
                }
                writeln!(
                    w,
                    "{}b = b.set_{}(Some({}::decode_xml(",
                    indent, field_snake, shape_name,
                )
                .unwrap();
                writeln!(
                    w,
                    "{}    req.take_body_bytes().await?, \"{}\")?));",
                    indent,
                    self.get_trait_str(field, SM_XML_NAME)
                )
                .unwrap();
            } else {
                writeln!(
                    w,
                    "{}b = b.set_{}(None); // TODO decode_input PAYLOAD ",
                    indent, field_snake
                )
                .unwrap();
            }
        } else {
            writeln!(
                w,
                "{}b = b.set_{}(None); // TODO decode_input XML ???",
                indent, field_snake
            )
            .unwrap();
        }
        is_input_body_stream
    }

    fn decode_output_field(&self, w: &mut dyn Write, field_name: &str, field: &Value) -> bool {
        let mut is_output_body_stream = false;
        let field_snake = field_name.snake();
        let indent = "            ";
        if self.has_trait(field, SM_HTTP_HEADER) {
            writeln!(
                w,
                "{}o.{}().set_header(h, \"{}\");",
                indent,
                field_snake,
                self.get_trait_str(field, SM_HTTP_HEADER)
            )
            .unwrap();
        } else if self.has_trait(field, SM_HTTP_PAYLOAD) {
            let (_shape_name, shape) = self.get_target_shape(field);
            if self.get_shape_type(&shape) == "blob" {
                is_output_body_stream = true;
            } else {
                writeln!(
                    w,
                    "{}// TODO encode_output XML PAYLOAD {}",
                    indent, field_snake
                )
                .unwrap();
            }
            // TODO
        } else if self.has_trait(field, SM_XML_NAME) {
            writeln!(w, "{}// TODO encode_output XML {}", indent, field_snake).unwrap();
        } else {
            writeln!(
                w,
                "{}// TODO encode_output XML (UNKNOWN) {}",
                indent, field_snake
            )
            .unwrap();
        }
        is_output_body_stream
    }

    fn gen_xml_models(&self, w: &mut dyn Write) {
        for (name, shape) in self.get_shapes_by_type("structure").iter() {
            if !self.xml_models.contains_key(name) {
                continue;
            }
            writeln!(
                w,
                "\
pub struct {name} {{}}
impl {name} {{
    pub fn decode_xml(bytes: Bytes, xml_name: &str)
        -> Result<aws_sdk_s3::model::{name}, S3Error> {{
        let mut b = aws_sdk_s3::model::{name}::builder();
        let mut doc = Document::try_from(&bytes[..]).unwrap();
        let mut d = doc.root_element().unwrap();
        if !d.start_el().matches(xml_name) {{
            Err(S3Error::bad_request(\"Bad {name}\"))?;
        }}
        while let Some(mut d) = d.next_tag() {{
            match d.start_el() {{",
                name = name
            )
            .unwrap();

            let indent = "                ";
            for (field_name, field) in self.get_members(&shape).iter() {
                let (shape_name, shape) = self.get_target_shape(field);
                let field_snake = field_name.snake();
                writeln!(w, "{}el if el.matches(\"{}\") => {{", indent, field_name,).unwrap();
                match self.get_shape_type(&shape).as_str() {
                    "list" => writeln!(
                        w,
                        "{}    b = b.set_{}(None); // TODO decode_xml LIST",
                        indent, field_snake,
                    )
                    .unwrap(),
                    "structure" => writeln!(
                        w,
                        "{}    b = b.set_{}(None); // TODO decode_xml STRUCTURE",
                        indent, field_snake,
                    )
                    .unwrap(),
                    _ => writeln!(
                        w,
                        "{}    b = b.set_{}(xml_to_data(&mut d));",
                        indent, field_snake,
                    )
                    .unwrap(),
                }
                writeln!(w, "{}}}", indent).unwrap();
            }

            writeln!(
                w,
                "\
                {}_ => Err(S3Error::bad_request(\"Bad {}\"))?,
            }}
        }}
        Ok(b.build())
    }}
}}",
                indent, name,
            )
            .unwrap();
        }
    }

    fn gen_enum_types(&self, w: &mut dyn Write) {
        for (name, _shape) in self.get_shapes_with_trait(SM_ENUM).iter() {
            writeln!(
                w,
                "\
impl FromHttp for aws_sdk_s3::model::{name} {{
    fn from_http(v: &str) -> Option<Self> {{
        Some(Self::from(v))
    }}
}}
impl ToHeader for &aws_sdk_s3::model::{name} {{
    fn to_header(self) -> Option<HeaderValue> {{
        self.as_str().to_header()
    }}
}}",
                name = name
            )
            .unwrap();
        }
    }

    /////////////
    // helpers //
    /////////////

    pub fn get_shapes_by_type(&self, t: &str) -> Vec<(String, Value)> {
        self.model["shapes"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_k, v)| v["type"].as_str() == Some(t))
            .map(|(k, v)| (k.unprefix().uncaps(), v.to_owned()))
            .collect()
    }

    pub fn get_shapes_with_trait(&self, t: &str) -> Vec<(String, Value)> {
        self.model["shapes"]
            .as_object()
            .unwrap()
            .iter()
            .filter(|(_k, v)| self.has_trait(&v, t))
            .map(|(k, v)| (k.unprefix().uncaps(), v.to_owned()))
            .collect()
    }

    pub fn get_target_shape(&self, p: &Value) -> (String, Value) {
        match &p["target"] {
            Value::String(s) => (s.unprefix().uncaps(), self.model["shapes"][s].to_owned()),
            _ => ("".to_string(), Value::Null),
        }
    }

    pub fn get_shape_type(&self, p: &Value) -> String {
        match &p["type"] {
            Value::String(s) => s.to_string(),
            _ => "".to_string(),
        }
    }

    pub fn has_trait(&self, p: &Value, t: &str) -> bool {
        p.pointer(&format!("/traits/{}", t)).is_some()
    }

    pub fn get_trait(&self, p: &Value, t: &str) -> Value {
        p.pointer(&format!("/traits/{}", t))
            .map_or(Value::Null, |v| v.to_owned())
    }

    pub fn get_trait_str(&self, p: &Value, t: &str) -> String {
        p.pointer(&format!("/traits/{}", t))
            .map_or("".to_string(), |v| v.as_str().unwrap().to_string())
    }

    pub fn get_members(&self, p: &Value) -> Vec<(String, Value)> {
        p["members"]
            .as_object()
            .unwrap_or(&Map::new())
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_owned()))
            .collect()
    }
}

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
