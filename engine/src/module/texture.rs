use crate::module::archive::*;
use crate::module::general::*;
use engine_macro::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;
use std::mem::MaybeUninit;

//================================================================

#[rustfmt::skip]
#[module(name = "texture", info = "Texture API.")]
#[module(name = "texture_target", info = "Texture (render-target) API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let texture        = lua.create_table()?;
    let texture_target = lua.create_table()?;

    texture.set("new",         lua.create_function(self::Texture::new)?)?;
    texture.set("new_archive", lua.create_function(self::Texture::new_archive)?)?;
    texture.set("draw_batch",  lua.create_function(self::Texture::draw_batch)?)?;

    texture_target.set("new",  lua.create_function(self::TextureTarget::new)?)?;

    global.set("texture",        texture)?;
    global.set("texture_target", texture_target)?;

    Ok(())
}

//================================================================

#[repr(C)]
#[derive(Debug)]
struct TextureBatch {
    // Texture ID.
    identifier: u32,
    // Z-index.
    index: u32,
    // Source Box2 point + scale.
    s_p_x: f32,
    s_p_y: f32,
    s_s_x: f32,
    s_s_y: f32,
    // Target Box2 point + scale.
    t_p_x: f32,
    t_p_y: f32,
    t_s_x: f32,
    t_s_y: f32,
    // Point.
    p_x: f32,
    p_y: f32,
    // Scale.
    s_x: i32,
    s_y: i32,
    // Angle.
    angle: f32,
    // Color.
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[class(info = "Texture class.")]
struct Texture {
    inner: ffi::Texture2D,
}

impl Texture {
    #[function(
        from = "texture",
        info = "Create a new texture resource.",
        parameter(name = "path", info = "Path to texture.", kind = "string"),
        result(
            name = "texture",
            info = "Texture resource.",
            kind(user_data(name = "Texture"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        unsafe {
            let inner = ffi::LoadTexture(c_string(&path)?.as_ptr());

            if ffi::IsTextureValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::external(format!(
                    "texture.new(): Error loading texture \"{path}\"."
                )))
            }
        }
    }

    #[function(
        from = "texture",
        info = "Create a new Texture resource from an archive.",
        parameter(name = "path", info = "Path to texture.", kind = "string"),
        parameter(
            name = "archive",
            info = "Archive to load the asset from.",
            kind(user_data(name = "Archive"))
        ),
        result(
            name = "texture",
            info = "Texture resource.",
            kind(user_data(name = "Texture"))
        )
    )]
    fn new_archive(
        _: &mlua::Lua,
        (path, archive): (String, mlua::AnyUserData),
    ) -> mlua::Result<Self> {
        let (data, extension) = Archive::borrow_file(&path, archive)?;

        unsafe {
            let inner = ffi::LoadImageFromMemory(
                c_string(&extension)?.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
            );
            let inner = ffi::LoadTextureFromImage(inner);

            if ffi::IsTextureValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::external(format!(
                    "texture.new_archive(): Error loading texture \"{path}\"."
                )))
            }
        }
    }

    #[function(
        from = "texture",
        info = "Perform a batch draw.",
        parameter(
            name = "buffer",
            info = "Texture batch buffer pointer.",
            kind(user_data(name = "cdata"))
        ),
        parameter(name = "length", info = "Texture batch length.", kind = "number")
    )]
    #[rustfmt::skip]
    fn draw_batch(_: &mlua::Lua, (buffer, length): (mlua::Value, usize)) -> mlua::Result<()> {
        let buffer = buffer.to_pointer();
        let buffer =
            unsafe { std::slice::from_raw_parts_mut(buffer as *mut TextureBatch, length) };

        let mut texture: ffi::Texture2D = unsafe { MaybeUninit::zeroed().assume_init() };

        buffer.sort_by(|a, b| {
            if a.index != b.index {
                a.index.cmp(&b.index)
            } else {
                a.identifier.cmp(&b.identifier)
            }
        });

        for entry in buffer {
            unsafe {
                texture.id     = entry.identifier;
                texture.width  = entry.s_x;
                texture.height = entry.s_y;

                ffi::DrawTexturePro(
                    texture,
                    ffi::Rectangle {
                        x:      entry.s_p_x,
                        y:      entry.s_p_y,
                        width:  entry.s_s_x,
                        height: entry.s_s_y,
                    },
                    ffi::Rectangle {
                        x:      entry.t_p_x,
                        y:      entry.t_p_y,
                        width:  entry.t_s_x,
                        height: entry.t_s_y,
                    },
                    ffi::Vector2 {
                        x: entry.p_x,
                        y: entry.p_y,
                    },
                    entry.angle,
                    ffi::Color {
                        r: entry.r,
                        g: entry.g,
                        b: entry.b,
                        a: entry.a,
                    },
                );
            }
        }

        Ok(())
    }

    #[method(
        from = "Texture",
        info = "Draw texture.",
        parameter(name = "source", info = "Source of texture to draw.", kind = "Box2"),
        parameter(name = "target", info = "Target of texture to draw.", kind = "Box2"),
        parameter(name = "point", info = "Point of texture to draw.", kind = "Vector2"),
        parameter(name = "angle", info = "Angle of texture to draw.", kind = "number"),
        parameter(name = "color", info = "Color of texture to draw.", kind = "Color")
    )]
    fn draw(
        lua: &mlua::Lua,
        this: &Self,
        (source, target, point, angle, color): (
            mlua::Value,
            mlua::Value,
            mlua::Value,
            f32,
            mlua::Value,
        ),
    ) -> mlua::Result<()> {
        unsafe {
            let source: Box2 = lua.from_value(source)?;
            let target: Box2 = lua.from_value(target)?;
            let point: Vector2 = lua.from_value(point)?;
            let color: Color = lua.from_value(color)?;

            ffi::DrawTexturePro(
                this.inner,
                source.into(),
                target.into(),
                point.into(),
                angle,
                color.into(),
            );

            Ok(())
        }
    }

    #[method(
        from = "Texture",
        info = "Get texture identifier.",
        result(name = "identifier", info = "Texture identifier.", kind = "number")
    )]
    fn get_identifier(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<u32> {
        Ok(this.inner.id)
    }

    #[method(
        from = "Texture",
        info = "Get texture scale.",
        result(name = "scale", info = "Texture scale.", kind = "Vector2")
    )]
    fn get_scale(lua: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<mlua::Value> {
        lua.to_value(&Vector2::new(
            this.inner.width as f32,
            this.inner.height as f32,
        ))
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadTexture(self.inner);
        }
    }
}

