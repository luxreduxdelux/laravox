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
use std::sync::Arc;

//================================================================

use rune::{Any, Module, Mut, alloc::HashMap, runtime::Function};
use three_d::{ClearState, ColorMaterial, ColorTarget, CpuTexture, Gm, Rectangle, RenderTarget};

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Frame {
    // active batch texture.
    texture: Option<Arc<three_d::Texture2D>>,

    // GPU buffer data.
    main_vertex_point: three_d::VertexBuffer<three_d::Vector2<f32>>,
    main_texture_point: three_d::VertexBuffer<three_d::Vector2<f32>>,
    main_texture_color: three_d::VertexBuffer<three_d::Vector4<f32>>,

    // CPU buffer data.
    side_vertex_point: Vec<three_d::Vector2<f32>>,
    side_texture_point: Vec<three_d::Vector2<f32>>,
    side_texture_color: Vec<three_d::Vector4<f32>>,

    // shader program.
    program: three_d::Program,
}

impl Frame {
    const VERTEX_POINT: &str = "vs_vertex_point";
    const TEXTURE_POINT: &str = "vs_texture_point";
    const TEXTURE_COLOR: &str = "vs_texture_color";
    const TEXTURE_SAMPLE: &str = "texture_sample";
    const VIEW_PROJECTION: &str = "view_projection";

    #[rune::function(path = Self::new)]
    #[rustfmt::skip]
    fn new(state: &State) -> Self {
        use three_d::*;

        // initialize each CPU buffer.
        let side_vertex_point  = Vec::with_capacity(1024);
        let side_texture_point = Vec::with_capacity(1024);
        let side_texture_color = Vec::with_capacity(1024);

        // initialize each GPU buffer.
        let main_vertex_point  = VertexBuffer::new_with_data(&state.frame_input.context, &side_vertex_point);
        let main_texture_point = VertexBuffer::new_with_data(&state.frame_input.context, &side_texture_point);
        let main_texture_color = VertexBuffer::new_with_data(&state.frame_input.context, &side_texture_color);

        let program = Program::from_source(
            &state.frame_input.context,
            include_str!("../base.vs"),
            include_str!("../base.fs"),
        )
        .unwrap();

        Self {
            texture: None,
            main_vertex_point,
            main_texture_point,
            main_texture_color,
            side_vertex_point,
            side_texture_point,
            side_texture_color,
            program,
        }
    }

    #[rune::function]
    fn draw(&mut self, state: &State, camera: &Camera, render_call: Function) {
        use three_d::*;

        state
            .frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.5, 0.5, 0.5, 1.0, 1.0))
            .write::<CoreError>(|| {
                // rust, fuck off
                unsafe {
                    let raw = self as *mut Frame;

                    render_call.call::<()>((&mut *raw,)).unwrap();
                }

                self.flush(state, camera);

                Ok(())
            })
            .unwrap();
    }

    fn draw_image(&mut self, image: &Image, box_a: &Box2 /*box_b: &Box2, color: Color*/) {
        // TO-DO only clone once, when setting a new texture, then re-use thereafter.

        let scale = (image.data.width() as f32, image.data.height() as f32);

        let box_b = Box2 {
            point: crate::general::Vector2 { x: 0.0, y: 0.0 },
            scale: crate::general::Vector2 {
                x: scale.0,
                y: scale.1,
            },
            angle: 0.0,
        };

        use three_d::*;

        self.texture = Some(image.data.clone());

        let x1 = box_a.point.x;
        let y1 = box_a.point.y;
        let x2 = box_a.point.x + box_a.scale.x;
        let y2 = box_a.point.y + box_a.scale.y;

        let u1 = box_b.point.x / scale.0;
        let v1 = box_b.point.y / scale.1;
        let u2 = (box_b.point.x + box_b.scale.x) / scale.0;
        let v2 = (box_b.point.y + box_b.scale.y) / scale.1;

        let color = vec4(
            1.0, //color.r as f32 / 255.0,
            1.0, //color.g as f32 / 255.0,
            1.0, //color.b as f32 / 255.0,
            1.0, //color.a as f32 / 255.0,
        );

        // move data into CPU buffer.
        self.side_vertex_point.extend([
            vec2(x1, y1),
            vec2(x2, y2),
            vec2(x1, y2),
            vec2(x1, y1),
            vec2(x2, y1),
            vec2(x2, y2),
        ]);
        self.side_texture_point.extend([
            vec2(u1, v1),
            vec2(u2, v2),
            vec2(u1, v2),
            vec2(u1, v1),
            vec2(u2, v1),
            vec2(u2, v2),
        ]);
        self.side_texture_color
            .extend([color, color, color, color, color, color]);
    }

    #[rustfmt::skip]
    fn flush(&mut self, state: &State, camera: &Camera) {
        if let Some(self_texture) = &self.texture && !self.side_vertex_point.is_empty() {
            //println!("flush!");

            use three_d::*;

            // move CPU buffer data to the GPU.
            self.main_vertex_point.fill(&self.side_vertex_point);
            self.main_texture_point.fill(&self.side_texture_point);
            self.main_texture_color.fill(&self.side_texture_color);

            // set every uniform, attribute.
            self.program.use_uniform(Self::VIEW_PROJECTION, camera.0.projection() * camera.0.view());
            self.program.use_texture(Self::TEXTURE_SAMPLE, self_texture);
            self.program.use_vertex_attribute(Self::VERTEX_POINT, &self.main_vertex_point);
            self.program.use_vertex_attribute(Self::TEXTURE_POINT, &self.main_texture_point);
            self.program.use_vertex_attribute(Self::TEXTURE_COLOR, &self.main_texture_color);

            // render the batch.
            self.program.draw_arrays(
                RenderStates::default(),
                state.frame_input.viewport,
                self.main_vertex_point.vertex_count(),
            );

            // clear CPU buffer.
            self.side_vertex_point.clear();
            self.side_texture_point.clear();
            self.side_texture_color.clear();
        }
    }
}

