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
    module::general::{Box2, Color, Vec2, Vec3, Vec4},
    script::State,
};
use std::sync::Arc;

//================================================================

use rune::{
    Any, Module, Value,
    alloc::HashMap,
    runtime::{Function, VmResult},
};

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
pub struct Frame {
    // active batch texture.
    // UNFORTUNATELY, three_d does NOT make the texture ID public,
    // so we need a way to hash the texture somehow...that way is through
    // a string, for now. hopefully the ID is made public some day.
    image: Option<(String, Arc<three_d::Texture2D>)>,

    basic: (String, Arc<three_d::Texture2D>),

    // active camera.
    camera: Option<three_d::Camera>,

    // GPU buffer data.
    main_vertex_point: three_d::VertexBuffer<three_d::Vector2<f32>>,
    main_texture_point: three_d::VertexBuffer<three_d::Vector2<f32>>,
    main_texture_color: three_d::VertexBuffer<three_d::Vector4<f32>>,

    // CPU buffer data.
    side_vertex_point: Vec<three_d::Vector2<f32>>,
    side_texture_point: Vec<three_d::Vector2<f32>>,
    side_texture_color: Vec<three_d::Vector4<f32>>,

    // shader program.
    program_active: Option<Arc<three_d::Program>>,
    program_normal: three_d::Program,
}

