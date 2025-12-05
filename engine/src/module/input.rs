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
#[module(name = "input.pad", info = "Input (pad) API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let input = lua.create_table()?;
    let board = lua.create_table()?;
    let mouse = lua.create_table()?;
    let pad   = lua.create_table()?;

    //================================================================

    board.set("get_press",          lua.create_function(self::board::get_press)?)?;
    board.set("get_press_repeat",   lua.create_function(self::board::get_press_repeat)?)?;
    board.set("get_down",           lua.create_function(self::board::get_down)?)?;
    board.set("get_release",        lua.create_function(self::board::get_release)?)?;
    board.set("get_up",             lua.create_function(self::board::get_up)?)?;
    board.set("get_last_press",     lua.create_function(self::board::get_last_press)?)?;
    board.set("get_last_character", lua.create_function(self::board::get_last_character)?)?;
    board.set("set_clip_board",     lua.create_function(self::board::set_clip_board)?)?;
    board.set("get_clip_board",     lua.create_function(self::board::get_clip_board)?)?;

    //================================================================

    mouse.set("get_press",      lua.create_function(self::mouse::get_press)?)?;
    mouse.set("get_down",       lua.create_function(self::mouse::get_down)?)?;
    mouse.set("get_release",    lua.create_function(self::mouse::get_release)?)?;
    mouse.set("get_up",         lua.create_function(self::mouse::get_up)?)?;
    mouse.set("get_last_press", lua.create_function(self::mouse::get_last_press)?)?;
    mouse.set("get_point",      lua.create_function(self::mouse::get_point)?)?;
    mouse.set("get_delta",      lua.create_function(self::mouse::get_delta)?)?;
    mouse.set("set_point",      lua.create_function(self::mouse::set_point)?)?;
    mouse.set("get_wheel",      lua.create_function(self::mouse::get_wheel)?)?;
    mouse.set("show_cursor",    lua.create_function(self::mouse::show_cursor)?)?;
    mouse.set("lock_cursor",    lua.create_function(self::mouse::lock_cursor)?)?;

    //================================================================

    pad.set("get_state",      lua.create_function(self::pad::get_state)?)?;
    pad.set("get_name",       lua.create_function(self::pad::get_name)?)?;
    pad.set("get_press",      lua.create_function(self::pad::get_press)?)?;
    pad.set("get_down",       lua.create_function(self::pad::get_down)?)?;
    pad.set("get_release",    lua.create_function(self::pad::get_release)?)?;
    pad.set("get_up",         lua.create_function(self::pad::get_up)?)?;
    pad.set("get_last_press", lua.create_function(self::pad::get_last_press)?)?;
    pad.set("get_axis_count", lua.create_function(self::pad::get_axis_count)?)?;
    pad.set("get_axis_state", lua.create_function(self::pad::get_axis_state)?)?;
    pad.set("set_vibration",  lua.create_function(self::pad::set_vibration)?)?;

    //================================================================

    input.set("board", board)?;
    input.set("mouse", mouse)?;
    input.set("pad", pad)?;
    global.set("input", input)?;

    Ok(())
}

//================================================================

mod board {
    const KEY_LIST: [i32; 109] = [
        39, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 59, 61, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        96, 32, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 280, 281,
        282, 283, 284, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 340, 341, 342,
        343, 344, 345, 346, 347, 348, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331,
        332, 333, 334, 335, 336, 4, 5, 24, 25,
    ];

