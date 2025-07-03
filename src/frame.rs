use rune::{Any, Module};

#[derive(Any)]
struct RuneFrame {}

impl RuneFrame {
    #[rune::function(keep, path = Self::new)]
    fn new() -> RuneFrame {
        RuneFrame {}
    }

    #[rune::function]
    fn foo(&self) -> usize {
        1337
    }
}

pub fn module() -> anyhow::Result<Module> {
    let mut m = Module::new();
    m.ty::<RuneFrame>()?;
    m.function_meta(RuneFrame::new__meta)?;
    m.function_meta(RuneFrame::foo)?;
    Ok(m)
}
