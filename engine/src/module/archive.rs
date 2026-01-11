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
use std::io::Read;

//================================================================

#[rustfmt::skip]
#[module(name = "archive", info = "Archive API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let archive = lua.create_table()?;

    archive.set("new", lua.create_function(self::Archive::new)?)?;

    global.set("archive", archive)?;

    Ok(())
}

//================================================================

#[class(info = "Archive class.")]
pub struct Archive {
    inner: zip::ZipArchive<std::fs::File>,
}

impl Archive {
    pub fn borrow_file(path: &str, user: mlua::AnyUserData) -> mlua::Result<(Vec<u8>, String)> {
        if let Ok(mut archive) = user.borrow_mut::<Self>() {
            let token: Vec<&str> = path.split(".").collect();
            let token = format!(".{}", token.get(1).unwrap());
            let mut find = archive.inner.by_path(path).unwrap();
            let mut file = Vec::new();
            find.read_to_end(&mut file)?;

            Ok((file, token))
        } else {
            Err(mlua::Error::runtime(
                "Archive argument for function is not of type Archive.",
            ))
        }
    }

    #[function(
        from = "archive",
        info = "Create a new Archive resource.",
        parameter(name = "path", info = "Path to archive.", kind = "string"),
        result(
            name = "archive",
            info = "Archive resource.",
            kind(user_data(name = "Archive"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        let inner = std::fs::File::open(path)?;
        let inner = zip::ZipArchive::new(inner).unwrap();

        Ok(Self { inner })
    }

    #[method(
        from = "Archive",
        info = "Get a full list of every file in the archive.",
        result(
            name = "file_list",
            info = "Table array of every file in the archive.",
            kind = "table"
        )
    )]
    fn get_list(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<Vec<String>> {
        Ok(this.inner.file_names().map(|x| x.to_string()).collect())
    }

    #[method(
        from = "Archive",
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
    fn get_file(
        lua: &mlua::Lua,
        this: &mut Self,
        (path, binary): (String, bool),
    ) -> mlua::Result<mlua::Value> {
        let mut find = this.inner.by_path(path).unwrap();
        let mut file = Vec::new();
        find.read_to_end(&mut file)?;

        if binary {
            Ok(lua.to_value(&file)?)
        } else {
            Ok(lua.to_value(&String::from_utf8(file).unwrap())?)
        }
    }

    #[method(
        from = "Archive",
        info = "Check the kind of a path.",
        parameter(name = "path", info = "Path.", kind = "string"),
        result(
            name = "kind",
            info = "Path kind.",
            kind(user_data(name = "PathKind")),
            optional = true
        )
    )]
    fn get_kind(_: &mlua::Lua, this: &mut Self, path: String) -> mlua::Result<Option<usize>> {
        let find = this.inner.by_path(path);

        if let Ok(find) = find {
            if find.is_file() {
                return Ok(Some(0));
            } else if find.is_dir() {
                return Ok(Some(1));
            } else if find.is_symlink() {
                return Ok(Some(2));
            }
        }

        Ok(None)
    }
}

impl mlua::UserData for Archive {
    #[rustfmt::skip]
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("get_list",     Self::get_list);
        method.add_method_mut("get_file", Self::get_file);
        method.add_method_mut("get_kind", Self::get_kind);
    }
}
