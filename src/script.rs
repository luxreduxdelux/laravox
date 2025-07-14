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

use crate::module::general::Vec2;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher, event};
use rodio::{OutputStream, OutputStreamHandle};
use rune::{
    Any,
    Context,
    Diagnostics,
    FromValue,
    Module,
    Source,
    Sources,
    Value,
    Vm,
    //compile::ErrorKind,
    runtime::{Function, GuardedArgs},
    termcolor::{Buffer, ColorChoice, StandardStream},
};
use std::sync::{Arc, mpsc::Receiver};
use three_d::{FrameInput, FrameInputGenerator};
use winit::event_loop::ControlFlow;

//================================================================

pub struct Script {
    /// Rune virtual machine handle.
    #[allow(dead_code)]
    pub handle: Option<Handle>,
    /// Rune context.
    #[allow(dead_code)]
    context: Context,
    /// Rune state.
    value: Option<Value>,
    /// Rust state.
    pub state: Option<State>,
    /// Rune error.
    pub error: Option<String>,
}

impl Script {
    pub fn new() -> anyhow::Result<Self> {
        // get the Rune context, with the Laravox/Rune standard library.
        let mut context = rune_modules::default_context()?;

        let mut module = Module::new();
        module.ty::<Window>()?;

        context.install(module)?;
        context.install(crate::module::general::module()?)?;
        context.install(crate::module::video::module()?)?;
        context.install(crate::module::audio::module()?)?;
        context.install(crate::module::input::module()?)?;
        context.install(crate::module::file::module()?)?;
        context.install(crate::module::physical::module()?)?;

        //================================================================

        // get the Rune compilation unit, with the Rune source and context.
        let compile = Handle::new(&context);

        // get the Rune handle, or alternatively, the error message, if any compile error was found.
        let (handle, error) = match compile {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error.to_string())),
        };

        Ok(Self {
            handle,
            context,
            value: None,
            state: None,
            error,
        })
    }

    pub fn window(&mut self) -> Window {
        let window = Window::default();

        // we have a handle to the Rune VM.
        if let Some(handle) = &self.handle {
            // call the window entry-point.
            match handle.safe_call(&handle.window, (window,)) {
                Ok(value) => return value,
                Err(error) => {
                    // error in entry-point, return default window.
                    self.error = Some(error.to_string());
                }
            }
        }

        Window::default()
    }

    fn begin(&mut self, frame: FrameInput) {
        if let Some(handle) = &self.handle {
            let mut state = State::new(frame);

            match handle.safe_call(&handle.begin, (&mut state,)) {
                Ok(value) => {
                    self.value = Some(value);
                }
                Err(error) => {
                    self.error = Some(error.to_string());
                }
            }

            self.state = Some(state);
        }
    }

    pub fn frame(&mut self, frame: FrameInput, control_flow: &mut ControlFlow) {
        if let Some(handle) = &self.handle {
            if let Some(value) = &self.value
                && let Some(state) = &mut self.state
            {
                state.frame = frame.clone();

                match handle.safe_call(&handle.frame, (value, state)) {
                    Ok(value) => match value {
                        1 => control_flow.set_exit(),
                        2 => self.rebuild(),
                        3 => self.restart(frame),
                        _ => {}
                    },
                    Err(error) => {
                        self.error = Some(error.to_string());
                    }
                };
            } else {
                self.begin(frame);
            }
        }
    }

    pub fn close(&mut self) {
        if let Some(handle) = &self.handle
            && let Some(value) = &self.value
        {
            match handle.safe_call(&handle.close, (value,)) {
                Ok(value) => value,
                Err(error) => {
                    self.error = Some(error.to_string());
                }
            }
        }
    }

    pub fn rebuild(&mut self) {
        self.error = None;

        match Handle::new(&self.context) {
            Ok(value) => self.handle = Some(value),
            Err(error) => self.error = Some(error.to_string()),
        }
    }

    pub fn restart(&mut self, frame: FrameInput) {
        self.error = None;

        match Handle::new(&self.context) {
            Ok(value) => self.handle = Some(value),
            Err(error) => self.error = Some(error.to_string()),
        }

        self.begin(frame);
    }

    pub fn watch(&mut self) {
        if let Some(handle) = &self.handle
            && let Some((_, rx)) = &handle.watcher
            && let Ok(Ok(event)) = rx.try_recv()
            && event.kind == EventKind::Access(event::AccessKind::Close(event::AccessMode::Write))
        {
            println!("rebuild!");
            self.rebuild();
        }
    }
}

