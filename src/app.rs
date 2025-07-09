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

use rune::{
    Any, Context, Diagnostics, Module, Source, Sources, Unit, Value, Vm, diagnostics,
    runtime::{Function, VmResult},
    termcolor::{ColorChoice, StandardStream},
};
use std::sync::Arc;
use three_d::{FrameInputGenerator, SurfaceSettings, WindowedContext};
use winit::event_loop::ControlFlow;

//================================================================

pub struct App {}

impl App {
    fn load_window(
        script: &Script,
    ) -> anyhow::Result<(
        three_d::WindowedContext,
        winit::event_loop::EventLoop<()>,
        winit::window::Window,
    )> {
        let r_window = script.window.call::<Window>(()).into_result()?;

        let event_loop = winit::event_loop::EventLoop::new();

        let mut window = winit::window::WindowBuilder::new();

        if let Some(title) = r_window.title {
            window = window.with_title(title);
        }

        if let Some(min_scale) = r_window.min_scale {
            window =
                window.with_min_inner_size(winit::dpi::LogicalSize::new(min_scale.0, min_scale.1));
        }

        if let Some(max_scale) = r_window.min_scale {
            window =
                window.with_max_inner_size(winit::dpi::LogicalSize::new(max_scale.0, max_scale.1));
        }

        let window = window.build(&event_loop)?;
        let surface = SurfaceSettings {
            vsync: true,
            ..Default::default()
        };

        let context = WindowedContext::from_winit_window(&window, surface).unwrap();

        Ok((context, event_loop, window))
    }

    fn load_value(
        script: &Script,
        frame_input: three_d::FrameInput,
        handle: rodio::OutputStreamHandle,
    ) -> anyhow::Result<Value> {
        let state = State::new(frame_input, FrameState::new(), handle);

        match script.begin.call::<Value>((state,)) {
            VmResult::Ok(value) => return Ok(value),
            VmResult::Err(error) => {
                let mut writer = StandardStream::stderr(ColorChoice::Auto);
                error.emit(&mut writer, &script.source).unwrap();
            }
        };

        Err(anyhow::Error::msg(""))
    }

    pub fn run() -> anyhow::Result<()> {
        // load audio handle.
        let (_stream, handle) = rodio::OutputStream::try_default()?;

        let mut script = Script::new()?;

        // load window.
        let (context, event_loop, window) = Self::load_window(&script)?;

        let mut frame_state = FrameState::new();
        let mut frame_input_generator = FrameInputGenerator::from_winit_window(&window);

        event_loop.run(move |event, _, control_flow| {
            match event {
                winit::event::Event::MainEventsCleared => {
                    let frame_input = frame_input_generator.generate(&context);
                    let state = State::new(frame_input.clone(), frame_state, handle.clone());

                    if script.value.is_none() {
                        script.value = Some(
                            Self::load_value(&script, frame_input.clone(), handle.clone()).unwrap(),
                        );
                    }

                    let value = script.value.as_ref().unwrap();
                    let frame = script.frame.call::<usize>((value, &state));

                    match frame {
                        VmResult::Ok(code) => {
                            if code == 1 {
                                // exit Laravox.
                                control_flow.set_exit();
                            } else if code == 2 {
                                // hot-reload.
                                script.reload().unwrap();
                            } else if code == 3 {
                                // restart.
                                script = Script::new().unwrap();
                            }
                        }
                        VmResult::Err(error) => {
                            let mut writer = StandardStream::stderr(ColorChoice::Auto);
                            error.emit(&mut writer, &script.source).unwrap();

                            if Script::error("Script Error", &error.to_string()) {
                                script = Script::new().unwrap();
                            } else {
                                control_flow.set_exit();
                            }
                        }
                    }

                    context.swap_buffers().unwrap();
                }
                _ => {}
            }

            frame_state.process(&event, &context, &mut frame_input_generator, control_flow);
        });
    }
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
    // this has to match the VirtualKeyCode count in winit.
    const BUTTON_COUNT_BOARD: usize = 163;
    // don't actually know what the total mouse button count is.
    // a run-time check is done to make sure you can actually index stuff...
    const BUTTON_COUNT_MOUSE: usize = 16;

    fn new() -> Self {
        Self {
            board: [InputState::default(); Self::BUTTON_COUNT_BOARD],
            mouse: [InputState::default(); Self::BUTTON_COUNT_MOUSE],
        }
    }

