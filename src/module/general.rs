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

use koto::{derive::*, prelude::*, runtime, serde::from_koto_value};
use serde::{Deserialize, Serialize};
use three_d::Srgba;

//================================================================

// A 2-D vector.
#[derive(KotoCopy, KotoType, Deserialize, Serialize, Copy, Clone, Debug)]
pub struct Vec2 {
    // 'X' component.
    pub x: f32,
    // 'Y' component.
    pub y: f32,
}

impl KotoObject for Vec2 {
    fn display(&self, ctx: &mut DisplayContext) -> runtime::Result<()> {
        ctx.append(self.type_string());
        Ok(())
    }

    fn negate(&self) -> runtime::Result<KValue> {
        Ok(KValue::Object(Self::rust_new(-self.x, -self.y).into()))
    }

    fn add(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x + other.x, self.y + other.y).into(),
        ))
    }

    fn add_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x + other.x, self.y + other.y).into(),
        ))
    }

    fn subtract(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x - other.x, self.y - other.y).into(),
        ))
    }

    fn subtract_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x - other.x, self.y - other.y).into(),
        ))
    }

    fn multiply(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x * other.x, self.y * other.y).into(),
        ))
    }

    fn multiply_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        Ok(KValue::Object(
            Self::rust_new(self.x * other.x, self.y * other.y).into(),
        ))
    }

    /*
    fn divide(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@/", self.type_string())
    }

    fn divide_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@/", self.type_string())
    }

    fn remainder(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@%", self.type_string())
    }

    fn remainder_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@%", self.type_string())
    }

    fn power(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@^", self.type_string())
    }

    fn power_rhs(&self, other: &KValue) -> runtime::Result<KValue> {
        let _ = other;
        unimplemented_error("@^", self.type_string())
    }
    */

    fn add_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        self.x += other.x;
        self.y += other.y;

        Ok(())
    }

    fn subtract_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        let other: Self = from_koto_value(other.clone()).unwrap();

        self.x -= other.x;
        self.y -= other.y;

        Ok(())
    }

    fn multiply_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        match other {
            KValue::Number(other) => {
                self.x *= f32::from(*other);
                self.y *= f32::from(*other);

                return Ok(());
            }
            _ => {}
        }

        runtime::Result::Err("".into())
    }

    /*
    fn divide_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        let _ = other;
        unimplemented_error("@/=", self.type_string())
    }

    fn remainder_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        let _ = other;
        unimplemented_error("@%=", self.type_string())
    }

    fn power_assign(&mut self, other: &KValue) -> runtime::Result<()> {
        let _ = other;
        unimplemented_error("@^=", self.type_string())
    }

    fn less(&self, other: &KValue) -> runtime::Result<bool> {
        let _ = other;
        unimplemented_error("@<", self.type_string())
    }

    fn less_or_equal(&self, other: &KValue) -> runtime::Result<bool> {
        match self.less(other) {
            Ok(true) => Ok(true),
            Ok(false) => match self.equal(other) {
                Ok(result) => Ok(result),
                Err(error) if error.is_unimplemented_error() => {
                    unimplemented_error("@<=", self.type_string())
                }
                error => error,
            },
            Err(error) if error.is_unimplemented_error() => {
                unimplemented_error("@<=", self.type_string())
            }
            error => error,
        }
    }

    fn greater(&self, other: &KValue) -> runtime::Result<bool> {
        match self.less(other) {
            Ok(true) => Ok(false),
            Ok(false) => match self.equal(other) {
                Ok(result) => Ok(!result),
                Err(error) if error.is_unimplemented_error() => {
                    unimplemented_error("@>", self.type_string())
                }
                error => error,
            },
            Err(error) if error.is_unimplemented_error() => {
                unimplemented_error("@>", self.type_string())
            }
            error => error,
        }
    }

    fn greater_or_equal(&self, other: &KValue) -> runtime::Result<bool> {
        match self.less(other) {
            Ok(result) => Ok(!result),
            Err(error) if error.is_unimplemented_error() => {
                unimplemented_error("@>=", self.type_string())
            }
            error => error,
        }
    }

    fn equal(&self, other: &KValue) -> runtime::Result<bool> {
        let _ = other;
        unimplemented_error("@==", self.type_string())
    }

    fn not_equal(&self, other: &KValue) -> runtime::Result<bool> {
        match self.equal(other) {
            Ok(result) => Ok(!result),
            Err(error) if error.is_unimplemented_error() => {
                unimplemented_error("@!=", self.type_string())
            }
            error => error,
        }
    }

    fn is_iterable(&self) -> IsIterable {
        IsIterable::NotIterable
    }

    fn make_iterator(&self, vm: &mut KotoVm) -> runtime::Result<KIterator> {
        let _ = vm;
        unimplemented_error("@iterator", self.type_string())
    }

    fn iterator_next(&mut self, vm: &mut KotoVm) -> Option<KIteratorOutput> {
        let _ = vm;
        None
    }

    fn iterator_next_back(&mut self, vm: &mut KotoVm) -> Option<KIteratorOutput> {
        let _ = vm;
        None
    }

    fn serialize(&self) -> runtime::Result<KValue> {
        unimplemented_error("serialize", self.type_string())
    }
    */
}