//================================================================

pub struct Handle {
    /// Rune virtual machine.
    #[allow(dead_code)]
    handle: Vm,
    /// Rune source code.
    source: Sources,
    /// entry-point function; retrieve window configuration.
    window: Function,
    /// entry-point function; retrieve Rune state.
    begin: Function,
    /// entry-point function; main loop.
    frame: Function,
    /// entry-point function; Rune state destructor.
    close: Function,
    /// source file watcher.
    watcher: Option<(
        RecommendedWatcher,
        Receiver<Result<notify::Event, notify::Error>>,
    )>,
}

impl Handle {
    const MAIN_PATH: &str = "script/main.rn";
    const MAIN_NAME: &str = "Main";
    const CALL_WINDOW: &str = "window";
    const CALL_BEGIN: &str = "begin";
    const CALL_FRAME: &str = "frame";
    const CALL_CLOSE: &str = "close";

    fn new(context: &Context) -> anyhow::Result<Self> {
        // read the main source file.
        let mut source = Sources::new();
        source.insert(Source::from_path(Self::MAIN_PATH)?)?;

        //================================================================

        let mut diagnostic = Diagnostics::new();

        let unit = rune::prepare(&mut source)
            .with_context(context)
            .with_diagnostics(&mut diagnostic)
            .build();

        // diagnostic may have warning/error data.
        if !diagnostic.is_empty() {
            // a color and plain buffer. color will print out to the standard error stream.
            let mut color = StandardStream::stderr(ColorChoice::Auto);
            let mut plain = Buffer::no_color();

            // write warning/error data.
            diagnostic.emit(&mut color, &source)?;
            diagnostic.emit(&mut plain, &source)?;

            // error found, return error string.
            if diagnostic.has_error() {
                return Err(anyhow::Error::msg(String::from_utf8(plain.into_inner())?));
            }
        }

        //================================================================

        // create a file-system path watcher.
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        let mut watcher_list = watcher.paths_mut();
        let mut watcher_find = 0;

        // iterate through the list of each source file found in the compile stage.
        while let Some(source) = source.get(rune::SourceId::new(watcher_find)) {
            if let Some(path) = source.path() {
                // watch source file.
                watcher_list.add(path, RecursiveMode::NonRecursive)?;
            }

            watcher_find += 1;
        }

        watcher_list.commit()?;

        //================================================================

        // create Rune virtual machine.
        let handle = Vm::new(Arc::new(context.runtime()?), Arc::new(unit?));

        Ok(Self {
            source,
            window: handle.lookup_function([Self::MAIN_NAME, Self::CALL_WINDOW])?,
            begin: handle.lookup_function([Self::MAIN_NAME, Self::CALL_BEGIN])?,
            frame: handle.lookup_function([Self::MAIN_NAME, Self::CALL_FRAME])?,
            close: handle.lookup_function([Self::MAIN_NAME, Self::CALL_CLOSE])?,
            handle,
            watcher: Some((watcher, rx)),
        })
    }

    fn safe_call<A: GuardedArgs, R: FromValue>(
        &self,
        call: &Function,
        argument: A,
    ) -> anyhow::Result<R> {
        match call.call(argument) {
            rune::runtime::VmResult::Ok(value) => Ok(value),
            rune::runtime::VmResult::Err(error) => {
                // a color and plain buffer. color will print out to the standard error stream.
                let mut color = StandardStream::stderr(ColorChoice::Auto);
                let mut plain = Buffer::no_color();

                // write warning/error data.
                error.emit(&mut color, &self.source)?;
                error.emit(&mut plain, &self.source)?;

                Err(anyhow::Error::msg(String::from_utf8(plain.into_inner())?))
            }
        }
    }
}