//================================================================

#[derive(Any, Clone)]
#[allow(dead_code)]
#[rune(item = ::video)]
struct Camera(three_d::Camera, Vector2);

impl Camera {
    #[rune::function(path = Self::new)]
    fn new(state: &State) -> Self {
        let mut camera = three_d::Camera::new_2d(state.frame_input.viewport);
        camera.disable_tone_and_color_mapping();

        Self(camera, Vector2 { x: 0.0, y: 0.0 })
    }

    fn origin(&self, vector: &Vector2) -> Vector2 {
        Vector2 {
            x: self.0.viewport().x as f32 + vector.x,
            //y: vector.y,
            y: self.0.viewport().height as f32 - vector.y,
        }
    }

    fn draw_texture(
        &self,
        data: &mut Gm<Rectangle, ColorMaterial>,
        box_a: &Box2,
        box_b: &Box2,
        color: &Color,
    ) {
        use three_d::*;

        let mut point = self.origin(&box_a.point);
        point.x += box_a.scale.x * 0.5;
        point.y -= box_a.scale.y * 0.5;

        data.geometry.set_center(vec2(point.x, point.y));
        data.geometry.set_size(box_a.scale.x, box_a.scale.y);
        data.geometry.set_rotation(degrees(box_a.angle));

        let texture = data.material.texture.as_mut().unwrap();
        let texture_size = vec2(
            texture.texture.width() as f32,
            texture.texture.height() as f32,
        );

        let m_point = Mat3::from_translation(vec2(
            box_b.point.x / texture_size.x,
            (texture_size.y - box_b.point.y) / texture_size.y,
        ));
        let m_scale = Mat3::from_nonuniform_scale(
            box_b.scale.x / texture_size.x,
            box_b.scale.y / texture_size.y,
        );

        texture.transformation = m_point * m_scale;

        data.material.color = color.clone().into();

        data.render(&self.0, &[]);
    }
}

//================================================================

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Render {
    texture_color: three_d::Texture2D,
    #[allow(dead_code)]
    data: Gm<Rectangle, ColorMaterial>,
    #[rune(get, set)]
    box_a: Box2,
    #[rune(get, set)]
    box_b: Box2,
    #[rune(get, set)]
    color: Color,
}

impl Render {
    #[rune::function(path = Self::new)]
    fn new(state: &State, scale: Vector2) -> anyhow::Result<Self> {
        use three_d::*;

        let texture_color = Texture2D::new_empty::<[u8; 4]>(
            &state.frame_input.context,
            scale.x as u32,
            scale.y as u32,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let texture = Texture2D::new_empty::<[u8; 4]>(
            &state.frame_input.context,
            scale.x as u32,
            scale.y as u32,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );
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
                vec2(0.0, 0.0),
                degrees(0.0),
                0.0,
                0.0,
            ),
            texture,
        );

        let t = data.material.texture.as_ref().unwrap();
        let s = (t.width() as f32, t.height() as f32);

