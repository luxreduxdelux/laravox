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
#[module(name = "input", info = "Input API.")]
#[module(name = "input.board", info = "Input (board) API.")]
#[module(name = "input.mouse", info = "Input (mouse) API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let input = lua.create_table()?;
    let board = lua.create_table()?;
    let mouse = lua.create_table()?;

    //================================================================

    board.set("get_press",   lua.create_function(self::board::get_press)?)?;
    board.set("get_down",    lua.create_function(self::board::get_down)?)?;
    board.set("get_release", lua.create_function(self::board::get_release)?)?;
    board.set("get_up",      lua.create_function(self::board::get_up)?)?;

    //================================================================

    mouse.set("get_press",   lua.create_function(self::mouse::get_press)?)?;
    mouse.set("get_down",    lua.create_function(self::mouse::get_down)?)?;
    mouse.set("get_release", lua.create_function(self::mouse::get_release)?)?;
    mouse.set("get_up",      lua.create_function(self::mouse::get_up)?)?;
    mouse.set("get_point",   lua.create_function(self::mouse::get_point)?)?;
    mouse.set("set_point",   lua.create_function(self::mouse::set_point)?)?;
    mouse.set("get_wheel",   lua.create_function(self::mouse::get_wheel)?)?;

    //================================================================

    input.set("board", board)?;
    input.set("mouse", mouse)?;
    global.set("input", input)?;

    Ok(())
}

mod board {
    use super::*;

    #[function(
        from = "input.board",
        info = "Get the state (press) of a key.",
        parameter(name = "code", info = "Key code.", kind = "number"),
        result(name = "state", info = "Key state.", kind = "boolean")
    )]
    pub fn get_press(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsKeyPressed(code) })
    }

    #[function(
        from = "input.board",
        info = "Get the state (down) of a key.",
        parameter(name = "code", info = "Key code.", kind = "number"),
        result(name = "state", info = "Key state.", kind = "boolean")
    )]
    pub fn get_down(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsKeyDown(code) })
    }

    #[function(
        from = "input.board",
        info = "Get the state (release) of a key.",
        parameter(name = "code", info = "Key code.", kind = "number"),
        result(name = "state", info = "Key state.", kind = "boolean")
    )]
    pub fn get_release(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsKeyReleased(code) })
    }

    #[function(
        from = "input.board",
        info = "Get the state (up) of a key.",
        parameter(name = "code", info = "Key code.", kind = "number"),
        result(name = "state", info = "Key state.", kind = "boolean")
    )]
    pub fn get_up(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsKeyUp(code) })
    }
}

mod mouse {
    use super::*;

    #[function(
        from = "input.mouse",
        info = "Get the state (press) of a mouse button.",
        parameter(name = "code", info = "Mouse button code.", kind = "number"),
        result(name = "state", info = "Mouse button state.", kind = "boolean")
    )]
    pub fn get_press(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsMouseButtonPressed(code) })
    }

    #[function(
        from = "input.mouse",
        info = "Get the state (down) of a mouse button.",
        parameter(name = "code", info = "Mouse button code.", kind = "number"),
        result(name = "state", info = "Mouse button state.", kind = "boolean")
    )]
    pub fn get_down(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsMouseButtonDown(code) })
    }

    #[function(
        from = "input.mouse",
        info = "Get the state (release) of a mouse button.",
        parameter(name = "code", info = "Mouse button code.", kind = "number"),
        result(name = "state", info = "Mouse button state.", kind = "boolean")
    )]
    pub fn get_release(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsMouseButtonReleased(code) })
    }

    #[function(
        from = "input.mouse",
        info = "Get the state (press) of a mouse button.",
        parameter(name = "code", info = "Mouse button code.", kind = "number"),
        result(name = "state", info = "Mouse button state.", kind = "boolean")
    )]
    pub fn get_up(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsMouseButtonUp(code) })
    }

    #[function(
        from = "input.mouse",
        info = "Get the point of the mouse cursor on-screen.",
        result(name = "point", info = "Mouse cursor point.", kind = "vector_2")
    )]
    pub fn get_point(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
        Ok(unsafe { lua.to_value(&Vector2::from(ffi::GetMousePosition()))? })
    }

    #[function(
        from = "input.mouse",
        info = "Set the point of the mouse cursor on-screen.",
        parameter(name = "point", info = "Mouse cursor point.", kind = "vector_2")
    )]
    pub fn set_point(lua: &mlua::Lua, point: mlua::Value) -> mlua::Result<()> {
        unsafe {
            let point: Vector2 = lua.from_value(point)?;
            ffi::SetMousePosition(point.x as i32, point.y as i32);
            Ok(())
        }
    }

    #[function(
        from = "input.mouse",
        info = "Get the scroll wheel delta of the mouse.",
        result(name = "delta", info = "Mouse wheel delta.", kind = "vector_2")
    )]
    pub fn get_wheel(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
        Ok(unsafe { lua.to_value(&Vector2::from(ffi::GetMouseWheelMoveV()))? })
    }
}
