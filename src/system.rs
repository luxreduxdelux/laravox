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

use crate::script::*;

//================================================================

use rune::Any;
use three_d::{
    ClearState, FrameInput, FrameInputGenerator, GUI, SurfaceSettings, WindowedContext,
    egui::RichText,
};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

//================================================================

#[derive(Any)]
pub struct System {
    #[allow(dead_code)]
    window: Window,
    handle: WindowedContext,
    event: EventLoop<()>,
    frame: FrameInputGenerator,
    interface: GUI,
}

impl System {
    pub fn new(window: crate::script::Window) -> anyhow::Result<Self> {
        // TO-DO actually use script window content.
        let mut window_builder = WindowBuilder::new()
            .with_title(window.name)
            .with_inner_size(LogicalSize::new(window.scale.0, window.scale.1));

        if let Some(scale_min) = window.scale_min {
            window_builder =
                window_builder.with_min_inner_size(LogicalSize::new(scale_min.0, scale_min.1));
        }

        if let Some(scale_max) = window.scale_max {
            window_builder =
                window_builder.with_max_inner_size(LogicalSize::new(scale_max.0, scale_max.1));
        }

        let surface = SurfaceSettings {
            vsync: window.sync,
            ..Default::default()
        };

        let event = EventLoop::new();
        let window = window_builder.build(&event)?;
        let handle = WindowedContext::from_winit_window(&window, surface)?;
        let frame = FrameInputGenerator::from_winit_window(&window);
        let interface = GUI::new(&handle);

        Ok(Self {
            window,
            handle,
            event,
            frame,
            interface,
        })
    }

    pub fn run(mut self, mut script: Script) -> anyhow::Result<()> {
        self.event.run(move |event, _, control_flow| {
            if event == winit::event::Event::MainEventsCleared {
                let mut frame = self.frame.generate(&self.handle);

                script.watch();

                if let Some(error) = &script.error {
                    let (rebuild, restart, exit) =
                        Self::error(&mut self.interface, &mut frame, error);

                    if rebuild {
                        script.rebuild();
                    }
                    if restart {
                        script.restart(frame);
                    }
                    if exit {
                        control_flow.set_exit();
                    }
                } else {
                    script.frame(frame, control_flow);
                }

                self.handle
                    .swap_buffers()
                    .expect("System::run(): Error swapping frame buffer.");
            }

            if let winit::event::Event::WindowEvent { ref event, .. } = event {
                self.frame.handle_winit_window_event(event);

                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        self.handle.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        self.handle.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    _ => {}
                }
            }

            if let Some(state) = &mut script.state {
                state.input.process(&event, &mut self.window);
            }
        });
    }

    fn error(interface: &mut GUI, frame: &mut FrameInput, error: &str) -> (bool, bool, bool) {
        let mut rebuild = false;
        let mut restart = false;
        let mut exit = false;

        interface.update(
            &mut frame.events,
            frame.accumulated_time,
            frame.viewport,
            frame.device_pixel_ratio,
            |context| {
                three_d::egui::CentralPanel::default().show(context, |ui| {
                    context.input(|reader| {
                        if reader.key_pressed(three_d::egui::Key::Num1) {
                            rebuild = true;
                        }
                        if reader.key_pressed(three_d::egui::Key::Num2) {
                            restart = true;
                        }
                        if reader.key_pressed(three_d::egui::Key::Escape) {
                            exit = true;
                        }
                    });

                    ui.heading("Script Error");

                    ui.separator();

                    three_d::egui::ScrollArea::vertical()
                        .max_height(frame.viewport.height as f32 - 120.0)
                        .show(ui, |ui| ui.label(RichText::new(error).monospace()));

                    ui.separator();

                    if ui
                        .button("Rebuild")
                        .on_hover_text("[Number 1] Rebuild the source code, preserving state.")
                        .clicked()
                    {
                        rebuild = true;
                    };

                    if ui
                        .button("Restart")
                        .on_hover_text("[Number 2] Restart the virtual machine, losing state.")
                        .clicked()
                    {
                        restart = true;
                    };

                    if ui
                        .button("Exit")
                        .on_hover_text("[Escape] Exit Laravox.")
                        .clicked()
                    {
                        exit = true;
                    };
                });
            },
        );

        frame
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .write(|| interface.render())
            .expect("System::error(): Error on UI render.");

        (rebuild, restart, exit)
    }
}
