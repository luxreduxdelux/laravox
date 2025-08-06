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

//use crate::module::general::Vec2;
use gilrs::{Event, GamepadId, Gilrs};
use koto::{
    Koto,
    runtime::{CallArgs, KMap, KValue},
};
use notify::{EventKind, RecommendedWatcher, event};
use rodio::{OutputStream, OutputStreamHandle};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc::Receiver};
use three_d::FrameInput;
use winit::{
    event_loop::ControlFlow,
    window::{CursorGrabMode, CursorIcon},
};

//================================================================

pub struct Script {
    /// Rune virtual machine handle.
    #[allow(dead_code)]
    pub handle: Option<Handle>,
    /// Rust state.
    pub state: Option<State>,
    /// Rune error.
    pub error: Option<String>,
}

impl Script {
    pub fn new() -> anyhow::Result<Self> {
        // get the Rune compilation unit, with the Rune source and context.
        let compile = Handle::new();

        // get the Rune handle, or alternatively, the error message, if any compile error was found.
        let (handle, error) = match compile {
            Ok(value) => (Some(value), None),
            Err(error) => (None, Some(error.to_string())),
        };

        Ok(Self {
            handle,
            state: None,
            error,
        })
    }

    pub fn window(&mut self) -> Window {
        /*
        // we have a handle to the Rune VM.
        if let Some(handle) = &mut self.handle {
            // call the window entry-point.
            match handle.safe_call(handle.window.clone(), &[]) {
                Ok(value) => koto::serde::from_koto_value(value).unwrap(),
                Err(error) => {
                    // error in entry-point, return default window.
                    self.error = Some(error.to_string());
                }
            }
        }
         */

        Window::default()
    }

    fn begin(&mut self, frame: FrameInput) {
        if let Some(handle) = &mut self.handle {
            let mut state = State::new(frame);

            unsafe {
                GLOBAL_STATE = &mut state;
            }

            self.state = Some(state);

            match handle.safe_call(handle.begin.clone(), &[]) {
                Ok(_) => {}
                Err(error) => {
                    self.error = Some(error.to_string());
                }
            }
        }
    }

