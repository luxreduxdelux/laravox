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

use engine_macro::*;

//================================================================

use mlua::prelude::*;
use serde_json::Value;

//================================================================

#[rustfmt::skip]
#[module(name = "data", info = "Data API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let data = lua.create_table()?;

    data.set("get_list",    lua.create_function(self::get_list)?)?;
    data.set("get_file",    lua.create_function(self::get_file)?)?;
    data.set("set_file",    lua.create_function(self::set_file)?)?;
    data.set("get_kind",    lua.create_function(self::get_kind)?)?;
    data.set("into_string", lua.create_function(self::into_string)?)?;
    data.set("from_string", lua.create_function(self::from_string)?)?;
    data.set("get_system",  lua.create_function(self::get_system)?)?;

    global.set("data", data)?;

    Ok(())
}

//================================================================

#[function(
    from = "data",
    info = "Get a full list of every file in a given directory.",
    parameter(name = "path", info = "Path to directory.", kind = "string"),
    parameter(name = "recurse", info = "Recurse directory search.", kind = "boolean"),
    result(
        name = "file_list",
        info = "Table array of every file in given directory.",
        kind = "table"
    )
)]
fn get_list(_: &mlua::Lua, (path, recurse): (String, bool)) -> mlua::Result<Vec<String>> {
    let mut list = Vec::new();
    get_list_aux(&mut list, path, recurse)?;

    Ok(list)
}

fn get_list_aux(list: &mut Vec<String>, path: String, recurse: bool) -> anyhow::Result<()> {
    let file_path = std::fs::read_dir(path)?;

    for file in file_path {
        let file = file?;
        let path = file.path().display().to_string();

        list.push(path.clone());

        if recurse && file.file_type()?.is_dir() {
            get_list_aux(list, path, recurse)?;
        }
    }

    Ok(())
}

#[function(
    from = "data",
    info = "Get the data of a file.",
    parameter(name = "path", info = "Path to file.", kind = "string"),
    parameter(
        name = "binary",
        info = "Return the value as binary, or as a string.",
        kind = "boolean"
    ),
    result(
        name = "data",
        info = "File data.",
        kind(user_data(name = "table|string"))
    )
)]
fn get_file(lua: &mlua::Lua, (path, binary): (String, bool)) -> mlua::Result<mlua::Value> {
    if binary {
        Ok(lua.to_value(&std::fs::read(path)?)?)
    } else {
        Ok(lua.to_value(&std::fs::read_to_string(path)?)?)
    }
}

#[function(
    from = "data",
    info = "Set the data of a file.",
    parameter(name = "path", info = "Path to file.", kind = "string"),
    parameter(name = "data", info = "Data to write to file.", kind = "string")
)]
fn set_file(_: &mlua::Lua, (path, data): (String, String)) -> mlua::Result<()> {
    Ok(std::fs::write(path, data)?)
}

#[function(
    from = "data",
    info = "Check the kind of a path.",
    parameter(name = "path", info = "Path.", kind = "string"),
    result(
        name = "kind",
        info = "Path kind.",
        kind(user_data(name = "PathKind")),
        optional = true
    )
)]
fn get_kind(_: &mlua::Lua, path: String) -> mlua::Result<Option<usize>> {
    let path = std::path::Path::new(&path);

    if path.exists() {
        if path.is_file() {
            return Ok(Some(0));
        } else if path.is_dir() {
            return Ok(Some(1));
        } else if path.is_symlink() {
            return Ok(Some(2));
        }
    }

    Ok(None)
}

#[function(
    from = "data",
    info = "Serialize a Lua table as a string.",
    parameter(
        name = "data",
        info = "Lua table to serialize as a string.",
        kind = "table"
    ),
    parameter(name = "pretty", info = "Pretty serialization.", kind = "boolean"),
    result(name = "data", info = "Table as a string.", kind = "string")
)]
fn into_string(_: &mlua::Lua, (data, pretty): (mlua::Value, bool)) -> mlua::Result<String> {
    let string = if pretty {
        serde_json::to_string_pretty(&data)
    } else {
        serde_json::to_string(&data)
    };

    match string {
        Ok(value) => Ok(value),
        Err(error) => Err(mlua::Error::runtime(error.to_string())),
    }
}

#[function(
    from = "data",
    info = "Deserialize a string as a Lua table.",
    parameter(
        name = "data",
        info = "String to deserialize as a table.",
        kind = "string"
    ),
    result(name = "data", info = "String as a table.", kind = "table")
)]
fn from_string(lua: &mlua::Lua, data: String) -> mlua::Result<mlua::Value> {
    match serde_json::from_str::<Value>(&data) {
        Ok(value) => lua.to_value(&value),
        Err(error) => Err(mlua::Error::runtime(error.to_string())),
    }
}

#[function(
    from = "data",
    info = "Get the current OS kind.",
    result(
        name = "system",
        info = "System kind.",
        kind(user_data(name = "SystemKind"))
    )
)]
#[rustfmt::skip]
fn get_system(_: &mlua::Lua, _: ()) -> mlua::Result<usize> {
    match std::env::consts::OS {
        "linux"   => Ok(0),
        "windows" => Ok(1),
        "mac"     => Ok(2),
        "android" => Ok(3),
        "ios"     => Ok(4),
        _         => Ok(5),
    }
}
