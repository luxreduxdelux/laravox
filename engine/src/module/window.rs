use engine_macro::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "window", info = "Window API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let window = lua.create_table()?;

    window.set("get_exit",           lua.create_function(self::get_exit)?)?;
    window.set("get_full_screen",    lua.create_function(self::get_full_screen)?)?;
    window.set("toggle_full_screen", lua.create_function(self::toggle_full_screen)?)?;
    window.set("set_exit_key",       lua.create_function(self::set_exit_key)?)?;
    window.set("get_screen_scale",   lua.create_function(self::get_screen_scale)?)?;
    window.set("get_window_scale",   lua.create_function(self::get_window_scale)?)?;
    window.set("set_window_scale",   lua.create_function(self::set_window_scale)?)?;
    window.set("get_render_scale",   lua.create_function(self::get_render_scale)?)?;
    window.set("set_frame_rate",     lua.create_function(self::set_frame_rate)?)?;
    window.set("get_frame_time",     lua.create_function(self::get_frame_time)?)?;
    window.set("get_time",           lua.create_function(self::get_time)?)?;
    window.set("get_frame_rate",     lua.create_function(self::get_frame_rate)?)?;
    window.set("get_focus",          lua.create_function(self::get_focus)?)?;
    window.set("get_resize",         lua.create_function(self::get_resize)?)?;
    window.set("dialog_message",     lua.create_function(self::dialog_message)?)?;
    //window.set("dialog_pick_file",   lua.create_function(self::dialog_pick_file)?)?;
    //window.set("dialog_pick_path",   lua.create_function(self::dialog_pick_path)?)?;

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
        info = "Exit key. If nil, no key will cause the window's exit state to enable.",
        kind = "number",
        optional = true
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
    result(name = "scale", info = "Screen scale.", kind = "Vector2")
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
    result(name = "scale", info = "Window scale.", kind = "Vector2")
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
    parameter(name = "scale", info = "Window scale.", kind = "Vector2")
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
    result(name = "scale", info = "Render state.", kind = "Vector2")
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

#[function(
    from = "window",
    info = "Show a message dialog.",
    parameter(
        name = "kind",
        info = "Message kind.",
        kind(user_data(name = "MessageKind"))
    ),
    parameter(name = "name", info = "Message window name.", kind = "string"),
    parameter(name = "text", info = "Message window text.", kind = "string"),
    result(
        name = "dialog",
        info = "True if the window size is different.",
        kind = "boolean"
    )
)]
fn dialog_message(
    _: &mlua::Lua,
    (kind, name, text): (usize, String, String),
) -> mlua::Result<bool> {
    let kind = match kind {
        0 => rfd::MessageLevel::Info,
        1 => rfd::MessageLevel::Warning,
        _ => rfd::MessageLevel::Error,
    };

    let result = rfd::MessageDialog::new()
        .set_level(kind)
        .set_title(name)
        .set_description(text)
        .set_buttons(rfd::MessageButtons::YesNo)
        .show();

    match result {
        rfd::MessageDialogResult::Yes => Ok(true),
        _ => Ok(false),
    }
}
