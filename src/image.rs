use rune::{Any, Module};
use three_d::{Camera, ColorMaterial, Gm, Object, Rectangle};

#[derive(Any)]
struct RuneCamera {
    camera: Camera,
}

#[derive(Any)]
struct RuneImage {
    image: Gm<Rectangle, ColorMaterial>,
}

impl RuneImage {
    #[rune::function]
    fn draw(&self, r_camera: &RuneCamera) {
        self.image.render(&r_camera.camera, &[]);
    }
}

pub fn module() -> anyhow::Result<Module> {
    let mut m = Module::new();
    m.ty::<RuneImage>()?;
    m.function_meta(RuneImage::draw)?;
    Ok(m)
}
