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

use super::general::Box2;
use super::general::Camera2D;
use engine_macro::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "screen", info = "Screen API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let screen = lua.create_table()?;

    // TO-DO any chance of merging draw/draw_2D?
    screen.set("wipe",                lua.create_function(self::wipe)?)?;
    screen.set("draw",                lua.create_function(self::draw)?)?;
    screen.set("draw_2D",             lua.create_function(self::draw_2D)?)?;
    screen.set("draw_scissor",        lua.create_function(self::draw_scissor)?)?;
    screen.set("draw_box_2",          lua.create_function(self::draw_box_2)?)?;
    screen.set("draw_line",           lua.create_function(self::draw_line)?)?;
    screen.set("get_screen_to_world", lua.create_function(self::get_screen_to_world)?)?;
    screen.set("get_world_to_screen", lua.create_function(self::get_world_to_screen)?)?;

    global.set("screen", screen)?;

    Ok(())
}

#[function(
    from = "screen",
    info = "Wipe the frame-buffer.",
    parameter(
        name = "color",
        info = "Color to wipe the frame-buffer with.",
        kind = "Color"
    )
)]
fn wipe(lua: &mlua::Lua, color: mlua::Value) -> mlua::Result<()> {
    unsafe {
        let color: Color = lua.from_value(color)?;

        ffi::ClearBackground(color.into());

        Ok(())
    }
}

#[function(
    from = "screen",
    info = "Initialize a draw session.",
    parameter(name = "call", info = "Draw function.", kind = "function")
)]
fn draw(_: &mlua::Lua, call: mlua::Function) -> mlua::Result<()> {
    unsafe {
        ffi::BeginDrawing();
        let call = call.call::<()>(());
        ffi::EndDrawing();

        call
    }
}

#[allow(non_snake_case)]
#[function(
    from = "screen",
    info = "Initialize a 2D draw session.",
    parameter(name = "call", info = "Draw function.", kind = "function"),
    parameter(name = "camera", info = "2D camera.", kind = "Camera2D")
)]
fn draw_2D(lua: &mlua::Lua, (call, camera): (mlua::Function, mlua::Value)) -> mlua::Result<()> {
    unsafe {
        let camera: Camera2D = lua.from_value(camera)?;

        ffi::BeginMode2D(camera.into());
        let call = call.call::<()>(());
        ffi::EndMode2D();

        call
    }
}

#[function(
    from = "screen",
    info = "Initialize a scissor clip draw session.",
    parameter(name = "call", info = "Draw function.", kind = "function"),
    parameter(name = "area", info = "Draw area.", kind = "Box2")
)]
fn draw_scissor(lua: &mlua::Lua, (call, area): (mlua::Function, mlua::Value)) -> mlua::Result<()> {
    unsafe {
        let area: Box2 = lua.from_value(area)?;

        ffi::BeginScissorMode(
            area.p_x as i32,
            area.p_y as i32,
            area.s_x as i32,
            area.s_y as i32,
        );
        let call = call.call::<()>(());
        ffi::EndScissorMode();

        call
    }
}

#[function(
    from = "screen",
    info = "Draw a 2D box.",
    parameter(name = "box_2", info = "2D box to draw.", kind = "Box2"),
    parameter(
        name = "point",
        info = "Point of the 2D box.",
        kind = "Vector2",
        optional = true
    ),
    parameter(
        name = "angle",
        info = "Angle of the 2D box.",
        kind = "number",
        optional = true
    ),
    parameter(
        name = "color",
        info = "Color of the 2D box.",
        kind = "Color",
        optional = true
    )
)]
fn draw_box_2(
    lua: &mlua::Lua,
    (box_2, point, angle, color): (
        mlua::Value,
        Option<mlua::Value>,
        Option<f32>,
        Option<mlua::Value>,
    ),
) -> mlua::Result<()> {
    unsafe {
        let box_2: Box2 = lua.from_value(box_2)?;
        let point: Vector2 = if let Some(point) = point {
            lua.from_value(point)?
        } else {
            Vector2::default()
        };
        let angle = angle.unwrap_or(0.0);
        let color: Color = if let Some(color) = color {
            lua.from_value(color)?
        } else {
            Color::WHITE
        };

        ffi::DrawRectanglePro(box_2.into(), point.into(), angle, color.into());

        Ok(())
    }
}

#[function(
    from = "screen",
    info = "Draw a 2D line.",
    parameter(name = "source", info = "Source of the 2D line.", kind = "Vector2"),
    parameter(name = "target", info = "Target of the 2D line.", kind = "Vector2"),
    parameter(
        name = "thick",
        info = "Thickness of the 2D line.",
        kind = "number",
        optional = true
    ),
    parameter(
        name = "color",
        info = "Color of the 2D line.",
        kind = "Color",
        optional = true
    )
)]
fn draw_line(
    lua: &mlua::Lua,
    (source, target, thick, color): (mlua::Value, mlua::Value, Option<f32>, Option<mlua::Value>),
) -> mlua::Result<()> {
    unsafe {
        let source: Vector2 = lua.from_value(source)?;
        let target: Vector2 = lua.from_value(target)?;
        let thick = thick.unwrap_or(1.0);
        let color: Color = if let Some(color) = color {
            lua.from_value(color)?
        } else {
            Color::WHITE
        };

        ffi::DrawLineEx(source.into(), target.into(), thick, color.into());

        Ok(())
    }
}

#[function(
    from = "screen",
    info = "Project a world point to a screen point.",
    parameter(name = "point", info = "World point.", kind = "Vector2"),
    parameter(name = "camera", info = "2D camera.", kind = "Camera2D")
)]
fn get_world_to_screen(
    lua: &mlua::Lua,
    (point, camera): (mlua::Value, mlua::Value),
) -> mlua::Result<mlua::Value> {
    unsafe {
        let point: Vector2 = lua.from_value(point)?;
        let camera: Camera2D = lua.from_value(camera)?;

        lua.to_value(&Vector2::from(ffi::GetWorldToScreen2D(
            point.into(),
            camera.into(),
        )))
    }
}

#[function(
    from = "screen",
    info = "Project a screen point to a world point.",
    parameter(name = "point", info = "Screen point.", kind = "Vector2"),
    parameter(name = "camera", info = "2D camera.", kind = "Camera2D")
)]
fn get_screen_to_world(
    lua: &mlua::Lua,
    (point, camera): (mlua::Value, mlua::Value),
) -> mlua::Result<mlua::Value> {
    unsafe {
        let point: Vector2 = lua.from_value(point)?;
        let camera: Camera2D = lua.from_value(camera)?;

        lua.to_value(&Vector2::from(ffi::GetScreenToWorld2D(
            point.into(),
            camera.into(),
        )))
    }
}