//================================================================

#[derive(Any)]
pub struct Window {
    #[rune(get, set)]
    pub name: String,
    #[rune(get, set)]
    pub icon: Option<String>,
    #[rune(get, set)]
    pub scale_min: Option<(u32, u32)>,
    #[rune(get, set)]
    pub scale_max: Option<(u32, u32)>,
    #[rune(get, set)]
    pub scale: (u32, u32),
    #[rune(get, set)]
    pub head: bool,
    #[rune(get, set)]
    pub sync: bool,
    #[rune(get, set)]
    pub full: bool,
    #[rune(get, set)]
    pub decor: bool,
    #[rune(get, set)]
    pub resize: bool,
    #[rune(get, set)]
    pub hidden: bool,
    #[rune(get, set)]
    pub minimize: bool,
    #[rune(get, set)]
    pub maximize: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            name: "Laravox".to_string(),
            icon: None,
            scale_min: None,
            scale_max: None,
            scale: (1024, 768),
            head: true,
            sync: true,
            full: false,
            decor: true,
            resize: true,
            hidden: false,
            minimize: false,
            maximize: false,
        }
    }
}

//================================================================

#[derive(Any)]
pub struct State {
    /// OpenGL handle.
    pub frame: FrameInput,
    /// input handle for window/device event data.
    pub input: Input,
    /// audio handle for audio sink creation.
    pub audio: (OutputStream, OutputStreamHandle),
}

impl State {
    fn new(frame: FrameInput) -> Self {
        let (stream, handle) = rodio::OutputStream::try_default().unwrap();

        Self {
            frame,
            input: Input::default(),
            audio: (stream, handle),
        }
    }
}

//================================================================

#[derive(Default)]
pub struct Input {
    pub board: Board,
    pub mouse: Mouse,
    pub window_get: WindowGet,
    pub window_set: WindowSet,
}

impl Input {
    // this has to match the VirtualKeyCode count in winit.
    const BUTTON_COUNT_BOARD: usize = 163;
    // don't actually know what the total mouse button count is.
    const BUTTON_COUNT_MOUSE: usize = 16;