    use crate::module::general::c_string;

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
        info = "Get the state (press-repeat) of a key.",
        parameter(name = "code", info = "Key code.", kind = "number"),
        result(name = "state", info = "Key state.", kind = "boolean")
    )]
    pub fn get_press_repeat(_: &mlua::Lua, code: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsKeyPressedRepeat(code) })
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

    #[function(
        from = "input.board",
        info = "Get the last key press.",
        result(name = "code", info = "Key code. May be nil.", kind = "number")
    )]
    pub fn get_last_press(_: &mlua::Lua, _: ()) -> mlua::Result<Option<i32>> {
        unsafe {
            // TO-DO this is not the actual range of the entire key list
            for x in self::KEY_LIST {
                if ffi::IsKeyPressed(x) {
                    return Ok(Some(x));
                }
            }

            Ok(None)
        }
    }

    #[function(
        from = "input.board",
        info = "Get the last key character press.",
        result(
            name = "character",
            info = "Key character. May be nil.",
            kind = "number"
        )
    )]
    pub fn get_last_character(_: &mlua::Lua, _: ()) -> mlua::Result<Option<i32>> {
        unsafe {
            let code = ffi::GetCharPressed();

            if code == 0 { Ok(None) } else { Ok(Some(code)) }
        }
    }

    #[function(
        from = "input.board",
        info = "Set the clip-board text.",
        parameter(name = "text", info = "Clip-board text.", kind = "string")
    )]
    pub fn set_clip_board(_: &mlua::Lua, text: String) -> mlua::Result<()> {
        unsafe {
            ffi::SetClipboardText(c_string(&text).as_ptr());
            Ok(())
        }
    }

    #[function(
        from = "input.board",
        info = "Get the clip-board text.",
        result(name = "text", info = "Clip-board text.", kind = "string")
    )]
    pub fn get_clip_board(_: &mlua::Lua, _: ()) -> mlua::Result<String> {
        unsafe {
            let value = std::ffi::CStr::from_ptr(ffi::GetClipboardText());
            Ok(value.to_string_lossy().to_string())
        }
    }
}

//================================================================

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
        info = "Get the last mouse button press.",
        result(
            name = "code",
            info = "Mouse button code. May be nil.",
            kind = "number"
        )
    )]
    pub fn get_last_press(_: &mlua::Lua, _: ()) -> mlua::Result<Option<i32>> {
        unsafe {
            for x in 0..7 {
                if ffi::IsMouseButtonPressed(x) {
                    return Ok(Some(x));
                }
            }

            Ok(None)
        }
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
        info = "Get the delta of the mouse.",
        result(name = "delta", info = "Mouse delta.", kind = "vector_2")
    )]
    pub fn get_delta(lua: &mlua::Lua, _: ()) -> mlua::Result<mlua::Value> {
        Ok(unsafe { lua.to_value(&Vector2::from(ffi::GetMouseDelta()))? })
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

    #[function(
        from = "input.mouse",
        info = "Show, or hide, the mouse cursor.",
        parameter(name = "show", info = "Show/hide mouse cursor.", kind = "boolean")
    )]
    pub fn show_cursor(_: &mlua::Lua, show: bool) -> mlua::Result<()> {
        unsafe {
            if show {
                ffi::ShowCursor();
            } else {
                ffi::HideCursor();
            }
            Ok(())
        }
    }

    #[function(
        from = "input.mouse",
        info = "Lock, or unlock, the mouse cursor.",
        parameter(name = "lock", info = "Lock/unlock mouse cursor.", kind = "boolean")
    )]
    pub fn lock_cursor(_: &mlua::Lua, lock: bool) -> mlua::Result<()> {
        unsafe {
            if lock {
                ffi::DisableCursor();
            } else {
                ffi::EnableCursor();
            }
            Ok(())
        }
    }
}

mod pad {
    use super::*;

