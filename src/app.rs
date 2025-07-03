use crate::script::*;

pub struct App {
    script: Script,
}

impl App {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            script: Script::new()?,
        })
    }

    pub fn initialize() -> anyhow::Result<()> {
        use three_d::*;

        let window = Window::new(WindowSettings {
            title: "Triangle!".to_string(),
            max_size: Some((1280, 720)),
            ..Default::default()
        })
        .unwrap();

        let context = window.gl();

        let mut camera = Camera::new_perspective(
            window.viewport(),
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );

        let positions = vec![
            vec3(0.5, -0.5, 0.0),  // bottom right
            vec3(-0.5, -0.5, 0.0), // bottom left
            vec3(0.0, 0.5, 0.0),   // top
        ];
        let colors = vec![
            Srgba::RED,   // bottom right
            Srgba::GREEN, // bottom left
            Srgba::BLUE,  // top
        ];
        let cpu_mesh = CpuMesh {
            positions: Positions::F32(positions),
            colors: Some(colors),
            ..Default::default()
        };

        let mut model = Gm::new(Mesh::new(&context, &cpu_mesh), ColorMaterial::default());

        model.set_animation(|time| Mat4::from_angle_y(radians(time * 0.005)));

        let app = App::new()?;

        window.render_loop(
            move |frame_input| // Begin a new frame with an updated frame input
            {
                camera.set_viewport(frame_input.viewport);

                model.animate(frame_input.accumulated_time as f32);

                frame_input.screen()
                    .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                    .render(
                        &camera, &model, &[]
                    );

                let begin = &app.script.state_main.0;
                let value = &app.script.state_main.2;

                begin.call::<()>((value,)).unwrap();

                FrameOutput::default()
            },
        );

        Ok(())
    }
}
