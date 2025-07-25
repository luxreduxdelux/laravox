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

/// A 2-D vector.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vec2 {
    /// 'X' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub x: f32,
    /// 'Y' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub y: f32,
}

impl Vec2 {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::scalar)?;
        module.function_meta(Self::x)?;
        module.function_meta(Self::y)?;
        module.function_meta(Self::zero)?;
        module.function_meta(Self::one)?;
        module.function_meta(Self::dot)?;
        module.function_meta(Self::format)?;
        module.function_meta(Self::format_debug)?;
        module.function_meta(Self::add)?;
        module.function_meta(Self::add_assign)?;
        module.function_meta(Self::sub)?;
        module.function_meta(Self::sub_assign)?;
        module.function_meta(Self::mul)?;
        module.function_meta(Self::mul_assign)?;

        Ok(())
    }

    pub fn rust_new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    //================================================================

    /// Create a new 2-D vector.
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Create a new 2-D vector, using a scalar value for both X and Y.
    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self { x: value, y: value }
    }

    /// Create a new 2-D vector, using a scalar value for X.
    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self { x, y: 0.0 }
    }

    /// Create a new 2-D vector, using a scalar value for Y.
    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self { x: 0.0, y }
    }

    /// Create a new 2-D vector, with every component set to zero.
    #[rune::function(path = Self::zero)]
    fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Create a new 2-D vector, with every component set to one.
    #[rune::function(path = Self::one)]
    fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// Calculate the dot product with another vector.
    #[rune::function]
    fn dot(&mut self, other: &Self) -> f32 {
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
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    #[rune::function(protocol = ADD_ASSIGN)]
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    #[rune::function(protocol = SUB)]
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    #[rune::function(protocol = SUB_ASSIGN)]
    fn sub_assign(&mut self, other: &Self) {
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

/// A 3-D vector.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vec3 {
    /// 'X' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub x: f32,
    /// 'Y' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub y: f32,
    /// 'Z' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub z: f32,
}

impl Vec3 {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        // TO-DO add cross product operation.

        module.function_meta(Self::new)?;
        module.function_meta(Self::scalar)?;
        module.function_meta(Self::x)?;
        module.function_meta(Self::y)?;
        module.function_meta(Self::z)?;
        module.function_meta(Self::zero)?;
        module.function_meta(Self::one)?;
        module.function_meta(Self::dot)?;
        module.function_meta(Self::format)?;
        module.function_meta(Self::format_debug)?;
        module.function_meta(Self::add)?;
        module.function_meta(Self::add_assign)?;
        module.function_meta(Self::sub)?;
        module.function_meta(Self::sub_assign)?;
        module.function_meta(Self::mul)?;
        module.function_meta(Self::mul_assign)?;

        Ok(())
    }

    pub fn rust_new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    //================================================================

    /// Create a new 3-D vector.
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a new 3-D vector, using a scalar value for X, Y and Z.
    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    /// Create a new 3-D vector, using a scalar value for X.
    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self { x, y: 0.0, z: 0.0 }
    }

    /// Create a new 3-D vector, using a scalar value for Y.
    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self { x: 0.0, y, z: 0.0 }
    }

    /// Create a new 3-D vector, using a scalar value for Z.
    #[rune::function(path = Self::z)]
    fn z(z: f32) -> Self {
        Self { x: 0.0, y: 0.0, z }
    }

    /// Create a new 3-D vector, with every component set to zero.
    #[rune::function(path = Self::zero)]
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// Create a new 3-D vector, with every component set to one.
    #[rune::function(path = Self::one)]
    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    /// Calculate the dot product with another vector.
    #[rune::function]
    fn dot(&mut self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
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
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    #[rune::function(protocol = ADD_ASSIGN)]
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    #[rune::function(protocol = SUB)]
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    #[rune::function(protocol = SUB_ASSIGN)]
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }

    #[rune::function(protocol = MUL)]
    fn mul(&mut self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }

    #[rune::function(protocol = MUL_ASSIGN)]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

//================================================================

/// A 4-D vector.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vec4 {
    /// 'X' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub x: f32,
    /// 'Y' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub y: f32,
    /// 'Z' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub z: f32,
    /// 'W' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub w: f32,
}

