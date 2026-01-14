use crate::module::general::*;
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

            if let Some(extension) = token.get(1) {
                let extension = format!(".{}", extension);
                let mut find = map_error(archive.inner.by_path(path))?;
                let mut file = Vec::new();
                find.read_to_end(&mut file)?;

                Ok((file, extension))
            } else {
                Err(mlua::Error::external(format!(
                    "Missing extension for path \"{path}\"."
                )))
            }
        } else {
            Err(mlua::Error::external(
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
        let inner = map_error(zip::ZipArchive::new(inner))?;

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
        let mut find = map_error(this.inner.by_path(path))?;

        if binary {
            let mut file = Vec::new();
            find.read_to_end(&mut file)?;

            Ok(lua.to_value(&file)?)
        } else {
            let mut file = String::new();
            find.read_to_string(&mut file)?;

            Ok(lua.to_value(&file)?)
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
