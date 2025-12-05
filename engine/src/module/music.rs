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

#[rustfmt::skip]
#[module(name = "music", info = "Music API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let music = lua.create_table()?;

    music.set("new", lua.create_function(self::Music::new)?)?;

    global.set("music", music)?;

    Ok(())
}

#[class(info = "Music class.")]
struct Music {
    inner: ffi::Music,
}

impl Music {
    #[function(
        from = "music",
        info = "Create a new Music resource.",
        parameter(name = "path", info = "Path to music.", kind = "string"),
        result(
            name = "music",
            info = "Music resource.",
            kind(user_data(name = "music"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        unsafe {
            let inner = ffi::LoadMusicStream(c_string(&path).as_ptr());

            if ffi::IsMusicValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::runtime(format!(
                    "music.new(): Error loading music \"{path}\"."
                )))
            }
        }
    }

    #[method(from = "music", info = "Update music.")]
    fn update(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::UpdateMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "music", info = "Play music.")]
    fn play(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::PlayMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "music", info = "Stop music.")]
    fn stop(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::StopMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "music", info = "Stop music.")]
    fn pause(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::PauseMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "music", info = "Stop music.")]
    fn resume(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::ResumeMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(
        from = "music",
        info = "Get the current play state.",
        result(name = "state", info = "Current play state.", kind = "boolean")
    )]
    fn get_play(_: &mlua::Lua, this: &Self) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsMusicStreamPlaying(this.inner)) }
    }

    #[method(
        from = "music",
        info = "Get the total length.",
        result(name = "length", info = "Total length.", kind = "number")
    )]
    fn get_length(_: &mlua::Lua, this: &Self) -> mlua::Result<f32> {
        unsafe { Ok(ffi::GetMusicTimeLength(this.inner)) }
    }

    #[method(
        from = "music",
        info = "Get the current time.",
        result(name = "time", info = "Current time.", kind = "number")
    )]
    fn get_time(_: &mlua::Lua, this: &Self) -> mlua::Result<f32> {
        unsafe { Ok(ffi::GetMusicTimePlayed(this.inner)) }
    }

    #[method(
        from = "music",
        info = "Set the time of the music.",
        parameter(name = "time", info = "Time value.", kind = "number")
    )]
    fn set_time(_: &mlua::Lua, this: &Self, time: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SeekMusicStream(this.inner, time);
            Ok(())
        }
    }

    #[method(
        from = "music",
        info = "Set the volume of the music.",
        parameter(name = "volume", info = "Volume value.", kind = "number")
    )]
    fn set_volume(_: &mlua::Lua, this: &Self, volume: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetMusicVolume(this.inner, volume);
            Ok(())
        }
    }

    #[method(
        from = "music",
        info = "Set the pitch of the music.",
        parameter(name = "pitch", info = "Pitch value.", kind = "number")
    )]
    fn set_pitch(_: &mlua::Lua, this: &Self, pitch: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetMusicPitch(this.inner, pitch);
            Ok(())
        }
    }

    #[method(
        from = "music",
        info = "Set the pan of the music.",
        parameter(name = "pan", info = "Pan value.", kind = "number")
    )]
    fn set_pan(_: &mlua::Lua, this: &Self, pan: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetMusicPan(this.inner, pan);
            Ok(())
        }
    }
}

impl Drop for Music {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadMusicStream(self.inner);
        }
    }
}

impl mlua::UserData for Music {
    #[rustfmt::skip]
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("update",     |lua, this, _: ()|       Self::update(lua, this));
        method.add_method("play",       |lua, this, _: ()|       Self::play(lua, this));
        method.add_method("stop",       |lua, this, _: ()|       Self::stop(lua, this));
        method.add_method("pause",      |lua, this, _: ()|       Self::pause(lua, this));
        method.add_method("resume",     |lua, this, _: ()|       Self::resume(lua, this));
        method.add_method("get_play",   |lua, this, _: ()|       Self::get_play(lua, this));
        method.add_method("get_length", |lua, this, _: ()|       Self::get_length(lua, this));
        method.add_method("get_time",   |lua, this, _: ()|       Self::get_time(lua, this));
        method.add_method("set_time",   |lua, this, time: f32  | Self::set_time(lua, this, time));
        method.add_method("set_volume", |lua, this, volume: f32| Self::set_volume(lua, this, volume));
        method.add_method("set_pitch",  |lua, this, pitch: f32 | Self::set_pitch(lua, this, pitch));
        method.add_method("set_pan",    |lua, this, pan: f32   | Self::set_pan(lua, this, pan));
    }
}
