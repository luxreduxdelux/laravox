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

use crate::{
    app::State,
    general::{Box2, Vector2},
};

//================================================================

use rune::{Any, Module};
use three_d::{ClearState, ColorMaterial, Gm, Rectangle};

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Frame {}

impl Frame {
    #[rune::function(path = Self::clear)]
    fn clear(state: &State) {
        state
            .frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0));
    }
}

//================================================================

#[derive(Any)]
#[allow(dead_code)]
#[rune(item = ::video)]
struct Camera(three_d::Camera);

impl Camera {
    #[rune::function(path = Self::new)]
    fn new(state: &State) -> Self {
        let mut camera = three_d::Camera::new_2d(state.frame_input.viewport);
        camera.disable_tone_and_color_mapping();

        Self(camera)
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Image {
    #[allow(dead_code)]
    data: Gm<Rectangle, ColorMaterial>,
}

impl Image {
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str) -> anyhow::Result<Self> {
        use three_d::*;

        let mut texture = three_d_asset::io::load(&[path])?;
        let texture = Texture2D::new(&state.frame_input.context, &texture.deserialize("")?);
        let texture = ColorMaterial {
            texture: Some(Texture2DRef::from_texture(texture)),
            ..Default::default()
        };

        let data = Gm::new(
            Rectangle::new(
                &state.frame_input.context,
                vec2(192.0, 192.0),
                degrees(0.0),
                128.0,
                128.0,
            ),
            texture,
        );

        Ok(Self { data })
    }

    #[rune::function]
    fn draw(&mut self, camera: &Camera, _box: &Box2) {
        use three_d::*;

        self.data
            .geometry
            .set_center(vec2(_box.point.x, _box.point.y));
        self.data.render(&camera.0, &[]);
    }

    #[rune::function]
    fn test(&mut self, function: rune::runtime::Function) -> anyhow::Result<()> {
        Ok(function.call::<()>((1,)).into_result()?)
    }
}

//================================================================

#[rune::module(::video)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    module.ty::<Frame>()?;
    module.function_meta(Frame::clear)?;

    module.ty::<Camera>()?;
    module.function_meta(Camera::new)?;

    module.ty::<Image>()?;
    module.function_meta(Image::new)?;
    module.function_meta(Image::draw)?;
    module.function_meta(Image::test)?;

    Ok(module)
}
