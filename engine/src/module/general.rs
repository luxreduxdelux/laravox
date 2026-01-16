use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::CString;

//================================================================

#[derive(Serialize, Deserialize)]
pub struct Box2 {
    pub p_x: f32,
    pub p_y: f32,
    pub s_x: f32,
    pub s_y: f32,
}

impl From<Box2> for ffi::Rectangle {
    fn from(value: Box2) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

//================================================================

#[derive(Serialize, Deserialize)]
pub struct Camera2D {
    pub point: Vector2,
    pub shift: Vector2,
    pub angle: f32,
    pub zoom: f32,
}

impl From<Camera2D> for ffi::Camera2D {
    fn from(value: Camera2D) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

//================================================================

pub fn c_string(text: &str) -> mlua::Result<CString> {
    let convert = CString::new(text);

    if let Ok(convert) = convert {
        Ok(convert)
    } else {
        Err(mlua::Error::external(format!(
            "Error converting Rust string to C string \"{text}\"."
        )))
    }
}

pub fn map_error<T, E>(result: std::result::Result<T, E>) -> mlua::Result<T>
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    match result {
        Ok(value) => Ok(value),
        Err(error) => Err(mlua::Error::ExternalError(error.into().into())),
    }
}

pub fn sub_string(_: &mlua::Lua, (value, index_a, index_b): (String, isize, Option<isize>)) -> mlua::Result<String> {
    let character: Vec<char> = value.chars().collect();
    let length = character.len() as isize;

    if length == 0 {
        return Ok(String::new());
    }

    let index_b = index_b.unwrap_or(-1);

    let mut a = if index_a < 0 { length + index_a } else { index_a - 1 };
    let mut b = if index_b < 0 { length + index_b } else { index_b - 1 };

    a = a.max(0);
    b = b.min(length - 1);

    if a > b || a >= length {
        return Ok(String::new());
    }

    Ok(character[a as usize..=b as usize]
        .iter()
        .collect())
}
