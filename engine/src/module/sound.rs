use crate::module::archive::*;
use crate::module::general::*;
use engine_macro::*;

//================================================================

use raylib::prelude::*;

//================================================================

#[rustfmt::skip]
#[module(name = "sound", info = "Sound API.")]
pub fn set_global(lua: &mlua::Lua, global: &mlua::Table) -> anyhow::Result<()> {
    let sound = lua.create_table()?;

    sound.set("get_master_volume", lua.create_function(self::get_master_volume)?)?;
    sound.set("set_master_volume", lua.create_function(self::set_master_volume)?)?;
    sound.set("new",               lua.create_function(self::Sound::new)?)?;
    sound.set("new_archive",       lua.create_function(self::Sound::new_archive)?)?;

    global.set("sound", sound)?;

    Ok(())
}

//================================================================

#[function(
    from = "sound",
    info = "Get the current master volume.",
    result(name = "volume", info = "Master volume.", kind = "number")
)]
fn get_master_volume(_: &mlua::Lua, _: ()) -> mlua::Result<f32> {
    Ok(unsafe { ffi::GetMasterVolume() })
}

#[function(
    from = "sound",
    info = "Set the current master volume. Will affect both sound and music.",
    parameter(name = "volume", info = "Master volume.", kind = "number")
)]
fn set_master_volume(_: &mlua::Lua, volume: f32) -> mlua::Result<()> {
    unsafe {
        ffi::SetMasterVolume(volume);

        Ok(())
    }
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
            kind(user_data(name = "Sound"))
        )
    )]
    fn new(_: &mlua::Lua, path: String) -> mlua::Result<Self> {
        unsafe {
            let inner = ffi::LoadSound(c_string(&path)?.as_ptr());

            if ffi::IsSoundValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::external(format!(
                    "sound.new(): Error loading sound \"{path}\"."
                )))
            }
        }
    }

    #[function(
        from = "sound",
        info = "Create a new Sound resource from an archive.",
        parameter(name = "path", info = "Path to sound.", kind = "string"),
        parameter(
            name = "archive",
            info = "Archive to load the asset from.",
            kind(user_data(name = "Archive"))
        ),
        result(
            name = "sound",
            info = "Sound resource.",
            kind(user_data(name = "Sound"))
        )
    )]
    fn new_archive(
        _: &mlua::Lua,
        (path, archive): (String, mlua::AnyUserData),
    ) -> mlua::Result<Self> {
        let (data, extension) = Archive::borrow_file(&path, archive)?;

        unsafe {
            let inner = ffi::LoadWaveFromMemory(
                c_string(&extension)?.as_ptr(),
                data.as_ptr(),
                data.len() as i32,
            );
            let inner = ffi::LoadSoundFromWave(inner);

            if ffi::IsSoundValid(inner) {
                Ok(Self { inner })
            } else {
                Err(mlua::Error::external(format!(
                    "sound.new_archive(): Error loading sound \"{path}\"."
                )))
            }
        }
    }

    #[method(from = "Sound", info = "Play sound.")]
    fn play(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::PlaySound(this.inner);
            Ok(())
        }
    }

    #[method(from = "Sound", info = "Stop sound.")]
    fn stop(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::StopSound(this.inner);
            Ok(())
        }
    }

    #[method(from = "Sound", info = "Pause sound.")]
    fn pause(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::PauseSound(this.inner);
            Ok(())
        }
    }

    #[method(from = "Sound", info = "Resume sound.")]
    fn resume(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<()> {
        unsafe {
            ffi::ResumeSound(this.inner);
            Ok(())
        }
    }

    #[method(
        from = "Sound",
        info = "Get the current play state.",
        result(name = "state", info = "Current play state.", kind = "boolean")
    )]
    fn is_play(_: &mlua::Lua, this: &Self, _: ()) -> mlua::Result<bool> {
        unsafe { Ok(ffi::IsSoundPlaying(this.inner)) }
    }

    #[method(
        from = "Sound",
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
        from = "Sound",
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
        from = "Sound",
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
        method.add_method("play",       Self::play);
        method.add_method("stop",       Self::stop);
        method.add_method("pause",      Self::pause);
        method.add_method("resume",     Self::resume);
        method.add_method("is_play",    Self::is_play);
        method.add_method("set_volume", Self::set_volume);
        method.add_method("set_pitch",  Self::set_pitch);
        method.add_method("set_pan",    Self::set_pan);
    }
}
