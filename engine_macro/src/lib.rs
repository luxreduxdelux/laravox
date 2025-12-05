/*
* Copyright (c) 2025 luxreduxdelux
*
* Redistribution and use in source and binary forms, with or without
* modification, are permitted provided that the following conditions are met:
*
* 1. Redistributions of source code must retain the above copyright notice,
* this list of conditions and the following disclaimer.
*
* 2. Redistributions in binary form must reproduce the above copyright notice,
* this list of conditions and the following disclaimer in the documentation
* and/or other materials provided with the distribution.
*
* Subject to the terms and conditions of this license, each copyright holder
* and contributor hereby grants to those receiving rights under this license
* a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable
* (except for failure to satisfy the conditions of this license) patent license
* to make, have made, use, offer to sell, sell, import, and otherwise transfer
* this software, where such license applies only to those patent claims, already
* acquired or hereafter acquired, licensable by such copyright holder or
* contributor that are necessarily infringed by:
*
* (a) their Contribution(s) (the licensed copyrights of copyright holders and
* non-copyrightable additions of contributors, in source or binary form) alone;
* or
*
* (b) combination of their Contribution(s) with the work of authorship to which
* such Contribution(s) was added by such copyright holder or contributor, if,
* at the time the Contribution is added, such addition causes such combination
* to be necessarily infringed. The patent license shall not apply to any other
* combinations which include the Contribution.
*
* Except as expressly stated above, no rights or licenses from any copyright
* holder or contributor is granted under this license, whether expressly, by
* implication, estoppel or otherwise.
*
* DISCLAIMER
*
* THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
* AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
* IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
* DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE LIABLE
* FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
* DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
* SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
* CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
* OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
* OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

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
                parameter.name, parameter.kind, parameter.info
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
                result.kind, result.name, result.info
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
}

#[derive(Debug, FromMeta)]
enum ValueKind {
    String,
    Number,
    Boolean,
    Function,
    Table,
    #[darling(rename = "vector_2")]
    Vector2,
    #[darling(rename = "vector_3")]
    Vector3,
    #[darling(rename = "box_2")]
    Box2,
    #[darling(rename = "box_3")]
    Box3,
    #[darling(rename = "camera_2D")]
    Camera2D,
    #[darling(rename = "camera_3D")]
    Camera3D,
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
            ValueKind::Vector2 => f.write_str("vector_2"),
            ValueKind::Vector3 => f.write_str("vector_3"),
            ValueKind::Box2 => f.write_str("box_2"),
            ValueKind::Box3 => f.write_str("box_3"),
            ValueKind::Camera2D => f.write_str("camera_2D"),
            ValueKind::Camera3D => f.write_str("camera_3D"),
            ValueKind::Color => f.write_str("color"),
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

    class.write(&input.ident.to_string().to_lowercase());

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
