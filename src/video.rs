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
    general::{Box2, Color, Vector2},
};

//================================================================

use rune::{Any, Module, alloc::HashMap};
use three_d::{ClearState, ColorMaterial, ColorTarget, CpuTexture, Gm, Rectangle, RenderTarget};

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
            is_transparent: true,
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
            texture: Some(texture.into()),
            ..Default::default()
        };

        let data = Gm::new(
            Rectangle::new(
                &state.frame_input.context,
                vec2(0.0, 9.0),
                degrees(0.0),
                0.0,
                0.0,
            ),
            texture,
        );

        Ok(Self { data })
    }

    #[rune::function]
    fn draw(&mut self, camera: &Camera, _box: &Box2, color: Color) {
        use three_d::*;

        self.data
            .geometry
            .set_center(vec2(_box.point.x, _box.point.y));
        self.data.geometry.set_size(_box.scale.x, _box.scale.y);
        self.data.geometry.set_rotation(degrees(_box.angle));

        self.data.material.color = color.into();

        self.data.render(&camera.0, &[]);
    }
}

//================================================================

struct Glyph {
    shift: f32,
    point: (f32, f32),
    scale: (f32, f32),
    push: (f32, f32),
}

impl Glyph {
    fn new(shift: f32, point: (f32, f32), scale: (f32, f32), push: (f32, f32)) -> Self {
        Self {
            shift,
            point,
            scale,
            push,
        }
    }
}

#[derive(Any)]
#[rune(item = ::video)]
struct Font {
    #[allow(dead_code)]
    map: HashMap<char, Glyph>,
    data: Gm<Rectangle, ColorMaterial>,
}

impl Font {
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str, scale: f32) -> anyhow::Result<Self> {
        let font = std::fs::read(path)?;
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        //let mut layout =
        //    fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYUp);
        //layout.append(
        //    &[font],
        //    &fontdue::layout::TextStyle::new("ABCDEFGHIJKLMNOP", 32.0, 0),
        //);

        let mut data = Vec::new();
        let mut size = (0, 0);

        let mut map = HashMap::new();

        for x in 32..127 {
            let character = x as u8 as char;
            let (metric, raster) = font.rasterize(character, scale);

            let mut buffer = vec![[0_u8, 0_u8, 0_u8, 0_u8]; metric.width * metric.height];

            for (i, pixel) in raster.iter().enumerate() {
                buffer[i] = [*pixel, *pixel, *pixel, *pixel];
            }

            map.try_insert(
                character,
                Glyph::new(
                    size.0 as f32,
                    (0.0, metric.ymin as f32),
                    (metric.width as f32, metric.height as f32),
                    (metric.advance_width as f32, metric.advance_height as f32),
                ),
            )
            .unwrap();

            size.0 += metric.width;
            size.1 = size.1.max(metric.height);

            data.push((metric, buffer));
        }

        let mut buffer = vec![[0_u8, 0_u8, 0_u8, 0_u8]; size.0 * size.1];
        let mut push = 0;

        for (metric, raster) in data {
            for y in 0..metric.height {
                let dst_start = y * size.0 + push;
                let src_start = y * metric.width;

                buffer[dst_start..dst_start + metric.width]
                    .copy_from_slice(&raster[src_start..src_start + metric.width]);
            }

            push += metric.width;
        }

        use three_d::*;

        let texture = CpuTexture {
            data: TextureData::RgbaU8(buffer),
            width: size.0 as u32,
            height: size.1 as u32,
            min_filter: Interpolation::Nearest,
            mag_filter: Interpolation::Nearest,
            ..Default::default()
        };

        let texture = ColorMaterial {
            is_transparent: true,
            render_states: RenderStates {
                write_mask: WriteMask::COLOR,
                blend: Blend::TRANSPARENCY,
                ..Default::default()
            },
            texture: Some(Texture2DRef::from_cpu_texture(
                &state.frame_input.context,
                &texture,
            )),
            ..Default::default()
        };

        let data = Gm::new(
            Rectangle::new(
                &state.frame_input.context,
                vec2(0.0, size.1 as f32 * 4.0),
                degrees(0.0),
                size.0 as f32 * 4.0,
                size.1 as f32 * 4.0,
            ),
            texture,
        );

        Ok(Self { map, data })
    }

    #[rune::function]
    #[rustfmt::skip]
    fn draw(&mut self, camera: &Camera, point: Vector2, scale: f32, text: String) {
        use three_d::*;

        let mut push = 0.0;

        for character in text.chars() {
            if let Some(glyph) = self.map.get(&character) {
                self.data.geometry.set_size(glyph.scale.0 * scale, glyph.scale.1 * scale);
                self.data.geometry.set_center(vec2(
                    point.x + (glyph.scale.0 * scale * 0.5) + push,
                    point.y + (glyph.scale.1 * scale * 0.5) + glyph.point.1
                ));

                let texture = self.data.material.texture.as_mut().unwrap();
                let texture_size = vec2(
                    texture.texture.width() as f32,
                    texture.texture.height() as f32,
                );

                let m_point = Mat3::from_translation(vec2(
                    glyph.shift / texture_size.x,
                    (texture_size.y - glyph.scale.1) / texture_size.y,
                ));
                let m_scale = Mat3::from_nonuniform_scale(
                    glyph.scale.0 / texture_size.x,
                    glyph.scale.1 / texture_size.y
                );

                texture.transformation = m_point * m_scale;

                push += glyph.push.0 * scale;

                self.data.render(&camera.0, &[]);
            }
        }
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

    module.ty::<Font>()?;
    module.function_meta(Font::new)?;
    module.function_meta(Font::draw)?;

    Ok(module)
}
