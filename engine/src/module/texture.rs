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

use crate::module::general::*;
use engine_macro::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "texture", info = "Texture API.")]
#[module(name = "texture_target", info = "Texture (render-target) API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let texture        = lua.create_table()?;
    let texture_target = lua.create_table()?;

    texture.set("new",        lua.create_function(self::Texture::new)?)?;
    texture_target.set("new", lua.create_function(self::TextureTarget::new)?)?;

    global.set("texture",        texture)?;
    global.set("texture_target", texture_target)?;

    Ok(())
}

//================================================================

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
            kind(user_data(name = "texture"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        unsafe {
            let inner = ffi::LoadTexture(c_string(&path).as_ptr());

            if ffi::IsTextureValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::runtime(format!(
                    "texture.new(): Error loading texture \"{path}\"."
                )))
            }
        }
    }

    #[method(
        from = "texture",
        info = "Draw texture.",
        parameter(name = "source", info = "Source of texture to draw.", kind = "box_2"),
        parameter(name = "target", info = "Target of texture to draw.", kind = "box_2"),
        parameter(name = "point", info = "Point of texture to draw.", kind = "vector_2"),
        parameter(name = "angle", info = "Angle of texture to draw.", kind = "number"),
        parameter(name = "color", info = "Color of texture to draw.", kind = "color")
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
        from = "texture",
        info = "Get texture scale.",
        result(name = "scale", info = "Texture scale.", kind = "vector_2")
    )]
    fn get_scale(lua: &mlua::Lua, this: &Self) -> mlua::Result<mlua::Value> {
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
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("draw", |lua, this, (text, point, scale, space, color)| {
            Self::draw(lua, this, (text, point, scale, space, color))
        });
        method.add_method("get_scale", |lua, this, _: ()| Self::get_scale(lua, this));
    }
}

//================================================================

#[class(name = "texture_target", info = "Texture (render-target) class.")]
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
            kind = "vector_2"
        ),
        result(
            name = "texture_target",
            info = "Render-target texture resource.",
            kind(user_data(name = "texture_target"))
        )
    )]
    fn new(lua: &mlua::Lua, scale: mlua::Value) -> mlua::Result<Self> {
        unsafe {
            let scale: Vector2 = lua.from_value(scale)?;
            let inner = ffi::LoadRenderTexture(scale.x as i32, scale.y as i32);

            if ffi::IsRenderTextureValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::runtime(
                    "texture_target.new(): Error loading render-target texture.",
                ))
            }
        }
    }

    #[method(
        from = "texture_target",
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
        from = "texture_target",
        info = "Draw texture.",
        parameter(name = "source", info = "Source of texture to draw.", kind = "box_2"),
        parameter(name = "target", info = "Target of texture to draw.", kind = "box_2"),
        parameter(name = "point", info = "Point of texture to draw.", kind = "vector_2"),
        parameter(name = "angle", info = "Angle of texture to draw.", kind = "number"),
        parameter(name = "color", info = "Color of texture to draw.", kind = "color")
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
        from = "texture_target",
        info = "Get texture scale.",
        result(name = "scale", info = "Texture scale.", kind = "vector_2")
    )]
    fn get_scale(lua: &mlua::Lua, this: &Self) -> mlua::Result<mlua::Value> {
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
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("begin", Self::begin);
        method.add_method("draw", |lua, this, (text, point, scale, space, color)| {
            Self::draw(lua, this, (text, point, scale, space, color))
        });
        method.add_method("get_scale", |lua, this, _: ()| Self::get_scale(lua, this));
    }
}
