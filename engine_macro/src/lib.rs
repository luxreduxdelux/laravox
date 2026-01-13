use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use std::fmt::Display;
use syn::{ItemFn, ItemStruct};

//================================================================

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Function {
    from: String,
    info: String,
    #[darling(multiple)]
    parameter: Vec<Value>,
    #[darling(multiple)]
    result: Vec<Value>,
}

impl Function {
    fn write(&self, name: &str, method: bool) {
        let mut buffer = String::new();

        for line in self.info.lines() {
            buffer.push_str(&format!("---{}\n", line.trim()));
        }

        let mut buffer_parameter = String::new();

        for (i, parameter) in self.parameter.iter().enumerate() {
            buffer.push_str(&format!(
                "---@param {} {} # {}\n",
                parameter.get_name(),
                parameter.kind,
                parameter.info
            ));

            if i == self.parameter.len() - 1 {
                buffer_parameter.push_str(&parameter.name);
            } else {
                buffer_parameter.push_str(&format!("{}, ", parameter.name));
            }
        }

        for result in &self.result {
            buffer.push_str(&format!(
                "---@return {} {} # {}\n",
                result.get_kind(),
                result.name,
                result.info
            ));
        }

        let from = if method {
            &self.from
        } else {
            &format!("laravox.{}", self.from)
        };
        let method = if method { ":" } else { "." };

        buffer.push_str(&format!(
            "function {from}{method}{name}({buffer_parameter}) end\n",
        ));

        write_to_out(&format!("function_{from}.{name}"), &buffer);
    }
}

//================================================================

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Class {
    name: Option<String>,
    info: String,
}

impl Class {
    fn write(&self, name: &str) {
        let mut buffer = String::new();
        let name = if let Some(name) = &self.name {
            name
        } else {
            name
        };

        for line in self.info.lines() {
            buffer.push_str(&format!("---{}\n", line.trim()));
        }

        buffer.push_str(&format!("---@class {name}\n{name} = {{}}"));

        write_to_out(&format!("class_{}", name), &buffer);
    }
}

//================================================================

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Module {
    name: String,
    info: String,
}

impl Module {
    fn write(&self) {
        let mut buffer = String::new();

        for line in self.info.lines() {
            buffer.push_str(&format!("---{}\n", line.trim()));
        }

        buffer.push_str(&format!("laravox.{} = {{}}", self.name));

        write_to_out(&format!("module_{}", &self.name), &buffer);
    }
}

//================================================================

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct Value {
    name: String,
    info: String,
    kind: ValueKind,
    optional: Option<bool>,
}

impl Value {
    fn get_name(&self) -> String {
        if let Some(optional) = self.optional
            && optional
        {
            format!("{}?", self.name)
        } else {
            self.name.clone()
        }
    }

    fn get_kind(&self) -> String {
        if let Some(optional) = self.optional
            && optional
        {
            format!("{}|nil", self.kind)
        } else {
            self.kind.to_string()
        }
    }
}

#[derive(Debug, FromMeta)]
enum ValueKind {
    String,
    Number,
    Boolean,
    Function,
    Table,
    #[darling(rename = "Vector2")]
    Vector2,
    #[darling(rename = "Vector3")]
    Vector3,
    #[darling(rename = "Box2")]
    Box2,
    #[darling(rename = "Box3")]
    Box3,
    #[darling(rename = "Camera2D")]
    Camera2D,
    #[darling(rename = "Camera3D")]
    Camera3D,
    #[darling(rename = "Color")]
    Color,
    UserData {
        name: String,
    },
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::String => f.write_str("string"),
            ValueKind::Number => f.write_str("number"),
            ValueKind::Boolean => f.write_str("boolean"),
            ValueKind::Function => f.write_str("function"),
            ValueKind::Table => f.write_str("table"),
            ValueKind::Vector2 => f.write_str("Vector2"),
            ValueKind::Vector3 => f.write_str("Vector3"),
            ValueKind::Box2 => f.write_str("Box2"),
            ValueKind::Box3 => f.write_str("Box3"),
            ValueKind::Camera2D => f.write_str("Camera2D"),
            ValueKind::Camera3D => f.write_str("Camera3D"),
            ValueKind::Color => f.write_str("Color"),
            ValueKind::UserData { name } => f.write_str(name),
        }
    }
}

//================================================================

#[proc_macro_attribute]
pub fn function(argument_list: TokenStream, input: TokenStream) -> TokenStream {
    let function: Function = match syn::parse(argument_list) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let input = syn::parse_macro_input!(input as ItemFn);

    function.write(&input.sig.ident.to_string(), false);

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn method(argument_list: TokenStream, input: TokenStream) -> TokenStream {
    let function: Function = match syn::parse(argument_list) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let input = syn::parse_macro_input!(input as ItemFn);

    function.write(&input.sig.ident.to_string(), true);

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn class(argument_list: TokenStream, input: TokenStream) -> TokenStream {
    let class: Class = match syn::parse(argument_list) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };
    let input = syn::parse_macro_input!(input as ItemStruct);

    class.write(&input.ident.to_string());

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn module(argument_list: TokenStream, input: TokenStream) -> TokenStream {
    let module: Module = match syn::parse(argument_list) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    module.write();

    input
}

fn write_to_out(path: &str, data: &str) {
    if !std::fs::exists("engine_macro/out").unwrap() {
        std::fs::create_dir("engine_macro/out").unwrap();
    }

    std::fs::write(format!("engine_macro/out/{}", path), data).unwrap();
}
