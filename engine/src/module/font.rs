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

use crate::module::general::*;
use engine_macro::*;

//================================================================

use mlua::prelude::*;
use raylib::prelude::*;

//================================================================

#[module(name = "font", info = "Font API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let font = lua.create_table()?;

    font.set("new", lua.create_function(self::Font::new)?)?;

    global.set("font", font)?;

    Ok(())
}

//================================================================

#[class(info = "Font class.")]
struct Font {
    inner: ffi::Font,
}

impl Font {
    #[function(
        from = "font",
        info = "Create a new font resource.",
        parameter(name = "path", info = "Path to font.", kind = "string"),
        parameter(name = "scale", info = "Fotn scale.", kind = "number"),
        result(name = "font", info = "Font resource.", kind(user_data(name = "Font")))
    )]
    fn new(
        lua: &mlua::Lua,
        (path, scale, code_point_range): (String, i32, Option<mlua::Value>),
    ) -> mlua::Result<Self> {
        unsafe {
            let mut range = Vec::new();

            if let Some(code_point_range) = code_point_range {
                let code_point_range: Vec<(i32, i32)> = lua.from_value(code_point_range)?;

                for i in code_point_range {
                    let current_range = i.0..=i.1;
                    let current_range: Vec<i32> = current_range.collect();

                    range.extend(current_range);
                }
            };

            let pointer = if range.is_empty() {
                std::ptr::null_mut()
            } else {
                range.as_mut_ptr()
            };

            let inner =
                ffi::LoadFontEx(c_string(&path).as_ptr(), scale, pointer, range.len() as i32);

            if ffi::IsFontValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::runtime(format!(
                    "font.new(): Error loading font \"{path}\"."
                )))
            }
        }
    }

    #[method(
        from = "Font",
        info = "Draw text.",
        parameter(name = "text", info = "Text to draw.", kind = "string"),
        parameter(name = "point", info = "Point of text to draw.", kind = "Vector2"),
        parameter(name = "scale", info = "Scale of text to draw.", kind = "number"),
        parameter(name = "space", info = "Space of text to draw.", kind = "number"),
        parameter(name = "color", info = "Color of text to draw.", kind = "Color")
    )]
    fn draw(
        lua: &mlua::Lua,
        this: &Self,
        (text, point, scale, space, color): (String, mlua::Value, f32, f32, mlua::Value),
    ) -> mlua::Result<()> {
        unsafe {
            let point: Vector2 = lua.from_value(point)?;
            let color: Color = lua.from_value(color)?;

            ffi::DrawTextEx(
                this.inner,
                c_string(&text).as_ptr(),
                point.into(),
                scale,
                space,
                color.into(),
            );
            Ok(())
        }
    }

    // Original code from: https://www.raylib.com/examples/text/loader.html?name=text_rectangle_bounds
    #[method(
        from = "Font",
        info = "Draw text with text wrap.",
        parameter(name = "text", info = "Text to draw.", kind = "string"),
        parameter(
            name = "box_2",
            info = "Constraint area of text to draw.",
            kind = "Box2"
        ),
        parameter(name = "scale", info = "Scale of text to draw.", kind = "number"),
        parameter(name = "space", info = "Space of text to draw.", kind = "number"),
        parameter(name = "color", info = "Color of text to draw.", kind = "Color"),
        result(
            name = "shift",
            info = "Amount of vertical line shifting.",
            kind = "number"
        )
    )]
    fn draw_wrap(
        lua: &mlua::Lua,
        this: &Self,
        (text, box_2, scale, space, color): (String, mlua::Value, f32, f32, mlua::Value),
    ) -> mlua::Result<f32> {
        let box_2: Box2 = lua.from_value(box_2)?;
        let color: Color = lua.from_value(color)?;

        let length: i32 = text.len() as i32;
        let text = std::ffi::CString::new(text).unwrap();
        let text = text.as_ptr();

        let mut text_shift_y: f32 = 0.0;
        let mut text_shift_x: f32 = 0.0;

        let scale_factor: f32 = scale / this.inner.baseSize as f32;

        const MEASURE_STATE: i32 = 0;
        const DRAW_STATE: i32 = 1;
        let mut state: i32 = MEASURE_STATE;

        let mut start_line: i32 = -1;
        let mut end_line: i32 = -1;
        let mut last_k: i32 = -1;

        let mut i: i32 = 0;
        let mut k: i32 = 0;

        while i < length {
            let mut code_point_byte_count: i32 = 0;
            let codepoint: i32 =
                unsafe { ffi::GetCodepoint(text.offset(i as isize), &mut code_point_byte_count) };

            let index: i32 = unsafe { ffi::GetGlyphIndex(this.inner, codepoint) };

            if codepoint == 0x3f {
                code_point_byte_count = 1;
            }

            i += code_point_byte_count - 1;

            let mut glyph_width: f32 = 0.0;

            if codepoint != '\n' as i32 {
                let glyph = unsafe { *this.inner.glyphs.offset(index as isize) };
                let rec_glyph = unsafe { *this.inner.recs.offset(index as isize) };

                glyph_width = if glyph.advanceX == 0 {
                    rec_glyph.width * scale_factor
                } else {
                    glyph.advanceX as f32 * scale_factor
                };

                if i + 1 < length {
                    glyph_width += space;
                }
            }

            if state == MEASURE_STATE {
                if codepoint == ' ' as i32 || codepoint == '\t' as i32 || codepoint == '\n' as i32 {
                    end_line = i;
                }

                if text_shift_x + glyph_width > box_2.s_x {
                    end_line = if end_line < 1 { i } else { end_line };

                    if i == end_line {
                        end_line -= code_point_byte_count;
                    }

                    if start_line + code_point_byte_count == end_line {
                        end_line = i - code_point_byte_count;
                    }

                    state = DRAW_STATE;
                } else if i + 1 == length {
                    end_line = i;
                    state = DRAW_STATE;
                } else if codepoint == '\n' as i32 {
                    state = DRAW_STATE;
                }

                if state == DRAW_STATE {
                    text_shift_x = 0.0;
                    i = start_line;
                    glyph_width = 0.0;

                    let tmp = last_k;
                    last_k = k - 1;
                    k = tmp;
                }
            } else {
                if codepoint != '\n' as i32 {
                    if text_shift_y + this.inner.baseSize as f32 * scale_factor > box_2.s_y {
                        break;
                    }

                    if codepoint != ' ' as i32 && codepoint != '\t' as i32 {
                        unsafe {
                            ffi::DrawTextCodepoint(
                                this.inner,
                                codepoint,
                                Vector2 {
                                    x: box_2.p_x + text_shift_x,
                                    y: box_2.p_y + text_shift_y,
                                }
                                .into(),
                                scale,
                                color.into(),
                            );
                        }
                    }
                }

                if i == end_line {
                    // 2.0 is to roughly be in par with the default text line spacing
                    text_shift_y += (this.inner.baseSize as f32 + 2.0) * scale_factor;
                    text_shift_x = 0.0;
                    start_line = end_line;
                    end_line = -1;
                    glyph_width = 0.0;
                    k = last_k;

                    state = MEASURE_STATE;
                }
            }

            if text_shift_x != 0.0 || codepoint != ' ' as i32 {
                text_shift_x += glyph_width;
            }

            i += 1;
            k += 1;
        }

        Ok(text_shift_y)
    }

    #[method(
        from = "Font",
        info = "Calculate the scale of text.",
        parameter(name = "text", info = "Text to evaluate.", kind = "string"),
        parameter(name = "scale", info = "Scale of text to evaluate.", kind = "number"),
        parameter(name = "space", info = "Space of text to evaluate.", kind = "number"),
        result(name = "scale", info = "Scale of text.", kind = "Vector2")
    )]
    fn measure(
        lua: &mlua::Lua,
        this: &Self,
        (text, scale, space): (String, f32, f32),
    ) -> mlua::Result<mlua::Value> {
        unsafe {
            lua.to_value(&Vector2::from(ffi::MeasureTextEx(
                this.inner,
                c_string(&text).as_ptr(),
                scale,
                space,
            )))
        }
    }

    // Original code from: https://www.raylib.com/examples/text/loader.html?name=text_rectangle_bounds
    #[method(
        from = "Font",
        info = "Calculate the scale of text, with text wrap.",
        parameter(name = "text", info = "Text to evaluate.", kind = "string"),
        parameter(
            name = "box_2",
            info = "Constraint area of text to draw.",
            kind = "Box2"
        ),
        parameter(name = "scale", info = "Scale of text to evaluate.", kind = "number"),
        parameter(name = "space", info = "Space of text to evaluate.", kind = "number"),
        result(
            name = "shift",
            info = "Amount of vertical line shifting.",
            kind = "number"
        )
    )]
    fn measure_wrap(
        lua: &mlua::Lua,
        this: &Self,
        (text, box_2, scale, space): (String, mlua::Value, f32, f32),
    ) -> mlua::Result<f32> {
        let box_2: Box2 = lua.from_value(box_2)?;

        let length: i32 = text.len() as i32;
        let text = std::ffi::CString::new(text).unwrap();
        let text = text.as_ptr();

        let mut text_shift_y: f32 = 0.0;
        let mut text_shift_x: f32 = 0.0;

        let scale_factor: f32 = scale / this.inner.baseSize as f32;

        const MEASURE_STATE: i32 = 0;
        const DRAW_STATE: i32 = 1;
        let mut state: i32 = MEASURE_STATE;

        let mut start_line: i32 = -1;
        let mut end_line: i32 = -1;
        let mut last_k: i32 = -1;

        let mut i: i32 = 0;
        let mut k: i32 = 0;

        while i < length {
            let mut code_point_byte_count: i32 = 0;
            let codepoint: i32 =
                unsafe { ffi::GetCodepoint(text.offset(i as isize), &mut code_point_byte_count) };

            let index: i32 = unsafe { ffi::GetGlyphIndex(this.inner, codepoint) };

            if codepoint == 0x3f {
                code_point_byte_count = 1;
            }

            i += code_point_byte_count - 1;

            let mut glyph_width: f32 = 0.0;

            if codepoint != '\n' as i32 {
                let glyph = unsafe { *this.inner.glyphs.offset(index as isize) };
                let rec_glyph = unsafe { *this.inner.recs.offset(index as isize) };

                glyph_width = if glyph.advanceX == 0 {
                    rec_glyph.width * scale_factor
                } else {
                    glyph.advanceX as f32 * scale_factor
                };

                if i + 1 < length {
                    glyph_width += space;
                }
            }

            if state == MEASURE_STATE {
                if codepoint == ' ' as i32 || codepoint == '\t' as i32 || codepoint == '\n' as i32 {
                    end_line = i;
                }

                if text_shift_x + glyph_width > box_2.s_x {
                    end_line = if end_line < 1 { i } else { end_line };

                    if i == end_line {
                        end_line -= code_point_byte_count;
                    }

                    if start_line + code_point_byte_count == end_line {
                        end_line = i - code_point_byte_count;
                    }

                    state = DRAW_STATE;
                } else if i + 1 == length {
                    end_line = i;
                    state = DRAW_STATE;
                } else if codepoint == '\n' as i32 {
                    state = DRAW_STATE;
                }

                if state == DRAW_STATE {
                    text_shift_x = 0.0;
                    i = start_line;
                    glyph_width = 0.0;

                    let tmp = last_k;
                    last_k = k - 1;
                    k = tmp;
                }
            } else {
                if codepoint != '\n' as i32 {
                    if text_shift_y + this.inner.baseSize as f32 * scale_factor > box_2.s_y {
                        break;
                    }

                    /*
                    if codepoint != ' ' as i32 && codepoint != '\t' as i32 {
                        unsafe {
                            ffi::DrawTextCodepoint(
                                this.inner,
                                codepoint,
                                Vector2 {
                                    x: box_2.p_x + text_shift_x,
                                    y: box_2.p_y + text_shift_y,
                                }
                                .into(),
                                scale,
                                color.into(),
                            );
                        }
                    }
                    */
                }

                if i == end_line {
                    // 2.0 is to roughly be in par with the default text line spacing
                    text_shift_y += (this.inner.baseSize as f32 + 2.0) * scale_factor;
                    text_shift_x = 0.0;
                    start_line = end_line;
                    end_line = -1;
                    glyph_width = 0.0;
                    k = last_k;

                    state = MEASURE_STATE;
                }
            }

            if text_shift_x != 0.0 || codepoint != ' ' as i32 {
                text_shift_x += glyph_width;
            }

            i += 1;
            k += 1;
        }

        Ok(text_shift_y)
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadFont(self.inner);
        }
    }
}

impl mlua::UserData for Font {
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("draw", |lua, this, (text, point, scale, space, color)| {
            Self::draw(lua, this, (text, point, scale, space, color))
        });
        method.add_method(
            "draw_wrap",
            |lua, this, (text, box_2, scale, space, color)| {
                Self::draw_wrap(lua, this, (text, box_2, scale, space, color))
            },
        );
        method.add_method("measure", |lua, this, (text, scale, space)| {
            Self::measure(lua, this, (text, scale, space))
        });
        method.add_method("measure_wrap", |lua, this, (text, box_2, scale, space)| {
            Self::measure_wrap(lua, this, (text, box_2, scale, space))
        });
    }
}
