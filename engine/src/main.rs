mod module;

//================================================================

use mimalloc::MiMalloc;
use mlua::prelude::*;
use raylib::prelude::*;
use serde::Deserialize;
use std::io::Read;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

//================================================================

#[allow(dead_code)]
struct Context {
    handle: RaylibHandle,
    thread: RaylibThread,
    audio: RaylibAudio,
}

#[derive(Deserialize)]
struct ContextInfo {
    icon: Option<String>,
    name: Option<String>,
    scale: (i32, i32),
    sync: bool,
    full: bool,
    rate: u32,
    log: bool,
}

impl Context {
    fn new(script: &Script) -> anyhow::Result<Self> {
        let info = script.info.call::<mlua::Value>(())?;
        let info: ContextInfo = script.lua.from_value(info)?;

        let mut flag = 0;

        if info.sync {
            flag += ConfigFlags::FLAG_VSYNC_HINT as u32;
        }
        if info.full {
            flag += ConfigFlags::FLAG_FULLSCREEN_MODE as u32;
        }

        unsafe {
            ffi::SetConfigFlags(flag);
        }

        let (mut handle, thread) = raylib::init()
            .size(info.scale.0, info.scale.1)
            .title(&info.name.unwrap_or("Laravox".to_string()))
            .resizable()
            .log_level(if info.log {
                TraceLogLevel::LOG_ALL
            } else {
                TraceLogLevel::LOG_NONE
            })
            .build();

        handle.set_exit_key(None);
        handle.set_target_fps(info.rate);

        if let Some(icon) = &info.icon {
            handle.set_window_icon(Image::load_image(icon)?);
        }

        let audio = raylib::audio::RaylibAudio::init_audio_device()?;

        script.set_global(true)?;

        Ok(Self {
            handle,
            thread,
            audio,
        })
    }
}

//================================================================

enum ScriptState {
    Success,
    Failure(String),
}

struct Script {
    lua: Lua,
    state: ScriptState,
    table: mlua::Table,
    info: mlua::Function,
    main: mlua::Function,
    fail: mlua::Function,
}

impl Script {
    const MAIN_PATH: &str = "main/main";
    const MAIN_FILE: &str = "main";
    const ENTRY_INFO: &str = "info";
    const ENTRY_MAIN: &str = "main";
    const ENTRY_FAIL: &str = "fail";
    const HOOK_NAME: &str = "laravox";

    fn new(set_window_global: bool) -> anyhow::Result<Self> {
        let lua = unsafe { Lua::unsafe_new() };

        let main = std::path::Path::new(Self::MAIN_FILE);

        if main.is_file() {
            let global = lua.globals();
            let loader = global.get::<mlua::Table>("package")?;
            let loader = loader.get::<mlua::Table>("loaders")?;

            let file = std::fs::File::open(Self::MAIN_FILE)?;
            let mut file = zip::ZipArchive::new(file)?;

            loader.push(lua.create_function_mut(move |lua, path: String| {
                let token: Vec<&str> = path.split(&format!("{}/", Self::MAIN_FILE)).collect();

                if let Some(path) = token.get(1)
                    && let Ok(mut entry) = file.by_name(&format!("{path}.lua"))
                {
                    let mut buffer = String::new();
                    entry.read_to_string(&mut buffer)?;
                    return Ok(lua.load(buffer).into_function());
                }

                Err(mlua::Error::external(format!(
                    "No module \"{path}\" found in the \"main\" ZIP archive."
                )))
            })?)?;
        }

        let table: mlua::Table = lua
            .load(format!("require(\"{}\")", Self::MAIN_PATH))
            .eval()?;
        let info = table.get(Self::ENTRY_INFO)?;
        let main = table.get(Self::ENTRY_MAIN)?;
        let fail = table.get(Self::ENTRY_FAIL)?;

        let script = Self {
            lua,
            state: ScriptState::Success,
            table,
            info,
            main,
            fail,
        };

        script.set_global(false)?;

        if set_window_global {
            script.set_global(true)?;
        }

        Ok(script)
    }

    fn set_global(&self, window: bool) -> anyhow::Result<()> {
        let global = self.lua.globals();
        let global = if let Ok(global) = global.get::<mlua::Table>(Self::HOOK_NAME) {
            global
        } else {
            let table = self.lua.create_table()?;
            global.set(Self::HOOK_NAME, &table)?;

            table
        };

        if window {
            crate::module::window::set_global(&self.lua, &global)?;
            crate::module::screen::set_global(&self.lua, &global)?;
            crate::module::texture::set_global(&self.lua, &global)?;
            crate::module::font::set_global(&self.lua, &global)?;
            crate::module::sound::set_global(&self.lua, &global)?;
            crate::module::music::set_global(&self.lua, &global)?;
            crate::module::input::set_global(&self.lua, &global)?;
        } else {
            crate::module::data::set_global(&self.lua, &global)?;
            crate::module::archive::set_global(&self.lua, &global)?;
            crate::module::network::set_global(&self.lua, &global)?;

            self.lua.globals().set(
                "print",
                self.lua.create_function(|_, value: mlua::Value| {
                    println!("{value:#?}");

                    Ok(())
                })?,
            )?;
            self.lua.globals().set(
                "format",
                self.lua
                    .create_function(|_, value: mlua::Value| Ok(format!("{value:#?}")))?,
            )?;

            // Add UTF-8 compliant sub-string replacement.
            let string: mlua::Table = self.lua.globals().get("string")?;
            string.set(
                "sub",
                self.lua
                    .create_function(crate::module::general::sub_string)?,
            )?;
        }

        Ok(())
    }
}

//================================================================

fn throw_error<T, E: std::string::ToString + std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            rfd::MessageDialog::new()
                .set_level(rfd::MessageLevel::Error)
                .set_title("Fatal Error")
                .set_description(error.to_string())
                .show();
            panic!("{error:?}")
        }
    }
}

fn main() -> anyhow::Result<()> {
    /*
    let hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic_info| {
        let time = chrono::Local::now();
        let file = format!("panic_{}.txt", time.format("%d_%m_%y_%H_%M_%S"));
        std::fs::write(file, format!("{}", panic_info)).unwrap();

        hook(panic_info);
    }));
    */

    let mut script = throw_error(Script::new(false));
    let _context = throw_error(Context::new(&script));

    loop {
        match script.state {
            ScriptState::Success => {
                let code = script.main.call::<bool>(&script.table);

                if let Err(error) = code {
                    script.state = ScriptState::Failure(error.to_string());
                } else if let Ok(code) = code {
                    if code {
                        let new = Script::new(true);

                        if let Err(error) = new {
                            script.state = ScriptState::Failure(error.to_string());
                        } else if let Ok(new) = new {
                            script = new;
                        }
                    } else {
                        break;
                    }
                }
            }
            ScriptState::Failure(ref error) => {
                let code =
                    throw_error(script.fail.call::<bool>((&script.table, error.to_string())));

                if code {
                    let new = Script::new(true);

                    if let Err(error) = new {
                        script.state = ScriptState::Failure(error.to_string());
                    } else if let Ok(new) = new {
                        script = new;
                    }
                } else {
                    break;
                }
            }
        }
    }

    drop(script);
    drop(_context);

    Ok(())
}
