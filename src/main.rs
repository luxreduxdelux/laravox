use app::App;

mod app;
mod frame;
mod image;
mod script;

fn main() -> anyhow::Result<()> {
    App::initialize()
}
