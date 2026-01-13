use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::{ffi::CString, path::Path};

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

pub fn c_string(text: &str) -> CString {
    CString::new(text).unwrap()
}