    pub fn frame(&mut self, frame: FrameInput, control_flow: &mut ControlFlow) {
        if let Some(handle) = &mut self.handle {
            if let Some(state) = &mut self.state {
                unsafe {
                    let mut state = &*GLOBAL_STATE;
                }

                state.frame = frame.clone();

                match handle.safe_call(handle.frame.clone(), &[]) {
                    Ok(_) => {}
                    /*
                    match value {
                        1 => control_flow.set_exit(),
                        2 => self.rebuild(),
                        3 => self.restart(frame),
                        _ => {}
                    }
                     */
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
        println!("Call close.");

        if let Some(handle) = &mut self.handle {
            match handle.safe_call(handle.close.clone(), &[]) {
                Ok(_) => {}
                Err(error) => {
                    self.error = Some(error.to_string());
                }
            }
        }
    }

    pub fn rebuild(&mut self) {
        self.error = None;

        match Handle::new() {
            Ok(value) => self.handle = Some(value),
            Err(error) => self.error = Some(error.to_string()),
        }
    }

    pub fn restart(&mut self, frame: FrameInput) {
        self.error = None;

        match Handle::new() {
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
    handle: Koto,
    //source: Sources,
    /// entry-point function; retrieve window configuration.
    window: KValue,
    /// entry-point function; retrieve Rune state.
    begin: KValue,
    /// entry-point function; main loop.
    frame: KValue,
    /// entry-point function; Rune state destructor.
    close: KValue,
    /// source file watcher.
    watcher: Option<(
        RecommendedWatcher,
        Receiver<Result<notify::Event, notify::Error>>,
    )>,
}

impl<'a> Handle {
    const MAIN_PATH: &'static str = "main/main.koto";
    const MAIN_NAME: &'static str = "Main";
    const CALL_WINDOW: &'static str = "window";
    const CALL_BEGIN: &'static str = "begin";
    const CALL_FRAME: &'static str = "frame";
    const CALL_CLOSE: &'static str = "close";

    fn get_function(export: &KMap, name: &str) -> anyhow::Result<KValue> {
        if let Some(call) = export.get(name) {
            Ok(call)
        } else {
            Err(anyhow::Error::msg(format!(
                "Handle::get_function(): Couldn't find function \"{name}\"."
            )))
        }
    }

    fn new() -> anyhow::Result<Self> {
        let mut handle = Koto::default();
        let prelude = handle.prelude();

        prelude.insert("general", crate::module::general::module());

        handle.compile_and_run(&std::fs::read_to_string(Self::MAIN_PATH)?)?;

        let export = handle.exports();

        let window = Self::get_function(export, Self::CALL_WINDOW)?;
        let begin = Self::get_function(export, Self::CALL_BEGIN)?;
        let frame = Self::get_function(export, Self::CALL_FRAME)?;
        let close = Self::get_function(export, Self::CALL_CLOSE)?;

        Ok(Self {
            handle,
            window,
            begin,
            frame,
            close,
            watcher: None,
        })
    }

    fn safe_call<A: Into<CallArgs<'a>>>(
        &'a mut self,
        call: KValue,
        argument: A,
    ) -> anyhow::Result<KValue> {
        Ok(self.handle.call_function(call, argument)?)
    }
}

//================================================================

#[derive(Deserialize, Serialize)]
pub struct Window {
    pub name: String,
    pub icon: Option<String>,
    pub scale_min: Option<(u32, u32)>,
    pub scale_max: Option<(u32, u32)>,
    pub scale: (u32, u32),
    pub head: bool,
    pub sync: bool,
    pub full: bool,
    pub decor: bool,
    pub resize: bool,
    pub hidden: bool,
    pub minimize: bool,
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

pub static mut GLOBAL_STATE: *mut State = std::ptr::null_mut();

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
    pub pad: Pad,
    pub window_get: WindowGet,
    pub window_set: WindowSet,
}

impl Input {
    // this has to match the VirtualKeyCode count in winit.
    const BUTTON_COUNT_BOARD: usize = 163;
    // don't actually know what the total mouse button count is.
    const BUTTON_COUNT_MOUSE: usize = 16;
    const BUTTON_COUNT_PAD: usize = 20;

    pub fn process(&mut self, event: &winit::event::Event<()>, window: &mut winit::window::Window) {
        match event {
            winit::event::Event::MainEventsCleared => {
                self.handle_state(window);
            }
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::Resized(physical_size) => {
                    self.window_get.maximize = window.is_maximized();
                    self.window_get.full = window.fullscreen().is_some();

                    //self.window_get.scale = Some(Vec2::rust_new(
                    //    physical_size.width as f32,
                    //    physical_size.height as f32,
                    //));
                }
                winit::event::WindowEvent::Moved(physical_position) => {
                    //self.window_get.point = Some(Vec2::rust_new(
                    //    physical_position.x as f32,
                    //    physical_position.y as f32,
                    //));
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
                    //self.mouse.point.x = position.x as f32;
                    //self.mouse.point.y = position.y as f32;
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

                    //self.mouse.wheel.x = x;
                    //self.mouse.wheel.y = y;
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
            },
            winit::event::Event::DeviceEvent { event, .. } => match event {
                winit::event::DeviceEvent::MouseMotion { delta } => {
                    //self.mouse.delta.x = delta.0 as f32;
                    //self.mouse.delta.y = delta.1 as f32;
                }
                _ => {}
            },
            _ => {}
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

        if let Some(icon) = self.window_set.cursor_icon {
            // warning -- this is assuming that the input is safe, from input.rs!
            let icon: CursorIcon = unsafe { std::mem::transmute(icon as i8) };

            window.set_cursor_icon(icon);

            self.window_set.cursor_icon = None;
        }

        if let Some(show) = self.window_set.cursor_show {
            window.set_cursor_visible(show);

            self.window_set.cursor_show = None;
        }

        if let Some(lock) = self.window_set.cursor_lock {
            let _ = window.set_cursor_grab(if lock {
                CursorGrabMode::Confined
            } else {
                CursorGrabMode::None
            });

            self.window_set.cursor_lock = None;
        }

        // reset all previous board state.
        for button in &mut self.board.data {
            button.press = false;
            button.release = false;
        }

        self.board.last_press = None;
        self.board.last_release = None;

        // reset all previous mouse state.
        for button in &mut self.mouse.data {
            button.press = false;
            button.release = false;
        }

        self.mouse.last_press = None;
        self.mouse.last_release = None;
        //self.mouse.wheel.x = 0.0;
        //self.mouse.wheel.y = 0.0;
        //self.mouse.delta.x = 0.0;
        //self.mouse.delta.y = 0.0;
        self.mouse.state = None;

        // reset all previous window state.
        //self.window_get.point = None;
        //self.window_get.scale = None;

        // process pad data.
        self.pad.process();
    }
}

pub struct Board {
    // board button data.
    pub data: [Button; Input::BUTTON_COUNT_BOARD],
    /// last board button press.
    pub last_press: Option<usize>,
    /// last board button release.
    pub last_release: Option<usize>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            data: [Button::default(); Input::BUTTON_COUNT_BOARD],
            last_press: None,
            last_release: None,
        }
    }
}

pub struct Mouse {
    /// mouse button data.
    pub data: [Button; Input::BUTTON_COUNT_MOUSE],
    /// mouse wheel data (delta).
    //pub wheel: Vec2,
    /// mouse cursor window enter-leave state.
    pub state: Option<bool>,
    /// mouse cursor point.
    //pub point: Vec2,
    /// mouse cursor delta.
    //pub delta: Vec2,
    /// last mouse button press.
    pub last_press: Option<usize>,
    /// last mouse button release.
    pub last_release: Option<usize>,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            data: [Button::default(); Input::BUTTON_COUNT_MOUSE],
            //wheel: Vec2::rust_new(0.0, 0.0),
            state: None,
            //point: Vec2::rust_new(0.0, 0.0),
            //delta: Vec2::rust_new(0.0, 0.0),
            last_press: None,
            last_release: None,
        }
    }
}

