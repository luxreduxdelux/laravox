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
use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "window", info = "Window API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let window = lua.create_table()?;

    window.set("get_exit",            lua.create_function(self::get_exit)?)?;
    window.set("get_full_screen",     lua.create_function(self::get_full_screen)?)?;
    window.set("toggle_full_screen",  lua.create_function(self::toggle_full_screen)?)?;
    window.set("set_exit_key",        lua.create_function(self::set_exit_key)?)?;
    window.set("get_screen_scale",    lua.create_function(self::get_screen_scale)?)?;
    window.set("get_window_scale",    lua.create_function(self::get_window_scale)?)?;
    window.set("set_window_scale",    lua.create_function(self::set_window_scale)?)?;
    window.set("get_render_scale",    lua.create_function(self::get_render_scale)?)?;
    window.set("set_frame_rate",      lua.create_function(self::set_frame_rate)?)?;
    window.set("get_frame_time",      lua.create_function(self::get_frame_time)?)?;
    window.set("get_time",            lua.create_function(self::get_time)?)?;
    window.set("get_frame_rate",      lua.create_function(self::get_frame_rate)?)?;
    window.set("get_focus",           lua.create_function(self::get_focus)?)?;
    window.set("get_resize",          lua.create_function(self::get_resize)?)?;

    global.set("window", window)?;

    Ok(())
}

//================================================================

#[function(
    from = "window",
    info = "Get the exit state of the window.",
    result(name = "state", info = "Exit state.", kind = "boolean")
)]
fn get_exit(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::WindowShouldClose()) }
}

#[function(
    from = "window",
    info = "Get the full-screen state of the window.",
    result(name = "state", info = "Full-screen state.", kind = "boolean")
)]
fn get_full_screen(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFullscreen()) }
}

#[function(from = "window", info = "Toggle between full-screen and window mode.")]
fn toggle_full_screen(_: &mlua::Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::ToggleFullscreen();
        Ok(())
    }
}

#[function(
    from = "window",
    info = "Set the exit key.",
    parameter(
        name = "code",
        info = "Exit key. If nil, the exit key will be made null.",
        kind = "number"
    )
)]
fn set_exit_key(_: &mlua::Lua, code: Option<i32>) -> mlua::Result<()> {
    unsafe {
        if let Some(code) = code {
            ffi::SetExitKey(code);
        } else {
            ffi::SetExitKey(0);
        }
        Ok(())
    }
}

#[function(
    from = "window",
    info = "Get the current screen scale.",
    result(name = "scale", info = "Screen scale.", kind = "vector_2")
)]
fn get_screen_scale(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
    unsafe {
        let current = ffi::GetCurrentMonitor();
        lua.to_value(&Vector2::new(
            ffi::GetMonitorWidth(current) as f32,
            ffi::GetMonitorHeight(current) as f32,
        ))
    }
}

#[function(
    from = "window",
    info = "Get the current window scale.",
    result(name = "scale", info = "Window scale.", kind = "vector_2")
)]
fn get_window_scale(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
    unsafe {
        lua.to_value(&Vector2::new(
            ffi::GetScreenWidth() as f32,
            ffi::GetScreenHeight() as f32,
        ))
    }
}

#[function(
    from = "window",
    info = "Set the current window scale.",
    parameter(name = "scale", info = "Window scale.", kind = "vector_2")
)]
fn set_window_scale(lua: &mlua::Lua, scale: mlua::Value) -> mlua::Result<()> {
    unsafe {
        let value: Vector2 = lua.from_value(scale)?;
        ffi::SetWindowSize(value.x as i32, value.y as i32);
        Ok(())
    }
}

#[function(
    from = "window",
    info = "Get the current render scale.",
    result(name = "scale", info = "Render state.", kind = "vector_2")
)]
fn get_render_scale(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
    unsafe {
        lua.to_value(&Vector2::new(
            ffi::GetRenderWidth() as f32,
            ffi::GetRenderHeight() as f32,
        ))
    }
}

#[function(
    from = "window",
    info = "Set the frame rate.",
    parameter(name = "rate", info = "Frame rate.", kind = "number")
)]
fn set_frame_rate(_: &mlua::Lua, rate: i32) -> mlua::Result<()> {
    unsafe {
        ffi::SetTargetFPS(rate);
        Ok(())
    }
}

#[function(
    from = "window",
    info = "Get the frame time.",
    result(name = "time", info = "Frame time.", kind = "number")
)]
fn get_frame_time(_: &mlua::Lua, _: ()) -> mlua::Result<f32> {
    unsafe { Ok(ffi::GetFrameTime()) }
}

#[function(
    from = "window",
    info = "Get the current time.",
    result(name = "time", info = "Current time.", kind = "number")
)]
fn get_time(_: &mlua::Lua, _: ()) -> mlua::Result<f64> {
    unsafe { Ok(ffi::GetTime()) }
}

#[function(
    from = "window",
    info = "Get the frame rate.",
    result(name = "rate", info = "Target frame rate.", kind = "number")
)]
fn get_frame_rate(_: &mlua::Lua, _: ()) -> mlua::Result<i32> {
    unsafe { Ok(ffi::GetFPS()) }
}

#[function(
    from = "window",
    info = "Get the focus state.",
    result(name = "state", info = "Focus state.", kind = "boolean")
)]
fn get_focus(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowFocused()) }
}

#[function(
    from = "window",
    info = "Check if the window size is different from the previous frame.",
    result(
        name = "resize",
        info = "True if the window size is different.",
        kind = "boolean"
    )
)]
fn get_resize(_: &mlua::Lua, _: ()) -> mlua::Result<bool> {
    unsafe { Ok(ffi::IsWindowResized()) }
}