        Ok(Self {
            texture_color,
            data,
            box_a: Box2 {
                point: crate::general::Vector2 { x: 0.0, y: 0.0 },
                scale: crate::general::Vector2 { x: s.0, y: s.1 },
                angle: 0.0,
            },
            box_b: Box2 {
                point: crate::general::Vector2 { x: 0.0, y: 0.0 },
                scale: crate::general::Vector2 { x: s.0, y: s.1 },
                angle: 0.0,
            },
            color: crate::general::Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        })
    }

    #[rune::function]
    fn draw_to(&mut self, state: &State, mut camera: Camera, call: Function) {
        use three_d::*;

        // this should really only use the material.texture from the
        // render target, however, DerefMut is not available for
        // an Arc<Texture2D>, so we can't use it.

        let color = self.texture_color.as_color_target(None);

        let mut view = camera.0.viewport();

        view.y = -(view.height as i32 - color.height() as i32);

        camera.0.set_viewport(view);

        let data = color
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write::<RendererError>(|| {
                call.call::<()>((camera,)).unwrap();
                Ok(())
            })
            .unwrap()
            .read();

        let texture = CpuTexture {
            data: TextureData::RgbaU8(data),
            width: color.width(),
            height: color.height(),
            min_filter: Interpolation::Nearest,
            mag_filter: Interpolation::Nearest,
            ..Default::default()
        };

        self.data.material.texture = Some(Texture2DRef::from_cpu_texture(
            &state.frame_input.context,
            &texture,
        ));
    }

    #[rune::function]
    #[inline]
    fn draw(&mut self, camera: &Camera) {
        camera.draw_texture(&mut self.data, &self.box_a, &self.box_b, &self.color);
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Image {
    #[allow(dead_code)]
    data: Arc<three_d::Texture2D>,
}

impl Image {
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str) -> anyhow::Result<Self> {
        use three_d::*;

        let mut texture = three_d_asset::io::load(&[path])?;
        let texture = Texture2D::new(&state.frame_input.context, &texture.deserialize("")?);

        Ok(Self {
            data: Arc::new(texture),
        })
    }

    #[rune::function]
    #[inline]
    fn draw(&mut self, frame: &mut Frame, box_a: &Box2) {
        frame.draw_image(self, box_a);

        //camera.draw_texture(&mut self.data, &self.box_a, &self.box_b, &self.color);
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
    scale: f32,
}

impl Font {
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str, scale: f32) -> anyhow::Result<Self> {
        let code: String = (32..127).map(|x| x as u8 as char).collect();
        let font = std::fs::read(path)?;
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        let mut layout =
            fontdue::layout::Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        layout.append(
            std::slice::from_ref(&font),
            &fontdue::layout::TextStyle::new(&code, scale, 0),
        );

        let mut data = Vec::new();
        let mut size = (0, 0);

        let mut map = HashMap::new();

        for glyph in layout.glyphs() {
            let (metric, raster) = font.rasterize(glyph.parent, scale);

            let mut buffer = vec![[0_u8, 0_u8, 0_u8, 0_u8]; metric.width * metric.height];

            for (i, pixel) in raster.iter().enumerate() {
                buffer[i] = [*pixel, *pixel, *pixel, *pixel];
            }

            map.try_insert(
                glyph.parent,
                Glyph::new(
                    size.0 as f32,
                    (0.0, glyph.y),
                    (glyph.width as f32, glyph.height as f32),
                    (metric.advance_width, metric.advance_height),
                ),
            )
            .unwrap();

            size.0 += glyph.width;
            size.1 = size.1.max(glyph.height);

            data.push((metric, buffer));
        }

        let mut buffer = vec![[0_u8, 0_u8, 0_u8, 0_u8]; size.0 * size.1];
        let mut push = 0;

        for (metric, raster) in data {
            for (i, pixel) in raster.iter().enumerate() {
                let p_x = i % metric.width;
                let p_y = i / metric.width;

                buffer[(push + p_x) + (size.0 * p_y)] = *pixel;
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
                vec2(0.0, 0.0),
                degrees(0.0),
                0.0,
                0.0,
            ),
            texture,
        );

        Ok(Self { map, data, scale })
    }

    #[rune::function]
    #[rustfmt::skip]
    fn draw(&mut self, camera: &Camera, point: &Vector2, scale: f32, text: String) {
        use three_d::*;

        let mut push = 0.0;

        let point = camera.origin(point);
        let scale = scale / self.scale;

        for character in text.chars() {
            if let Some(glyph) = self.map.get(&character) {
                self.data.geometry.set_center(vec2(
                    point.x + glyph.scale.0 * 0.5 + push,
                    point.y - glyph.scale.1 * 0.5 - glyph.point.1
                ));
                self.data.geometry.set_size(glyph.scale.0 * scale, glyph.scale.1 * scale);

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
    module.function_meta(Frame::new)?;
    module.function_meta(Frame::draw)?;

    module.ty::<Camera>()?;
    module.function_meta(Camera::new)?;

    module.ty::<Render>()?;
    module.function_meta(Render::new)?;
    module.function_meta(Render::draw_to)?;
    module.function_meta(Render::draw)?;

    module.ty::<Image>()?;
    module.function_meta(Image::new)?;
    module.function_meta(Image::draw)?;

    module.ty::<Font>()?;
    module.function_meta(Font::new)?;
    module.function_meta(Font::draw)?;

    Ok(module)
}