#[koto_impl]
impl Vec2 {
    #[rustfmt::skip]
    fn module(module: &KMap) {
        let type_module = KMap::with_type("Vec2");

        type_module.add_fn("new",      Self::new);
        type_module.add_fn("scalar",   Self::scalar);
        type_module.add_fn("scalar_x", Self::scalar_x);
        type_module.add_fn("scalar_y", Self::scalar_y);
        type_module.add_fn("zero",     Self::zero);
        type_module.add_fn("one",      Self::one);

        module.insert("Vec2", type_module);
    }

    pub fn rust_new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    //================================================================

    koto_fn! {
        // Create a new 2-D vector.
        fn new(x: f32, y: f32) -> KObject {
            Vec2 { x, y }.into()
        }

        // Create a new 2-D vector, using a scalar value for both X and Y.
        fn scalar(value: f32) -> KObject {
            Vec2 { x: value, y: value }.into()
        }

        // Create a new 2-D vector, using a scalar value for X.
        fn scalar_x(x: f32) -> KObject {
            Vec2 { x, y: 0.0 }.into()
        }

        // Create a new 2-D vector, using a scalar value for Y.
        fn scalar_y(y: f32) -> KObject {
            Vec2 { x: 0.0, y }.into()
        }

        // Create a new 2-D vector, with every component set to zero.
        fn zero() -> KObject {
            Vec2 { x: 0.0, y: 0.0 }.into()
        }

        // Create a new 2-D vector, with every component set to one.
        fn one() -> KObject {
            Vec2 { x: 1.0, y: 1.0 }.into()
        }
    }

    #[koto_method]
    fn x(&self) -> runtime::Result<KNumber> {
        Ok(self.x.into())
    }

    #[koto_method]
    fn y(&self) -> runtime::Result<KNumber> {
        Ok(self.y.into())
    }

    //#[koto_method]
    //fn dot(&self, other: &Vec2) -> f32 {
    //    (self.x * other.x) + (self.y * other.y)
    //}
}

//================================================================

/*
// A 3-D vector.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vec3 {
    // 'X' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub x: f32,
    // 'Y' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub y: f32,
    // 'Z' component.
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

    // Create a new 3-D vector.
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    // Create a new 3-D vector, using a scalar value for X, Y and Z.
    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }

    // Create a new 3-D vector, using a scalar value for X.
    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self { x, y: 0.0, z: 0.0 }
    }

    // Create a new 3-D vector, using a scalar value for Y.
    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self { x: 0.0, y, z: 0.0 }
    }

    // Create a new 3-D vector, using a scalar value for Z.
    #[rune::function(path = Self::z)]
    fn z(z: f32) -> Self {
        Self { x: 0.0, y: 0.0, z }
    }

    // Create a new 3-D vector, with every component set to zero.
    #[rune::function(path = Self::zero)]
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    // Create a new 3-D vector, with every component set to one.
    #[rune::function(path = Self::one)]
    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    // Calculate the dot product with another vector.
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

// A 4-D vector.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Vec4 {
    // 'X' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub x: f32,
    // 'Y' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub y: f32,
    // 'Z' component.
    #[rune(get, set, add_assign, sub_assign, div_assign, mul_assign, rem_assign)]
    pub z: f32,
    // 'W' component.
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

    // Create a new 4-D vector.
    #[rune::function(path = Self::new)]
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    // Create a new 4-D vector, using a scalar value for X, Y, Z and W.
    #[rune::function(path = Self::scalar)]
    fn scalar(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
            w: value,
        }
    }

    // Create a new 4-D vector, using a scalar value for X.
    #[rune::function(path = Self::x)]
    fn x(x: f32) -> Self {
        Self {
            x,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    // Create a new 4-D vector, using a scalar value for Y.
    #[rune::function(path = Self::y)]
    fn y(y: f32) -> Self {
        Self {
            x: 0.0,
            y,
            z: 0.0,
            w: 0.0,
        }
    }

    // Create a new 4-D vector, using a scalar value for Z.
    #[rune::function(path = Self::z)]
    fn z(z: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z,
            w: 0.0,
        }
    }

    // Create a new 4-D vector, using a scalar value for W.
    #[rune::function(path = Self::w)]
    fn w(w: f32) -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w,
        }
    }

    // Create a new 4-D vector, with every component set to zero.
    #[rune::function(path = Self::zero)]
    fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }

    // Create a new 4-D vector, with every component set to one.
    #[rune::function(path = Self::one)]
    fn one() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        }
    }

    // Calculate the dot product with another vector.
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

// A 2-D box, with point, scale, and angle.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Box2 {
    // Point of the box.
    #[rune(get, set, copy)]
    pub point: Vec2,
    // Scale of the box.
    #[rune(get, set, copy)]
    pub scale: Vec2,
    // Angle of the box.
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

    // Create a new 2-D box.
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

// A color.
#[derive(Any, TryClone, Copy, Clone, Debug)]
#[rune(item = ::general)]
pub struct Color {
    // 'R' channel.
    #[rune(get, set)]
    pub r: u8,
    // 'G' channel.
    #[rune(get, set)]
    pub g: u8,
    // 'B' channel.
    #[rune(get, set)]
    pub b: u8,
    // 'A' channel.
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
*/

//================================================================

pub fn module() -> KMap {
    let module = KMap::with_type("general");

    Vec2::module(&module);
    //Vec3::module(&module);
    //Vec4::module(&module);
    //Box2::module(&module);
    //Color::module(&module);

    module
}