    fn process(
        &mut self,
        event: &winit::event::Event<()>,
        context: &WindowedContext,
        generator: &mut FrameInputGenerator,
        control_flow: &mut ControlFlow,
    ) {
        for button in &mut self.board {
            button.press = false;
            button.release = false;
        }

        for button in &mut self.mouse {
            button.press = false;
            button.release = false;
        }

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        context.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => (),
                }
            }
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::Button { button, state } => {
                    if let Some(entry) = self.mouse.get_mut(*button as usize) {
                        match state {
                            winit::event::ElementState::Pressed => {
                                entry.up = false;
                                entry.press = true;
                            }
                            winit::event::ElementState::Released => {
                                entry.up = true;
                                entry.release = true;
                            }
                        }
                    }
                }
                winit::event::DeviceEvent::Key(keyboard_input) => {
                    if let Some(button) = keyboard_input.virtual_keycode {
                        match keyboard_input.state {
                            winit::event::ElementState::Pressed => {
                                self.board[button as usize].up = false;
                                self.board[button as usize].press = true;
                            }
                            winit::event::ElementState::Released => {
                                self.board[button as usize].up = true;
                                self.board[button as usize].release = true;
                            }
                        }
                    }
                }

                _ => {}
            },
            _ => {}
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

struct Script {
    #[allow(dead_code)]
    handle: Vm,
    context: Context,
    source: Sources,
    window: Function,
    begin: Function,
    close: Function,
    frame: Function,
    value: Option<Value>,
}

impl Script {
    fn compile_unit() -> anyhow::Result<(Context, Sources, Unit)> {
        let mut module = Module::new();
        module.ty::<Window>()?;

        // install each module into the Rune context.
        let mut context = rune_modules::default_context()?;

        context.install(module)?;
        context.install(crate::module::general::module()?)?;
        context.install(crate::module::video::module()?)?;
        context.install(crate::module::audio::module()?)?;
        context.install(crate::module::input::module()?)?;
        context.install(crate::module::file::module()?)?;

        let mut source = Sources::new();

        source.insert(Source::from_path("script/main.rn")?)?;

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut source)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            diagnostics.emit(&mut writer, &source)?;

            for diagnostic in diagnostics.diagnostics() {
                if let diagnostics::Diagnostic::Fatal(_fatal_diagnostic) = diagnostic {
                    if Self::error("Compile Error", &_fatal_diagnostic.to_string()) {
                        return Self::compile_unit();
                    }
                }
            }
        }

        Ok((context, source, result?))
    }

    fn reload(&mut self) -> anyhow::Result<()> {
        let mut source = Sources::new();

        source.insert(Source::from_path("script/main.rn")?)?;

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut source)
            .with_context(&self.context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Auto);
            diagnostics.emit(&mut writer, &source)?;

            for diagnostic in diagnostics.diagnostics() {
                if let diagnostics::Diagnostic::Fatal(_fatal_diagnostic) = diagnostic {
                    if Self::error("Compile Error", &_fatal_diagnostic.to_string()) {
                        return self.reload();
                    }
                }
            }
        }

        self.handle = Vm::new(Arc::new(self.context.runtime()?), Arc::new(result?));
        self.source = source;
        self.window = self.handle.lookup_function(["Main", "window"])?;
        self.begin = self.handle.lookup_function(["Main", "begin"])?;
        self.close = self.handle.lookup_function(["Main", "close"])?;
        self.frame = self.handle.lookup_function(["Main", "frame"])?;

        Ok(())
    }

    fn new() -> anyhow::Result<Self> {
        let (context, source, unit) = Self::compile_unit()?;

        let handle = Vm::new(Arc::new(context.runtime()?), Arc::new(unit));

        Ok(Self {
            context,
            window: handle.lookup_function(["Main", "window"])?,
            begin: handle.lookup_function(["Main", "begin"])?,
            close: handle.lookup_function(["Main", "close"])?,
            frame: handle.lookup_function(["Main", "frame"])?,
            value: None,
            handle,
            source,
        })
    }

    fn error(title: &str, error: &str) -> bool {
        let error = format!("{error}\n\nReload script?");

        let result = rfd::MessageDialog::new()
            .set_level(rfd::MessageLevel::Error)
            .set_title(title)
            .set_buttons(rfd::MessageButtons::YesNo)
            .set_description(error)
            .show();

        match result {
            rfd::MessageDialogResult::Yes => true,
            _ => false,
        }
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        if let Some(value) = &self.value {
            self.close.call::<()>((value,)).into_result().unwrap();
        }
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
