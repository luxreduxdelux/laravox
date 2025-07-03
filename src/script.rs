use rune::{
    Diagnostics, Hash, Source, Value, Vm,
    runtime::{DynamicStruct, Function, Struct},
    termcolor::{ColorChoice, StandardStream},
};
use std::sync::Arc;

pub struct Script {
    pub machine: Vm,
    pub state_main: (Function, Function, Value),
}

impl Script {
    pub fn new() -> anyhow::Result<Self> {
        // install each module into the Rune context.
        let mut context = rune_modules::default_context()?;

        context.install(crate::image::module()?)?;
        context.install(crate::frame::module()?)?;

        let runtime = Arc::new(context.runtime()?);

        // load the main file.
        let mut sources = rune::sources! {
         entry => {
                pub fn main() {
                    (Main::frame, Main::close, Main::begin())
                }
            }
        };

        sources.insert(Source::from_path("test/main.rs")?)?;

        let mut diagnostics = Diagnostics::new();

        let result = rune::prepare(&mut sources)
            .with_context(&context)
            .with_diagnostics(&mut diagnostics)
            .build();

        if !diagnostics.is_empty() {
            let mut writer = StandardStream::stderr(ColorChoice::Always);
            diagnostics.emit(&mut writer, &sources)?;
        }

        let unit = result?;
        let unit = Arc::new(unit);
        let mut machine = Vm::new(runtime, unit);

        let state_main = machine.execute(["main"], ()).unwrap().complete().unwrap();
        let state_main: (Function, Function, Value) = rune::from_value(state_main)?;

        Ok(Self {
            machine,
            state_main,
        })
    }
}

impl Drop for Script {
    fn drop(&mut self) {
        let close = &self.state_main.1;
        let value = &self.state_main.2;

        close.call::<()>((value,)).unwrap();
    }
}
