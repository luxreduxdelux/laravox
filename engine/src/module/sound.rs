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

use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "sound", info = "Sound API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let sound = lua.create_table()?;

    sound.set("new", lua.create_function(self::Sound::new)?)?;

    global.set("sound", sound)?;

    Ok(())
}

//================================================================

#[class(info = "Sound class.")]
struct Sound {
    inner: ffi::Sound,
}

impl Sound {
    #[function(
        from = "sound",
        info = "Create a new Sound resource.",
        parameter(name = "path", info = "Path to sound.", kind = "string"),
        result(
            name = "sound",
            info = "Sound resource.",
            kind(user_data(name = "sound"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        unsafe {
            let inner = ffi::LoadSound(c_string(&path).as_ptr());

            if ffi::IsSoundValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::runtime(format!(
                    "sound.new(): Error loading sound \"{path}\"."
                )))
            }
        }
    }

    #[method(from = "sound", info = "Play sound.")]
    fn play(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::PlaySound(this.inner);
            Ok(())
        }
    }

    #[method(from = "sound", info = "Stop sound.")]
    fn stop(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::StopSound(this.inner);
            Ok(())
        }
    }

    #[method(from = "sound", info = "Pause sound.")]
    fn pause(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::PauseSound(this.inner);
            Ok(())
        }
    }

    #[method(from = "sound", info = "Resume sound.")]
    fn resume(_: &mlua::Lua, this: &Self) -> mlua::Result<()> {
        unsafe {
            ffi::ResumeSound(this.inner);
            Ok(())
        }
    }

    #[method(
        from = "sound",
        info = "Get the current play state.",
        result(name = "state", info = "Current play state.", kind = "boolean")
    )]
    fn get_play(_: &mlua::Lua, this: &Self) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsSoundPlaying(this.inner)) }
    }

    #[method(
        from = "sound",
        info = "Set the volume of the sound.",
        parameter(name = "volume", info = "Volume value.", kind = "number")
    )]
    fn set_volume(_: &mlua::Lua, this: &Self, volume: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetSoundVolume(this.inner, volume);
            Ok(())
        }
    }

    #[method(
        from = "sound",
        info = "Set the pitch of the sound.",
        parameter(name = "pitch", info = "Pitch value.", kind = "number")
    )]
    fn set_pitch(_: &mlua::Lua, this: &Self, pitch: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetSoundPitch(this.inner, pitch);
            Ok(())
        }
    }

    #[method(
        from = "sound",
        info = "Set the pan of the sound.",
        parameter(name = "pan", info = "Pan value.", kind = "number")
    )]
    fn set_pan(_: &mlua::Lua, this: &Self, pan: f32) -> mlua::Result<()> {
        unsafe {
            ffi::SetSoundPan(this.inner, pan);
            Ok(())
        }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            ffi::UnloadSound(self.inner);
        }
    }
}

impl mlua::UserData for Sound {
    #[rustfmt::skip]
    fn add_methods<M: mlua::UserDataMethods<Self>>(method: &mut M) {
        method.add_method("play",       |lua, this, _: ()|       Self::play(lua, this));
        method.add_method("stop",       |lua, this, _: ()|       Self::stop(lua, this));
        method.add_method("pause",      |lua, this, _: ()|       Self::pause(lua, this));
        method.add_method("resume",     |lua, this, _: ()|       Self::resume(lua, this));
        method.add_method("get_play",   |lua, this, _: ()|       Self::get_play(lua, this));
        method.add_method("set_volume", |lua, this, volume: f32| Self::set_volume(lua, this, volume));
        method.add_method("set_pitch",  |lua, this, pitch: f32 | Self::set_pitch(lua, this, pitch));
        method.add_method("set_pan",    |lua, this, pan: f32   | Self::set_pan(lua, this, pan));
    }
}