    #[function(
        from = "input.pad",
        info = "Get the state of a game-pad.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        result(name = "state", info = "Game-pad state.", kind = "boolean")
    )]
    pub fn get_state(_: &mlua::Lua, index: i32) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsGamepadAvailable(index) })
    }

    #[function(
        from = "input.pad",
        info = "Get the name of a game-pad.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        result(name = "name", info = "Game-pad name.", kind = "string")
    )]
    pub fn get_name(_: &mlua::Lua, index: i32) -> mlua::Result<String> {
        unsafe {
            let value = std::ffi::CStr::from_ptr(ffi::GetGamepadName(index));
            Ok(value.to_string_lossy().to_string())
        }
    }

    #[function(
        from = "input.pad",
        info = "Get the state (press) of a game-pad button.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(name = "code", info = "Game-pad button code.", kind = "number"),
        result(name = "state", info = "Game-pad button state.", kind = "boolean")
    )]
    pub fn get_press(_: &mlua::Lua, (index, code): (i32, i32)) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsGamepadButtonPressed(index, code) })
    }

    #[function(
        from = "input.pad",
        info = "Get the state (down) of a game-pad button.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(name = "code", info = "Game-pad button code.", kind = "number"),
        result(name = "state", info = "Game-pad button state.", kind = "boolean")
    )]
    pub fn get_down(_: &mlua::Lua, (index, code): (i32, i32)) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsGamepadButtonDown(index, code) })
    }

    #[function(
        from = "input.pad",
        info = "Get the state (release) of a game-pad button.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(name = "code", info = "Game-pad button code.", kind = "number"),
        result(name = "state", info = "Game-pad button state.", kind = "boolean")
    )]
    pub fn get_release(_: &mlua::Lua, (index, code): (i32, i32)) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsGamepadButtonReleased(index, code) })
    }

    #[function(
        from = "input.pad",
        info = "Get the state (up) of a game-pad button.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(name = "code", info = "Game-pad button code.", kind = "number"),
        result(name = "state", info = "Game-pad button state.", kind = "boolean")
    )]
    pub fn get_up(_: &mlua::Lua, (index, code): (i32, i32)) -> mlua::Result<bool> {
        Ok(unsafe { ffi::IsGamepadButtonUp(index, code) })
    }

    #[function(
        from = "input.pad",
        info = "Get the last game-pad button press.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        result(
            name = "code",
            info = "Game-pad button code. May be nil.",
            kind = "number"
        )
    )]
    pub fn get_last_press(_: &mlua::Lua, index: i32) -> mlua::Result<Option<i32>> {
        unsafe {
            for x in 1..18 {
                if ffi::IsGamepadButtonPressed(index, x) {
                    return Ok(Some(x));
                }
            }

            Ok(None)
        }
    }

    #[function(
        from = "input.pad",
        info = "Get the axis count of a game-pad.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        result(name = "count", info = "Game-pad axis count.", kind = "number")
    )]
    pub fn get_axis_count(_: &mlua::Lua, index: i32) -> mlua::Result<i32> {
        Ok(unsafe { ffi::GetGamepadAxisCount(index) })
    }

    #[function(
        from = "input.pad",
        info = "Get the axis state of a game-pad.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(name = "axis", info = "Game-pad axis.", kind = "number"),
        result(name = "count", info = "Game-pad axis state.", kind = "number")
    )]
    pub fn get_axis_state(_: &mlua::Lua, (index, axis): (i32, i32)) -> mlua::Result<f32> {
        Ok(unsafe { ffi::GetGamepadAxisMovement(index, axis) })
    }

    #[function(
        from = "input.pad",
        info = "Set a vibration on a game-pad.",
        parameter(name = "index", info = "Game-pad index.", kind = "number"),
        parameter(
            name = "motor_a",
            info = "Game-pad motor (A) vibration scale.",
            kind = "number"
        ),
        parameter(
            name = "motor_b",
            info = "Game-pad motor (B) vibration scale.",
            kind = "number"
        ),
        parameter(name = "time", info = "Vibration duration.", kind = "number")
    )]
    pub fn set_vibration(
        _: &mlua::Lua,
        (index, motor_a, motor_b, time): (i32, f32, f32, f32),
    ) -> mlua::Result<()> {
        unsafe {
            ffi::SetGamepadVibration(index, motor_a, motor_b, time);
            Ok(())
        }
    }
}
