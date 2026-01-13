use crate::module::general::*;
use engine_macro::*;

//================================================================

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

//================================================================

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
            kind(user_data(name = "Music"))
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

    #[method(from = "Music", info = "Update music.")]
    fn update(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::UpdateMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "Music", info = "Play music.")]
    fn play(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::PlayMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "Music", info = "Stop music.")]
    fn stop(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::StopMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "Music", info = "Pause music.")]
    fn pause(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::PauseMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(from = "Music", info = "Resume music.")]
    fn resume(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::ResumeMusicStream(this.inner);
            Ok(())
        }
    }

    #[method(
        from = "Music",
        info = "Get the current play state.",
        result(name = "state", info = "Current play state.", kind = "boolean")
    )]
    fn get_play(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsMusicStreamPlaying(this.inner)) }
    }

    #[method(
        from = "Music",
        info = "Get the total length.",
        result(name = "length", info = "Total length.", kind = "number")
    )]
    fn get_length(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<f32> {
        unsafe { Ok(ffi::GetMusicTimeLength(this.inner)) }
    }

    #[method(
        from = "Music",
        info = "Get the current time.",
        result(name = "time", info = "Current time.", kind = "number")
    )]
    fn get_time(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<f32> {
        unsafe { Ok(ffi::GetMusicTimePlayed(this.inner)) }
    }

    #[method(
        from = "Music",
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
        from = "Music",
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
        from = "Music",
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
        from = "Music",
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
        method.add_method("update",     Self::update);
        method.add_method("play",       Self::play);
        method.add_method("stop",       Self::stop);
        method.add_method("pause",      Self::pause);
        method.add_method("resume",     Self::resume);
        method.add_method("get_play",   Self::get_play);
        method.add_method("get_length", Self::get_length);
        method.add_method("get_time",   Self::get_time);
        method.add_method("set_time",   Self::set_time);
        method.add_method("set_volume", Self::set_volume);
        method.add_method("set_pitch",  Self::set_pitch);
        method.add_method("set_pan",    Self::set_pan);
    }
}
