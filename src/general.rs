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

use rune::{
    Any, Module,
    alloc::{clone::TryClone, fmt::TryWrite},
    runtime::{Formatter, VmResult},
    vm_write,
};
use three_d::Srgba;

//================================================================

#[derive(Any, TryClone, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vector2 {
    #[rune(get, set)]
    pub x: f32,
    #[rune(get, set)]
    pub y: f32,
}

impl Vector2 {
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self { x: value, y: value }
    }

    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self { x, y: 0.0 }
    }

    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self { x: 0.0, y }
    }

    #[rune::function(path = Self::dot)]
    fn dot(&mut self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    #[rune::function(protocol = DISPLAY_FMT)]
    fn format(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }

    #[rune::function(protocol = DEBUG_FMT)]
    fn format_debug(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }

    #[rune::function(protocol = ADD)]
    fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[rune::function(protocol = ADD_ASSIGN)]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }

    #[rune::function(protocol = SUB)]
    fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[rune::function(protocol = SUB_ASSIGN)]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }

    #[rune::function(protocol = MUL)]
    fn mul(&mut self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }

    #[rune::function(protocol = MUL_ASSIGN)]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
    }
}

//================================================================

#[derive(Any, TryClone, Clone, Debug)]
#[rune(item = ::general)]
pub struct Box2 {
    #[rune(get, set)]
    pub point: Vector2,
    #[rune(get, set)]
    pub scale: Vector2,
    #[rune(get, set)]
    pub angle: f32,
}

impl Box2 {
    #[rune::function(path = Self::new)]
    fn new(point: Vector2, scale: Vector2, angle: f32) -> Self {
        Self {
            point,
            scale,
            angle,
        }
    }

    #[rune::function(protocol = DISPLAY_FMT)]
    fn format(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }

    #[rune::function(protocol = DEBUG_FMT)]
    fn format_debug(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }
}

//================================================================

#[derive(Any, TryClone, Clone, Debug)]
#[rune(item = ::general)]
pub struct Color {
    #[rune(get, set)]
    pub r: u8,
    #[rune(get, set)]
    pub g: u8,
    #[rune(get, set)]
    pub b: u8,
    #[rune(get, set)]
    pub a: u8,
}

impl Color {
    #[rune::function(path = Self::new)]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[rune::function(path = Self::white)]
    fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    #[rune::function(path = Self::black)]
    fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    #[rune::function(protocol = DISPLAY_FMT)]
    fn format(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }

    #[rune::function(protocol = DEBUG_FMT)]
    fn format_debug(&self, formatter: &mut Formatter) -> VmResult<()> {
        vm_write!(formatter, "{:?}", self)
    }
}

impl From<Color> for Srgba {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

//================================================================

#[rune::module(::general)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    module.ty::<Vector2>()?;
    module.function_meta(Vector2::new)?;
    module.function_meta(Vector2::scalar)?;
    module.function_meta(Vector2::x)?;
    module.function_meta(Vector2::y)?;
    module.function_meta(Vector2::dot)?;
    module.function_meta(Vector2::format)?;
    module.function_meta(Vector2::format_debug)?;
    module.function_meta(Vector2::add)?;
    module.function_meta(Vector2::add_assign)?;
    module.function_meta(Vector2::sub)?;
    module.function_meta(Vector2::sub_assign)?;
    module.function_meta(Vector2::mul)?;
    module.function_meta(Vector2::mul_assign)?;

    module.ty::<Box2>()?;
    module.function_meta(Box2::new)?;
    module.function_meta(Box2::format)?;
    module.function_meta(Box2::format_debug)?;

    module.ty::<Color>()?;
    module.function_meta(Color::new)?;
    module.function_meta(Color::format)?;
    module.function_meta(Color::format_debug)?;
    module.function_meta(Color::white)?;
    module.function_meta(Color::black)?;

    Ok(module)
}