    pub fn process(
        &mut self,
        event: &winit::event::Event<()>,
        window: &mut winit::window::Window,
        generator: &mut FrameInputGenerator,
    ) {
        self.handle_state(window);

        if let winit::event::Event::WindowEvent { event, .. } = event {
            generator.handle_winit_window_event(event);

            match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    self.window_get.maximize = window.is_maximized();
                    self.window_get.full = window.fullscreen().is_some();

                    self.window_get.scale = Some(Vec2::rust_new(
                        physical_size.width as f32,
                        physical_size.height as f32,
                    ));
                }
                winit::event::WindowEvent::Moved(physical_position) => {
                    self.window_get.point = Some(Vec2::rust_new(
                        physical_position.x as f32,
                        physical_position.y as f32,
                    ));
                }
                winit::event::WindowEvent::Focused(focus) => {
                    if let Some(minimize) = window.is_minimized() {
                        self.window_get.minimize = minimize;
                    }

                    self.window_get.focus = *focus;
                }
                winit::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(button) = input.virtual_keycode {
                        match input.state {
                            winit::event::ElementState::Pressed => {
                                self.board.data[button as usize].down = true;
                                self.board.data[button as usize].press = true;
                            }
                            winit::event::ElementState::Released => {
                                self.board.data[button as usize].down = false;
                                self.board.data[button as usize].release = true;
                            }
                        }
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } => {
                    self.mouse.point.x = position.x as f32;
                    self.mouse.point.y = position.y as f32;
                }
                winit::event::WindowEvent::CursorEntered { .. } => {
                    self.mouse.state = Some(true);
                }
                winit::event::WindowEvent::CursorLeft { .. } => {
                    self.mouse.state = Some(false);
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } => {
                    let (x, y) = match delta {
                        winit::event::MouseScrollDelta::LineDelta(x, y) => (*x, *y),
                        winit::event::MouseScrollDelta::PixelDelta(physical_position) => {
                            (physical_position.x as f32, physical_position.y as f32)
                        }
                    };

                    self.mouse.wheel.x = x;
                    self.mouse.wheel.y = y;
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } => {
                    let button = match button {
                        winit::event::MouseButton::Left => 0,
                        winit::event::MouseButton::Right => 1,
                        winit::event::MouseButton::Middle => 2,
                        winit::event::MouseButton::Other(index) => *index as usize,
                    };

                    if let Some(entry) = self.mouse.data.get_mut(button) {
                        match state {
                            winit::event::ElementState::Pressed => {
                                entry.down = true;
                                entry.press = true;
                            }
                            winit::event::ElementState::Released => {
                                entry.down = false;
                                entry.release = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn handle_state(&mut self, window: &mut winit::window::Window) {
        // minimize the window.
        if self.window_set.minimize.is_some() {
            window.set_minimized(true);
            self.window_set.minimize = None;
        }

        // maximize the window.
        if self.window_set.maximize.is_some() {
            window.set_maximized(true);
            self.window_set.minimize = None;
        }

        // focus the window.
        if self.window_set.focus.is_some() {
            window.focus_window();
            self.window_set.focus = None;
        }

        // focus the window.
        if let Some(name) = &self.window_set.name {
            window.set_title(name);
            self.window_set.name = None;
        }

        // go full-screen, or back to window mode.
        if let Some(full) = self.window_set.full {
            if full {
                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
            } else {
                window.set_fullscreen(None);
            }

            self.window_set.full = None;
        }

        // reset all previous board state.
        for button in &mut self.board.data {
            button.press = false;
            button.release = false;
        }

        // reset all previous mouse state.
        for button in &mut self.mouse.data {
            button.press = false;
            button.release = false;
        }

        self.mouse.wheel.x = 0.0;
        self.mouse.wheel.y = 0.0;
        self.mouse.state = None;

        // reset all previous window state.
        self.window_get.point = None;
        self.window_get.scale = None;
    }
}

pub struct Board {
    // board button data.
    pub data: [Button; Input::BUTTON_COUNT_BOARD],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            data: [Button::default(); Input::BUTTON_COUNT_BOARD],
        }
    }
}

pub struct Mouse {
    /// mouse button data.
    pub data: [Button; Input::BUTTON_COUNT_MOUSE],
    /// mouse wheel data (delta).
    pub wheel: Vec2,
    /// mouse cursor window enter-leave state.
    pub state: Option<bool>,
    /// mouse cursor point.
    pub point: Vec2,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            data: [Button::default(); Input::BUTTON_COUNT_MOUSE],
            wheel: Vec2::rust_new(0.0, 0.0),
            state: None,
            point: Vec2::rust_new(0.0, 0.0),
        }
    }
}

#[derive(Default)]
pub struct WindowGet {
    /// is the window minimized?
    pub minimize: bool,
    /// is the window maximized?
    pub maximize: bool,
    /// is the window focused?
    pub focus: bool,
    /// has the window point been modified?
    pub point: Option<Vec2>,
    /// has the window scale been modified?
    pub scale: Option<Vec2>,
    /// is the window full-screen?
    pub full: bool,
}

#[derive(Default)]
pub struct WindowSet {
    /// minimize the window.
    pub minimize: Option<()>,
    /// maximize the window.
    pub maximize: Option<()>,
    /// focus the window.
    pub focus: Option<()>,
    /// set the window point.
    pub point: Option<Vec2>,
    /// set the window name.
    pub name: Option<String>,
    /// set the window icon.
    pub icon: Option<String>,
    /// set the minimum window scale.
    pub scale_min: Option<Vec2>,
    /// set the maximum window scale.
    pub scale_max: Option<Vec2>,
    /// set the window scale.
    pub scale: Option<Vec2>,
    /// go full-screen, or window mode.
    pub full: Option<bool>,
}

#[derive(Copy, Clone, Default)]
pub struct Button {
    pub down: bool,
    pub press: bool,
    pub release: bool,
}