impl Frame {
    const VERTEX_POINT: &str = "vs_vertex_point";
    const TEXTURE_POINT: &str = "vs_texture_point";
    const TEXTURE_COLOR: &str = "vs_texture_color";
    const TEXTURE_SAMPLE: &str = "texture_sample";
    const VIEW_PROJECTION: &str = "view_projection";

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::set_shader)?;
        module.function_meta(Self::draw)?;
        module.function_meta(Self::draw_to)?;
        module.function_meta(Self::draw_box)?;
        module.function_meta(Self::draw_line)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    #[rustfmt::skip]
    fn new(state: &State) -> Self {
        use three_d::*;

        // initialize each CPU buffer.
        let side_vertex_point  = Vec::with_capacity(1024);
        let side_texture_point = Vec::with_capacity(1024);
        let side_texture_color = Vec::with_capacity(1024);

        // initialize each GPU buffer.
        let main_vertex_point  = VertexBuffer::new_with_data(&state.frame.context, &side_vertex_point);
        let main_texture_point = VertexBuffer::new_with_data(&state.frame.context, &side_texture_point);
        let main_texture_color = VertexBuffer::new_with_data(&state.frame.context, &side_texture_color);

        let program = Program::from_source(
            &state.frame.context,
            include_str!("../../data/base.vs"),
            include_str!("../../data/base.fs"),
        )
        .unwrap();

        let basic = CpuTexture {
            data: TextureData::RgbaU8(vec![[255, 255, 255, 255]]),
            width: 1,
            height: 1,
            ..Default::default()
        };

        let basic = Texture2D::new(&state.frame.context, &basic);

        Self {
            image: None,
            basic: ("frame_box".to_string(), Arc::new(basic)),
            camera: None,
            main_vertex_point,
            main_texture_point,
            main_texture_color,
            side_vertex_point,
            side_texture_point,
            side_texture_color,
            program_active: None,
            program_normal: program,
        }
    }

    #[rune::function]
    fn set_shader(&mut self, shader: &Shader) {
        // TO-DO check if the given shader is not the same as the current active shader.
        // if it is the same, then don't flush at all.
        self.flush();

        self.program_active = Some(shader.data.clone());
    }

    #[rune::function]
    fn draw(&mut self, state: &State, camera: &Camera, render_call: Function) -> VmResult<()> {
        use three_d::*;

        let camera = camera.clone();

        self.camera = Some(camera.inner);

        let mut result: VmResult<()> = VmResult::Ok(());

        state
            .frame
            .screen()
            .clear(ClearState::color_and_depth(1.0, 0.0, 0.0, 1.0, 1.0))
            .write::<CoreError>(|| {
                // rust, fuck off
                unsafe {
                    let raw = self as *mut Frame;

                    result = render_call.call::<()>((&mut *raw,));
                }

                self.flush();

                Ok(())
            })
            .unwrap();

        self.program_active = None;

        result
    }

    #[rune::function]
    fn draw_to(
        &mut self,
        state: &State,
        camera: &Camera,
        render: &mut Render,
        render_call: Function,
    ) -> VmResult<()> {
        use three_d::*;

        let mut camera = camera.clone();

        let viewport = Viewport::new_at_origo(render.data.width(), render.data.height());

        camera.inner.set_viewport(viewport);
        camera.inner.set_view(
            three_d::vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * -0.5,
                1.0,
            ),
            three_d::vec3(
                viewport.width as f32 * 0.5,
                viewport.height as f32 * -0.5,
                0.0,
            ),
            three_d::vec3(0.0, 1.0, 0.0),
        );
        camera
            .inner
            .set_orthographic_projection(viewport.height as f32, 0.0, 1.0);

        self.camera = Some(camera.inner);

        let color = render.data_write.as_color_target(None);
        let mut value = VmResult::Ok(());

        let data: Vec<[u8; 4]> = color
            .clear(ClearState::color_and_depth(1.0, 0.0, 0.0, 1.0, 1.0))
            .write::<CoreError>(|| {
                // rust, fuck off
                unsafe {
                    let raw = self as *mut Frame;

                    value = render_call.call::<()>((&mut *raw,));
                }

                self.flush();

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

        // TO-DO there's probably a better way to go about this...
        render.data = Arc::new(Texture2D::new(&state.frame.context, &texture));

        self.program_active = None;

        value
    }

    #[rune::function]
    fn draw_box(&mut self, box_a: &Box2, color: &Color) {
        let data = self.basic.1.clone();

        self.draw_image(
            "frame_box",
            &data,
            box_a,
            &Box2::rust_new(&Vec2::rust_new(0.0, 0.0), &Vec2::rust_new(1.0, 1.0), 0.0),
            color,
        );
    }

    #[rune::function]
    fn draw_line(&mut self, point_a: &Vec2, point_b: &Vec2, thick: f32, color: &Color) {
        let data = self.basic.1.clone();

        let dx = (point_b.x - point_a.x).abs() as i32;
        let dy = (point_b.y - point_a.y).abs() as i32 * -1;
        let sx = if point_a.x < point_b.x { 1 } else { -1 };
        let sy = if point_a.y < point_b.y { 1 } else { -1 };

        let mut e_1 = dx + dy;
        let mut e_2;

        let mut x = point_a.x as i32;
        let mut y = point_a.y as i32;

        let f_x = point_b.x as i32;
        let f_y = point_b.y as i32;

        loop {
            self.draw_image(
                "frame_box",
                &data,
                &Box2::rust_new(
                    &Vec2::rust_new(x as f32, y as f32),
                    &Vec2::rust_new(thick, thick),
                    0.0,
                ),
                &Box2::rust_new(&Vec2::rust_new(0.0, 0.0), &Vec2::rust_new(1.0, 1.0), 0.0),
                color,
            );

            if x == f_x && y == f_y {
                break;
            }

            e_2 = e_1 * 2;

            if e_2 >= dy {
                if x == f_x {
                    break;
                }

                e_1 += dy;
                x += sx;
            }

            if e_2 <= dx {
                if y == f_y {
                    break;
                }

                e_1 += dx;
                y += sy;
            }
        }
    }

    pub fn rust_draw_line(&mut self, point_a: &Vec2, point_b: &Vec2, thick: i32, color: &Color) {
        let data = self.basic.1.clone();

        let dx = (point_b.x - point_a.x).abs() as i32;
        let dy = (point_b.y - point_a.y).abs() as i32 * -1;
        let sx = if point_a.x < point_b.x { 1 } else { -1 };
        let sy = if point_a.y < point_b.y { 1 } else { -1 };

        let mut e_1 = dx + dy;
        let mut e_2 = 0;

        let mut x = point_a.x as i32;
        let mut y = point_a.y as i32;

        let f_x = point_b.x as i32;
        let f_y = point_b.y as i32;

        loop {
            self.draw_image(
                "frame_box",
                &data,
                &Box2::rust_new(
                    &Vec2::rust_new(x as f32, y as f32),
                    &Vec2::rust_new(thick as f32, thick as f32),
                    0.0,
                ),
                &Box2::rust_new(&Vec2::rust_new(0.0, 0.0), &Vec2::rust_new(1.0, 1.0), 0.0),
                color,
            );

            if x == f_x && y == f_y {
                break;
            }

            e_2 = e_1 * 2;

            if e_2 >= dy {
                if x == f_x {
                    break;
                }

                e_1 += dy;
                x += sx;
            }

            if e_2 <= dx {
                if y == f_y {
                    break;
                }

                e_1 += dx;
                y += sy;
            }
        }
    }

    fn draw_image(
        &mut self,
        hash: &str,
        image: &Arc<three_d::Texture2D>,
        box_a: &Box2,
        box_b: &Box2,
        color: &Color,
    ) {
        let scale = (image.width() as f32, image.height() as f32);

        use three_d::*;

        // if we are already have an image to draw with...
        if let Some((self_hash, _)) = &self.image {
            // if the given image is not the same as our current one, flush the queue.
            if hash != self_hash {
                self.flush();

                // replace image.
                self.image = Some((hash.to_string(), image.clone()));
            }
        } else {
            // no image. set as active image.
            self.image = Some((hash.to_string(), image.clone()));
        }

        let mut box_a = *box_a;

        if box_a.scale.x < 0.0 {
            box_a.point.x -= box_a.scale.x;
        }
        if box_a.scale.y < 0.0 {
            box_a.point.y -= box_a.scale.y;
        }

        let x1 = box_a.point.x;
        let y1 = -box_a.point.y;
        let x2 = box_a.point.x + box_a.scale.x;
        let y2 = -box_a.point.y - box_a.scale.y;

        let u1 = box_b.point.x / scale.0;
        let v1 = -box_b.point.y / scale.1;
        let u2 = (box_b.point.x + box_b.scale.x) / scale.0;
        let v2 = (-box_b.point.y - box_b.scale.y) / scale.1;

        let color = vec4(
            color.r as f32 / 255.0,
            color.g as f32 / 255.0,
            color.b as f32 / 255.0,
            color.a as f32 / 255.0,
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
    fn flush(&mut self) {
        if let Some((_, self_image)) = &self.image && let Some(camera) = &self.camera && !self.side_vertex_point.is_empty() {
            //println!("flush!");

            use three_d::*;

            // move CPU buffer data to the GPU.
            self.main_vertex_point.fill(&self.side_vertex_point);
            self.main_texture_point.fill(&self.side_texture_point);
            self.main_texture_color.fill(&self.side_texture_color);

            let program = if let Some(program) = &self.program_active {
                program
            } else {
                &self.program_normal
            };

            // set every uniform, attribute.
            program.use_uniform(Self::VIEW_PROJECTION, camera.projection() * camera.view());
            program.use_texture(Self::TEXTURE_SAMPLE, self_image);
            program.use_vertex_attribute(Self::VERTEX_POINT, &self.main_vertex_point);
            program.use_vertex_attribute(Self::TEXTURE_POINT, &self.main_texture_point);
            program.use_vertex_attribute(Self::TEXTURE_COLOR, &self.main_texture_color);

            // render the batch.
            program.draw_arrays(
                RenderStates {
                    write_mask: WriteMask::COLOR,
                    blend: Blend::TRANSPARENCY,
                    ..Default::default()
                },
                camera.viewport(),
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

#[derive(Any)]
#[rune(item = ::video)]
struct Shader {
    #[allow(dead_code)]
    data: Arc<three_d::Program>,
}

impl Shader {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::set_uniform_u8)?;
        module.function_meta(Self::set_uniform_u16)?;
        module.function_meta(Self::set_uniform_u32)?;
        module.function_meta(Self::set_uniform_i8)?;
        module.function_meta(Self::set_uniform_i16)?;
        module.function_meta(Self::set_uniform_i32)?;
        module.function_meta(Self::set_uniform_vec_2)?;
        module.function_meta(Self::set_uniform_vec_3)?;
        module.function_meta(Self::set_uniform_vec_4)?;
        module.function_meta(Self::set_uniform_image)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    fn new(
        state: &State,
        path_vs: Option<String>,
        path_fs: Option<String>,
    ) -> anyhow::Result<Self> {
        use three_d::*;

        let path_vs = {
            if let Some(path_vs) = path_vs {
                &std::fs::read_to_string(path_vs)?
            } else {
                include_str!("../../data/base.vs")
            }
        };

        let path_fs = {
            if let Some(path_fs) = path_fs {
                &std::fs::read_to_string(path_fs)?
            } else {
                include_str!("../../data/base.vs")
            }
        };

        let program = Program::from_source(&state.frame.context, path_vs, path_fs).unwrap();

        Ok(Self {
            data: Arc::new(program),
        })
    }

    #[rune::function]
    fn set_uniform_u8(&self, name: String, data: u8) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_u16(&self, name: String, data: u16) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_u32(&self, name: String, data: u32) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_i8(&self, name: String, data: i8) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_i16(&self, name: String, data: i16) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_i32(&self, name: String, data: i32) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_f32(&self, name: String, data: f32) {
        self.data.use_uniform_if_required(&name, data);
    }

    #[rune::function]
    fn set_uniform_vec_2(&self, name: String, data: &Vec2) {
        self.data
            .use_uniform_if_required(&name, three_d::vec2(data.x, data.y));
    }

    #[rune::function]
    fn set_uniform_vec_3(&self, name: String, data: &Vec3) {
        self.data
            .use_uniform_if_required(&name, three_d::vec3(data.x, data.y, data.z));
    }

    #[rune::function]
    fn set_uniform_vec_4(&self, name: String, data: &Vec4) {
        self.data
            .use_uniform_if_required(&name, three_d::vec4(data.x, data.y, data.z, data.w));
    }

    #[rune::function]
    fn set_uniform_image(&self, name: String, data: &Image) {
        if self.data.requires_uniform(&name) {
            self.data.use_texture(&name, &data.data);
        }
    }
}

//================================================================

#[derive(Any, Clone)]
#[allow(dead_code)]
#[rune(item = ::video)]
struct Camera {
    inner: three_d::Camera,
}

impl Camera {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    fn new(state: &State, point: &Vec2, focus: &Vec2, angle: f32, zoom: f32) -> Self {
        let mut point = *point;

        let scale = Vec2::rust_new(
            state.frame.viewport.width as f32,
            state.frame.viewport.height as f32,
        );

        point.x += focus.x + (scale.x - scale.x / zoom) * 0.5 * -1.0;
        point.y += focus.y + (scale.y - scale.y / zoom) * 0.5;

        let scale = Vec2::rust_new(
            state.frame.viewport.width as f32 * 0.5,
            state.frame.viewport.height as f32 * -0.5,
        );

        // TO-DO zoom with anchor. by default it's the center of the view-port.

        let mut camera = three_d::Camera::new_orthographic(
            state.frame.viewport,
            three_d::vec3(point.x + scale.x, point.y + scale.y, 1.0),
            three_d::vec3(point.x + scale.x, point.y + scale.y, 0.0),
            three_d::vec3(0.0, 1.0, 0.0),
            state.frame.viewport.height as f32,
            0.0,
            10.0,
        );

        camera.roll(three_d::degrees(angle));
        camera.set_zoom_factor(zoom);

        camera.disable_tone_and_color_mapping();

        Self { inner: camera }
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Render {
    data_write: three_d::Texture2D,
    data: Arc<three_d::Texture2D>,
    hash: String,
}

impl Render {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::draw)?;
        module.function_meta(Self::draw_box)?;
        module.function_meta(Self::draw_box_color)?;
        module.function_meta(Self::draw_box_color_clip)?;
        module.function_meta(Self::scale)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    fn new(state: &State, scale: Vec2) -> anyhow::Result<Self> {
        use three_d::*;

        let data_write = Texture2D::new_empty::<[u8; 4]>(
            &state.frame.context,
            scale.x as u32,
            scale.y as u32,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let data = Arc::new(Texture2D::new_empty::<[u8; 4]>(
            &state.frame.context,
            scale.x as u32,
            scale.y as u32,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        ));

        Ok(Self {
            data_write,
            data,
            hash: "render_target".to_string(),
        })
    }

    #[rune::function]
    fn draw(&mut self, frame: &mut Frame, point: &Vec2) {
        frame.draw_image(
            &self.hash,
            &self.data,
            &Box2::rust_new(
                point,
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Color::rust_new(255, 255, 255, 255),
        );
    }

    #[rune::function]
    fn draw_box(&mut self, frame: &mut Frame, box_a: &Box2) {
        frame.draw_image(
            &self.hash,
            &self.data,
            box_a,
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Color::rust_new(255, 255, 255, 255),
        );
    }

    #[rune::function]
    fn draw_box_color(&mut self, frame: &mut Frame, box_a: &Box2, color: &Color) {
        frame.draw_image(
            &self.hash,
            &self.data,
            box_a,
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            color,
        );
    }

    #[rune::function]
    fn draw_box_color_clip(
        &mut self,
        frame: &mut Frame,
        box_a: &Box2,
        box_b: &Box2,
        color: &Color,
    ) {
        frame.draw_image(&self.hash, &self.data, box_a, box_b, color);
    }

    #[rune::function]
    fn scale(&self) -> Vec2 {
        Vec2 {
            x: self.data.width() as f32,
            y: self.data.height() as f32,
        }
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Image {
    #[allow(dead_code)]
    data: Arc<three_d::Texture2D>,
    hash: String,
}

impl Image {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::draw)?;
        module.function_meta(Self::draw_box)?;
        module.function_meta(Self::draw_box_color)?;
        module.function_meta(Self::draw_box_color_clip)?;
        module.function_meta(Self::scale)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str) -> anyhow::Result<Self> {
        use three_d::*;

        let mut texture = three_d_asset::io::load(&[path])?;
        let mut texture: CpuTexture = texture.deserialize("")?;

        texture.min_filter = Interpolation::Nearest;
        texture.mag_filter = Interpolation::Nearest;

        let texture = Texture2D::new(&state.frame.context, &texture);

        Ok(Self {
            data: Arc::new(texture),
            hash: path.to_string(),
        })
    }

    #[rune::function]
    fn draw(&mut self, frame: &mut Frame, point: &Vec2) {
        frame.draw_image(
            &self.hash,
            &self.data,
            &Box2::rust_new(
                point,
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Color::rust_new(255, 255, 255, 255),
        );
    }

    #[rune::function]
    fn draw_box(&mut self, frame: &mut Frame, box_a: &Box2) {
        frame.draw_image(
            &self.hash,
            &self.data,
            box_a,
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            &Color::rust_new(255, 255, 255, 255),
        );
    }

    #[rune::function]
    fn draw_box_color(&mut self, frame: &mut Frame, box_a: &Box2, color: &Color) {
        frame.draw_image(
            &self.hash,
            &self.data,
            box_a,
            &Box2::rust_new(
                &Vec2::rust_new(0.0, 0.0),
                &Vec2::rust_new(self.data.width() as f32, self.data.height() as f32),
                0.0,
            ),
            color,
        );
    }

    #[rune::function]
    fn draw_box_color_clip(
        &mut self,
        frame: &mut Frame,
        box_a: &Box2,
        box_b: &Box2,
        color: &Color,
    ) {
        frame.draw_image(&self.hash, &self.data, box_a, box_b, color);
    }

    #[rune::function]
    fn scale(&self) -> Vec2 {
        Vec2 {
            x: self.data.width() as f32,
            y: self.data.height() as f32,
        }
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
    data: Arc<three_d::Texture2D>,
    hash: String,
    scale: f32,
}

impl Font {
    const DEFAULT_FONT: &[u8] = include_bytes!("../../data/font.ttf");

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::draw)?;
        module.function_meta(Self::measure)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::new)]
    fn new(state: &State, path: Option<String>, scale: f32, filter: bool) -> anyhow::Result<Self> {
        let code: String = (32..127).map(|x| x as u8 as char).collect();

        let font = {
            if let Some(path) = &path {
                &std::fs::read(path)?
            } else {
                Self::DEFAULT_FONT
            }
        };

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

            // pad each letter by 4px.
            size.0 += glyph.width + 4;
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

            push += metric.width + 4;
        }

        use three_d::*;

        let filter = {
            if filter {
                Interpolation::Linear
            } else {
                Interpolation::Nearest
            }
        };

        let data = CpuTexture {
            data: TextureData::RgbaU8(buffer),
            width: size.0 as u32,
            height: size.1 as u32,
            min_filter: filter,
            mag_filter: filter,
            ..Default::default()
        };

        let data = Arc::new(Texture2D::new(&state.frame.context, &data));

        Ok(Self {
            map,
            data,
            hash: path.unwrap_or("default_font".to_string()),
            scale,
        })
    }

    #[rune::function]
    fn draw(&self, frame: &mut Frame, point: &Vec2, text: String, scale: f32) {
        let mut push = Vec2::rust_new(0.0, 0.0);
        let scale_normal = scale / self.scale;

        for character in text.chars() {
            if character == '\n' {
                push.x = 0.0;
                push.y += scale;
            } else {
                if let Some(glyph) = self.map.get(&character) {
                    frame.draw_image(
                        &self.hash,
                        &self.data,
                        &Box2 {
                            point: Vec2 {
                                x: point.x + push.x,
                                y: point.y + push.y + glyph.point.1 * scale_normal,
                            },
                            scale: Vec2 {
                                x: glyph.scale.0 * scale_normal,
                                y: glyph.scale.1 * scale_normal,
                            },
                            angle: 0.0,
                        },
                        &Box2 {
                            point: Vec2 {
                                x: glyph.shift,
                                y: 0.0,
                            },
                            scale: Vec2 {
                                x: glyph.scale.0,
                                y: glyph.scale.1,
                            },
                            angle: 0.0,
                        },
                        &Color {
                            r: 255,
                            g: 255,
                            b: 255,
                            a: 255,
                        },
                    );

                    push.x += glyph.push.0 * scale_normal;
                } else {
                    push.x += scale;
                }
            }
        }
    }

    #[rune::function]
    fn measure(&self, text: String, scale: f32) -> Vec2 {
        let mut size = Vec2::rust_new(0.0, scale);
        let mut push = 0.0;
        let scale_normal = scale / self.scale;

        for character in text.chars() {
            if character == '\n' {
                push = 0.0;
                size.y += scale;
            } else {
                if let Some(glyph) = self.map.get(&character) {
                    push += glyph.push.0 * scale_normal;
                } else {
                    push += scale;
                }
            }

            if size.x <= push {
                size.x = push;
            }
        }

        size
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::video)]
struct Window {}

impl Window {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::get_time_frame)?;
        module.function_meta(Self::get_time_since)?;
        //module.function_meta(Self::get_visible)?;
        //module.function_meta(Self::get_minimize)?;
        //module.function_meta(Self::get_maximize)?;
        //module.function_meta(Self::get_focus)?;
        //module.function_meta(Self::get_resize)?;
        module.function_meta(Self::get_scale)?;
        module.function_meta(Self::get_full)?;
        module.function_meta(Self::set_minimize)?;
        module.function_meta(Self::set_maximize)?;
        module.function_meta(Self::set_focus)?;
        module.function_meta(Self::set_name)?;
        module.function_meta(Self::set_icon)?;
        module.function_meta(Self::set_point)?;
        module.function_meta(Self::set_scale_min)?;
        module.function_meta(Self::set_scale_max)?;
        module.function_meta(Self::set_scale)?;
        module.function_meta(Self::set_full)?;

        Ok(())
    }

    //================================================================

    #[rune::function(path = Self::get_time_frame)]
    fn get_time_frame(state: &State) -> f64 {
        state.frame.elapsed_time
    }

    #[rune::function(path = Self::get_time_since)]
    fn get_time_since(state: &State) -> f64 {
        state.frame.accumulated_time
    }

    #[rune::function(path = Self::get_scale)]
    fn get_scale(state: &State) -> Vec2 {
        Vec2::rust_new(
            state.frame.window_width as f32,
            state.frame.window_height as f32,
        )
    }

    #[rune::function(path = Self::get_full)]
    fn get_full(state: &State) -> bool {
        state.input.window_get.full
    }

    #[rune::function(path = Self::set_minimize)]
    fn set_minimize(state: &mut State) {
        state.input.window_set.minimize = Some(());
    }

    #[rune::function(path = Self::set_maximize)]
    fn set_maximize(state: &mut State) {
        state.input.window_set.maximize = Some(());
    }

    #[rune::function(path = Self::set_focus)]
    fn set_focus(state: &mut State) {
        state.input.window_set.focus = Some(());
    }

    #[rune::function(path = Self::set_point)]
    fn set_point(state: &mut State, point: &Vec2) {
        state.input.window_set.point = Some(*point);
    }

    #[rune::function(path = Self::set_name)]
    fn set_name(state: &mut State, name: &str) {
        state.input.window_set.name = Some(name.to_string());
    }

    #[rune::function(path = Self::set_icon)]
    fn set_icon(state: &mut State, icon: &str) {
        state.input.window_set.icon = Some(icon.to_string());
    }

    #[rune::function(path = Self::set_full)]
    fn set_full(state: &mut State, window: bool) {
        state.input.window_set.full = Some(window);
    }

    #[rune::function(path = Self::set_scale_min)]
    fn set_scale_min(state: &mut State, scale_min: &Vec2) {
        state.input.window_set.scale_min = Some(*scale_min);
    }

    #[rune::function(path = Self::set_scale_max)]
    fn set_scale_max(state: &mut State, scale_max: &Vec2) {
        state.input.window_set.scale_max = Some(*scale_max);
    }

    #[rune::function(path = Self::set_scale)]
    fn set_scale(state: &mut State, scale: &Vec2) {
        state.input.window_set.scale = Some(*scale);
    }
}

//================================================================

#[rune::module(::video)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    Frame::module(&mut module)?;
    Shader::module(&mut module)?;
    Camera::module(&mut module)?;
    Render::module(&mut module)?;
    Image::module(&mut module)?;
    Font::module(&mut module)?;
    Window::module(&mut module)?;

    Ok(module)
}
