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
    Any, Context, Diagnostics, FromValue, Module, Source, Sources, Value, Vm,
    runtime::{Function, GuardedArgs},
    termcolor::{Buffer, ColorChoice, StandardStream},
};
use std::sync::Arc;

//================================================================

pub struct Script {
    /// Rune virtual machine handle.
    #[allow(dead_code)]
    handle: Option<Handle>,
    /// Rune context.
    #[allow(dead_code)]
    context: Context,
    /// Rune state.
    value: Option<Value>,
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
            error,
        })
    }

    pub fn window(&mut self) -> Window {
        // we have a handle to the Rune VM.
        if let Some(handle) = &self.handle {
            // call the window entry-point.
            match handle.safe_call(&handle.window, ()) {
                Ok(value) => {
                    // all clear, return window.
                    value
                }
                Err(error) => {
                    // error in entry-point, return default window.
                    self.error = Some(error.to_string());
                    Window::default()
                }
            }
        } else {
            // return default window.
            Window::default()
        }
    }

    fn begin(&mut self, state: &crate::system::State) {
        if let Some(handle) = &self.handle {
            match handle.safe_call(&handle.begin, (state,)) {
                Ok(value) => {
                    self.value = Some(value);
                }
                Err(error) => {
                    self.error = Some(error.to_string());
                }
            }
        }
    }

    pub fn frame(&mut self, state: &crate::system::State) {
        if let Some(handle) = &self.handle {
            if let Some(value) = &self.value {
                match handle.safe_call(&handle.frame, (value, state)) {
                    Ok(value) => value,
                    Err(error) => {
                        self.error = Some(error.to_string());
                    }
                }
            } else {
                self.begin(state);
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
}

//================================================================

struct Handle {
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

        // create Rune virtual machine.
        let handle = Vm::new(Arc::new(context.runtime()?), Arc::new(unit?));

        Ok(Self {
            source,
            window: handle.lookup_function([Self::MAIN_NAME, Self::CALL_WINDOW])?,
            begin: handle.lookup_function([Self::MAIN_NAME, Self::CALL_BEGIN])?,
            frame: handle.lookup_function([Self::MAIN_NAME, Self::CALL_FRAME])?,
            close: handle.lookup_function([Self::MAIN_NAME, Self::CALL_CLOSE])?,
            handle,
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
#[rune(constructor)]
pub struct Window {
    pub title: String,
    pub min_scale: Option<(u32, u32)>,
    pub max_scale: Option<(u32, u32)>,
    pub scale: (u32, u32),
    pub sync: bool,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: "Laravox".to_string(),
            min_scale: None,
            max_scale: None,
            scale: (1024, 768),
            sync: true,
        }
    }
}
