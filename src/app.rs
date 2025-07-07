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

use libloading::{Library, Symbol};
use rune::{
    Any, Diagnostics, Module, Source, Sources, Value, Vm,
    runtime::Function,
    termcolor::{ColorChoice, StandardStream},
};
use std::sync::Arc;
use three_d::{Event, SurfaceSettings};

//================================================================

pub struct App {}

impl App {
    fn load_script() -> anyhow::Result<(Vec<Library>, Sources, Vm)> {
        println!();
        println!("//================================================================");
        println!("//  Laravox (1.0.0)");
        println!("//================================================================");
        println!();

        let mut module = Module::new();
        module.ty::<Entry>()?;
        module.ty::<Window>()?;

        // install each module into the Rune context.
        let mut context = rune_modules::default_context()?;

        let library_list = Vec::new();

        /*
        unsafe {
            let library = Library::new("/home/think/laravox/librune_library.so").unwrap();
            let get_module: Symbol<fn() -> Module> = library.get(b"module").unwrap();

            let result = get_module();

            context.install(result)?;

            library_list.push(library);
        }
        */

        context.install(module)?;
        context.install(crate::video::module()?)?;
        context.install(crate::audio::module()?)?;
        context.install(crate::input::module()?)?;
        context.install(crate::general::module()?)?;

        let runtime = Arc::new(context.runtime()?);

        let mut sources = Sources::new();

        sources.insert(Source::from_path("script/main.rn")?)?;

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            // if not ColorChoice::Never, will bug out on Sublime's terminal
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            diagnostics.emit(&mut writer, &sources)?;
        }

        let unit = result?;
        let unit = Arc::new(unit);

        let vm = Vm::new(runtime.clone(), unit.clone());

        Ok((library_list, sources, vm))
    }

    fn load_window(entry: &Entry) -> anyhow::Result<three_d::Window> {
        let window: Window = rune::from_value(entry.window.call::<Value>(()).into_result()?)?;

        Ok(three_d::Window::new(three_d::WindowSettings {
            title: window.title.unwrap_or("Laravox".to_string()),
            min_size: window.min_scale.unwrap_or((640, 480)),
            max_size: window.max_scale,
            initial_size: window.scale,
            borderless: window.full.unwrap_or(false),
            surface_settings: SurfaceSettings {
                vsync: true,
                ..Default::default()
            },
        })?)
    }

    fn load_value(
        entry: &Entry,
        frame_input: three_d::FrameInput,
        handle: rodio::OutputStreamHandle,
    ) -> anyhow::Result<Value> {
        let state = State::new(frame_input, FrameState::new(), handle);

        Ok(entry.begin.call::<Value>((state,)).into_result()?)
    }

    pub fn run() -> anyhow::Result<()> {
        let (_stream, handle) = rodio::OutputStream::try_default()?;

        let (library_list, mut source, mut script) = Self::load_script()?;
        let mut entry = Entry::new(&mut script)?;
        let window = Self::load_window(&entry)?;
        let mut frame_state = FrameState::new();
        let mut value = None;

        window.render_loop(move |frame_input| {
            if value.is_none() {
                value =
                    Some(Self::load_value(&entry, frame_input.clone(), handle.clone()).unwrap());
            }

            let v = value.as_ref().unwrap();

            frame_state.process(&frame_input.events);

            //println!("{}", 1.0 / (frame_input.elapsed_time / 1000.0));

            let state = State::new(frame_input.clone(), frame_state, handle.clone());

            let call = entry.frame.call::<usize>((v, &state)).into_result();

            match call {
                Ok(code) => {
                    if code == 0 {
                        entry.close.call::<()>((v, &state)).into_result().unwrap();

                        three_d::FrameOutput {
                            exit: true,
                            swap_buffers: false,
                            wait_next_event: false,
                        }
                    } else if code == 1 {
                        entry.close.call::<()>((v, &state)).into_result().unwrap();

                        let (c_library_list, c_source, mut c_script) = Self::load_script().unwrap();
                        let c_entry = Entry::new(&mut c_script).unwrap();

                        source = c_source;
                        entry = c_entry;

                        value = Some(
                            Self::load_value(&entry, frame_input.clone(), handle.clone()).unwrap(),
                        );

                        three_d::FrameOutput::default()
                    } else {
                        three_d::FrameOutput::default()
                    }
                }
                Err(error) => {
                    entry.close.call::<()>((v, &state)).into_result().unwrap();

                    let mut writer = StandardStream::stderr(ColorChoice::Auto);
                    error.emit(&mut writer, &source).unwrap();
                    three_d::FrameOutput {
                        exit: true,
                        swap_buffers: false,
                        wait_next_event: false,
                    }
                }
            }
        });

        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {}
}

//================================================================

#[derive(Any)]
pub struct State {
    pub frame_input: three_d::FrameInput,
    pub frame_state: FrameState,
    pub audio_entry: rodio::OutputStreamHandle,
}

impl State {
    pub fn new(
        frame_input: three_d::FrameInput,
        frame_state: FrameState,
        audio_entry: rodio::OutputStreamHandle,
    ) -> Self {
        Self {
            frame_input,
            frame_state,
            audio_entry,
        }
    }
}

//================================================================

#[derive(Clone, Copy, Debug)]
pub struct FrameState {
    pub board: [InputState; Self::BUTTON_COUNT_BOARD],
    pub mouse: [InputState; Self::BUTTON_COUNT_MOUSE],
}

impl FrameState {
    const BUTTON_COUNT_BOARD: usize = 51;
    const BUTTON_COUNT_MOUSE: usize = 3;

    fn new() -> Self {
        Self {
            board: [InputState::default(); Self::BUTTON_COUNT_BOARD],
            mouse: [InputState::default(); Self::BUTTON_COUNT_MOUSE],
        }
    }

    fn process(&mut self, event_list: &[three_d::Event]) {
        for button in &mut self.board {
            button.press = false;
            button.release = false;
        }

        for button in &mut self.mouse {
            button.press = false;
            button.release = false;
        }

        for event in event_list {
            match event {
                Event::MousePress { button, .. } => {
                    self.mouse[*button as usize].up = false;
                    self.mouse[*button as usize].press = true;
                }
                Event::MouseRelease { button, .. } => {
                    self.mouse[*button as usize].up = true;
                    self.mouse[*button as usize].release = true;
                }
                Event::KeyPress { kind, .. } => {
                    self.board[*kind as usize].up = false;
                    self.board[*kind as usize].press = true;
                }
                Event::KeyRelease { kind, .. } => {
                    self.board[*kind as usize].up = true;
                    self.board[*kind as usize].release = true;
                }
                _ => {}
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct InputState {
    pub up: bool,
    pub press: bool,
    pub release: bool,
}

//================================================================

#[derive(Any)]
#[rune(constructor)]
pub struct Entry {
    window: Function,
    begin: Function,
    close: Function,
    frame: Function,
}

impl Entry {
    fn new(rune: &mut Vm) -> anyhow::Result<Self> {
        Ok(Self {
            window: rune.lookup_function(["Main", "window"])?,
            begin: rune.lookup_function(["Main", "begin"])?,
            close: rune.lookup_function(["Main", "close"])?,
            frame: rune.lookup_function(["Main", "frame"])?,
        })
    }
}

//================================================================

#[derive(Any)]
#[rune(constructor)]
pub struct Window {
    pub title: Option<String>,
    pub min_scale: Option<(u32, u32)>,
    pub max_scale: Option<(u32, u32)>,
    pub scale: Option<(u32, u32)>,
    pub full: Option<bool>,
}