impl mlua::UserData for Texture {
    #[rustfmt::skip]
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("draw",            Self::draw);
        method.add_method("get_identifier",  Self::get_identifier);
        method.add_method("get_scale",       Self::get_scale);
    }
}

//================================================================

#[class(info = "Texture (render-target) class.")]
struct TextureTarget {
    inner: ffi::RenderTexture,
}

impl TextureTarget {
    #[function(
        from = "texture_target",
        info = "Create a new render-target texture resource.",
        parameter(
            name = "scale",
            info = "Render-target texture scale.",
            kind = "Vector2"
        ),
        result(
            name = "texture_target",
            info = "Render-target texture resource.",
            kind(user_data(name = "TextureTarget"))
        )
    )]
    fn new(lua: &mlua::Lua, scale: mlua::Value) -> mlua::Result<Self> {
        unsafe {
            let scale: Vector2 = lua.from_value(scale)?;
            let inner = ffi::LoadRenderTexture(scale.x as i32, scale.y as i32);

            if ffi::IsRenderTextureValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::external(
                    "texture_target.new(): Error loading render-target texture.",
                ))
            }
        }
    }

    #[method(
        from = "TextureTarget",
        info = "Initialize a draw session.",
        parameter(name = "call", info = "Draw function.", kind = "function")
    )]
    fn begin(_: &mlua::Lua, this: &Self, call: mlua::Function) -> mlua::Result<()> {
        unsafe {
            ffi::BeginTextureMode(this.inner);
            let call = call.call::<()>(());
            ffi::EndTextureMode();

            call
        }
    }

    #[method(
        from = "TextureTarget",
        info = "Draw texture.",
        parameter(name = "source", info = "Source of texture to draw.", kind = "Box2"),
        parameter(name = "target", info = "Target of texture to draw.", kind = "Box2"),
        parameter(name = "point", info = "Point of texture to draw.", kind = "Vector2"),
        parameter(name = "angle", info = "Angle of texture to draw.", kind = "number"),
        parameter(name = "color", info = "Color of texture to draw.", kind = "Color")
    )]
    fn draw(
        lua: &mlua::Lua,
        this: &Self,
        (source, target, point, angle, color): (
            mlua::Value,
            mlua::Value,
            mlua::Value,
            f32,
            mlua::Value,
        ),
    ) -> mlua::Result<()> {
        unsafe {
            let mut source: Box2 = lua.from_value(source)?;
            let target: Box2 = lua.from_value(target)?;
            let point: Vector2 = lua.from_value(point)?;
            let color: Color = lua.from_value(color)?;

            source.s_y = -source.s_y;

            ffi::DrawTexturePro(
                this.inner.texture,
                source.into(),
                target.into(),
                point.into(),
                angle,
                color.into(),
            );

            Ok(())
        }
    }

    #[method(
        from = "TextureTarget",
        info = "Get texture scale.",
        result(name = "scale", info = "Texture scale.", kind = "Vector2")
    )]
    fn get_scale(lua: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<mlua::Value> {
        lua.to_value(&Vector2::new(
            this.inner.texture.width as f32,
            this.inner.texture.height as f32,
        ))
    }
}

impl Drop for TextureTarget {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadRenderTexture(self.inner);
        }
    }
}

impl mlua::UserData for TextureTarget {
    #[rustfmt::skip]
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("begin",     Self::begin);
        method.add_method("draw",      Self::draw);
        method.add_method("get_scale", Self::get_scale);
    }
}