impl Vec4 {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::scalar)?;
        module.function_meta(Self::x)?;
        module.function_meta(Self::y)?;
        module.function_meta(Self::z)?;
        module.function_meta(Self::w)?;
        module.function_meta(Self::zero)?;
        module.function_meta(Self::one)?;
        module.function_meta(Self::dot)?;
        module.function_meta(Self::format)?;
        module.function_meta(Self::format_debug)?;
        module.function_meta(Self::add)?;
        module.function_meta(Self::add_assign)?;
        module.function_meta(Self::sub)?;
        module.function_meta(Self::sub_assign)?;
        module.function_meta(Self::mul)?;
        module.function_meta(Self::mul_assign)?;

        Ok(())
    }

    pub fn rust_new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    //================================================================

    /// Create a new 4-D vector.
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Create a new 4-D vector, using a scalar value for X, Y, Z and W.
    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    /// Create a new 4-D vector, using a scalar value for X.
    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self {
            x,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new 4-D vector, using a scalar value for Y.
    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self {
            x: 0.0,
            y,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new 4-D vector, using a scalar value for Z.
    #[rune::function(path = Self::z)]
    fn z(z: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z,
            w: 0.0,
        }
    }

    /// Create a new 4-D vector, using a scalar value for W.
    #[rune::function(path = Self::w)]
    fn w(w: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w,
        }
    }

    /// Create a new 4-D vector, with every component set to zero.
    #[rune::function(path = Self::zero)]
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    /// Create a new 4-D vector, with every component set to one.
    #[rune::function(path = Self::one)]
    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    /// Calculate the dot product with another vector.
    #[rune::function]
    fn dot(&mut self, other: &Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
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
    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    #[rune::function(protocol = ADD_ASSIGN)]
    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }

    #[rune::function(protocol = SUB)]
    fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }

    #[rune::function(protocol = SUB_ASSIGN)]
    fn sub_assign(&mut self, other: &Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }

    #[rune::function(protocol = MUL)]
    fn mul(&mut self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }

    #[rune::function(protocol = MUL_ASSIGN)]
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

//================================================================

/// A 2-D box, with point, scale, and angle.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Box2 {
    /// Point of the box.
    #[rune(get, set, copy)]
    pub point: Vec2,
    /// Scale of the box.
    #[rune(get, set, copy)]
    pub scale: Vec2,
    /// Angle of the box.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub angle: f32,
}

impl Box2 {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::format)?;
        module.function_meta(Self::format_debug)?;

        Ok(())
    }

    pub fn rust_new(point: &Vec2, scale: &Vec2, angle: f32) -> Self {
        Self {
            point: *point,
            scale: *scale,
            angle,
        }
    }

    //================================================================

    /// Create a new 2-D box.
    #[rune::function(path = Self::new)]
    fn new(point: &Vec2, scale: &Vec2, angle: f32) -> Self {
        Self {
            point: *point,
            scale: *scale,
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

/// A color.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Color {
    /// 'R' channel.
    #[rune(get, set)]
    pub r: u8,
    /// 'G' channel.
    #[rune(get, set)]
    pub g: u8,
    /// 'B' channel.
    #[rune(get, set)]
    pub b: u8,
    /// 'A' channel.
    #[rune(get, set)]
    pub a: u8,
}

impl Color {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::r)?;
        module.function_meta(Self::g)?;
        module.function_meta(Self::b)?;
        module.function_meta(Self::white)?;
        module.function_meta(Self::black)?;
        module.function_meta(Self::format)?;
        module.function_meta(Self::format_debug)?;

        Ok(())
    }

    pub fn rust_new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    //================================================================

    // Create a new color.
    #[rune::function(path = Self::new)]
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    // Create a new color, using the constant color red.
    #[rune::function(path = Self::r)]
    fn r() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    // Create a new color, using the constant color green.
    #[rune::function(path = Self::g)]
    fn g() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    // Create a new color, using the constant color blue.
    #[rune::function(path = Self::b)]
    fn b() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    // Create a new color, using the constant color white.
    #[rune::function(path = Self::white)]
    fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    // Create a new color, using the constant color black.
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

    Vec2::module(&mut module)?;
    Vec3::module(&mut module)?;
    Vec4::module(&mut module)?;
    Box2::module(&mut module)?;
    Color::module(&mut module)?;

    module.function("sin", |number: f64| number.sin()).build()?;
    module.function("cos", |number: f64| number.cos()).build()?;

    Ok(module)
}