#[derive(Default)]
pub struct PadState {
    pub button: [Button; Input::BUTTON_COUNT_PAD],
    pub axis: [f32; 9],
    /// last pad button press.
    pub last_press: Option<usize>,
    /// last pad button release.
    pub last_release: Option<usize>,
}

pub struct Pad {
    /// pad button data.
    pub data: HashMap<GamepadId, PadState>,
    /// GILRS handle.
    pub handle: Gilrs,
}

impl Pad {
    pub fn get_pad(&self, index: usize) -> Option<(&GamepadId, &PadState)> {
        for (i, value) in self.data.iter().enumerate() {
            if i == index {
                return Some(value);
            }
        }

        None
    }

    fn process(&mut self) {
        // reset all previous pad state.
        for pad in self.data.values_mut() {
            for button in &mut pad.button {
                button.press = false;
                button.release = false;
            }

            pad.last_press = None;
            pad.last_release = None;
        }

        while let Some(Event { event, id, .. }) = self.handle.next_event() {
            let entry = self.data.entry(id).or_default();

            match event {
                gilrs::EventType::ButtonPressed(button, _) => {
                    entry.button[button as usize].down = true;
                    entry.button[button as usize].press = true;
                }
                gilrs::EventType::ButtonReleased(button, _) => {
                    entry.button[button as usize].down = false;
                    entry.button[button as usize].release = true;
                }
                gilrs::EventType::AxisChanged(axis, value, _) => {
                    entry.axis[axis as usize] = value;
                }
                gilrs::EventType::Connected => {}
                gilrs::EventType::Disconnected => {}
                gilrs::EventType::ButtonChanged(button, value, _) => match button {
                    // code.into_u32() doesn't seem to work properly, just map the button
                    // to the corresponding axis entry manually for now.
                    gilrs::Button::LeftTrigger2 => entry.axis[2] = value,
                    gilrs::Button::RightTrigger2 => entry.axis[5] = value,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

impl Default for Pad {
    fn default() -> Self {
        Self {
            data: HashMap::default(),
            handle: Gilrs::new().expect("Pad::default(): Couldn't get GILRS handle."),
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
    //pub point: Option<Vec2>,
    /// has the window scale been modified?
    //pub scale: Option<Vec2>,
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
    //pub point: Option<Vec2>,
    /// set the window name.
    pub name: Option<String>,
    /// set the window icon.
    pub icon: Option<String>,
    /// set the minimum window scale.
    //pub scale_min: Option<Vec2>,
    /// set the maximum window scale.
    //pub scale_max: Option<Vec2>,
    /// set the window scale.
    //pub scale: Option<Vec2>,
    /// go full-screen, or window mode.
    pub full: Option<bool>,
    /// cursor icon.
    pub cursor_icon: Option<usize>,
    /// show, or hide the cursor.
    pub cursor_show: Option<bool>,
    /// lock, or free the cursor.
    pub cursor_lock: Option<bool>,
}

#[derive(Copy, Clone, Default)]
pub struct Button {
    pub down: bool,
    pub press: bool,
    pub release: bool,
}
