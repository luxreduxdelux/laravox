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

    screen.set("wipe",                lua.create_function(self::wipe)?)?;
    screen.set("draw",                lua.create_function(self::draw)?)?;
    screen.set("draw_2D",             lua.create_function(self::draw_2D)?)?;
    screen.set("draw_2D_begin",             lua.create_function(self::draw_2D_begin)?)?;
    screen.set("draw_2D_close",             lua.create_function(self::draw_2D_close)?)?;
    screen.set("draw_scissor",        lua.create_function(self::draw_scissor)?)?;
    screen.set("draw_scissor_begin",  lua.create_function(self::draw_scissor_begin)?)?;
    screen.set("draw_scissor_close",  lua.create_function(self::draw_scissor_close)?)?;
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

// TO-DO temporary
fn draw_2D_begin(lua: &mlua::Lua, camera: mlua::Value) -> mlua::Result<()> {
    unsafe {
        let camera: Camera2D = lua.from_value(camera)?;

        ffi::BeginMode2D(camera.into());

        Ok(())
    }
}

// TO-DO temporary
fn draw_2D_close(_: &mlua::Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::EndMode2D();

        Ok(())
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
    info = "Manually begin a scissor clip draw session. Use `draw_scissor` whenever possible.",
    parameter(name = "area", info = "Draw area.", kind = "Box2")
)]
fn draw_scissor_begin(lua: &mlua::Lua, area: mlua::Value) -> mlua::Result<()> {
    unsafe {
        let area: Box2 = lua.from_value(area)?;

        ffi::BeginScissorMode(
            area.p_x as i32,
            area.p_y as i32,
            area.s_x as i32,
            area.s_y as i32,
        );

        Ok(())
    }
}

#[function(
    from = "screen",
    info = "Manually close a scissor clip draw session. Use `draw_scissor` whenever possible."
)]
fn draw_scissor_close(_: &mlua::Lua, _: ()) -> mlua::Result<()> {
    unsafe {
        ffi::EndScissorMode();

        Ok(())
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
