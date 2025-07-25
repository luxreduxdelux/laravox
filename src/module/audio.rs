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

use crate::{module::general::Vec2, script::State};

//================================================================

use rodio::SpatialSink;
use rune::{Any, Module};
use std::{fs::File, sync::Arc};

//================================================================

/// A handle to a sound file.
#[derive(Any)]
#[rune(item = ::audio)]
struct Sound {
    data: SoundData,
    sink: SpatialSink,
}

impl Sound {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::play)?;
        module.function_meta(Self::stop)?;
        module.function_meta(Self::pause)?;
        module.function_meta(Self::resume)?;
        module.function_meta(Self::get_state)?;
        module.function_meta(Self::set_volume)?;
        module.function_meta(Self::set_speed)?;
        module.function_meta(Self::set_point)?;

        Ok(())
    }

    //================================================================

    /// Create a new sound instance.
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str) -> anyhow::Result<Self> {
        let data = std::fs::read(path)?;
        let sink = SpatialSink::try_new(
            &state.audio.1,
            [0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        )?;

        Ok(Self {
            data: SoundData(Arc::new(data)),
            sink,
        })
    }

    /// Play the track.
    #[rune::function]
    fn play(&mut self) -> anyhow::Result<()> {
        let data = std::io::Cursor::new(SoundData(self.data.0.clone()));

        self.sink.stop();
        self.sink.append(rodio::Decoder::new(data)?);
        self.sink.play();

        Ok(())
    }

    /// Stop the track.
    #[rune::function]
    fn stop(&mut self) {
        self.sink.stop();
    }

    /// Pause the track.
    #[rune::function]
    fn pause(&mut self) {
        self.sink.pause();
    }

    /// Resume the track.
    #[rune::function]
    fn resume(&mut self) {
        self.sink.play();
    }

    /// Get current playing state. If true, track is currently playing, false otherwise.
    #[rune::function]
    fn get_state(&mut self) -> bool {
        !self.sink.is_paused()
    }

    /// Set track volume.
    #[rune::function]
    fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    /// Set track speed.
    #[rune::function]
    fn set_speed(&mut self, speed: f32) {
        self.sink.set_speed(speed);
    }

    /// Set track panning, with `point_source` being the source of the emitter, and `point_target` being the listener's point.
    #[rune::function]
    fn set_point(&mut self, point_source: &Vec2, point_target: &Vec2) {
        self.sink
            .set_emitter_position([point_source.x, point_source.y, 0.0]);
        self.sink
            .set_left_ear_position([point_target.x - 1.0, point_target.y, 0.0]);
        self.sink
            .set_right_ear_position([point_target.x + 1.0, point_target.y, 0.0]);
    }
}

struct SoundData(Arc<Vec<u8>>);

impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

//================================================================

/// A handle to a music file.
#[derive(Any)]
#[rune(item = ::audio)]
struct Music {
    data: File,
    sink: SpatialSink,
}

impl Music {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::new)?;
        module.function_meta(Self::play)?;
        module.function_meta(Self::stop)?;
        module.function_meta(Self::pause)?;
        module.function_meta(Self::resume)?;
        module.function_meta(Self::get_state)?;
        module.function_meta(Self::set_volume)?;
        module.function_meta(Self::set_speed)?;
        module.function_meta(Self::set_point)?;

        Ok(())
    }

    //================================================================

    /// Create a new music instance.
    #[rune::function(path = Self::new)]
    fn new(state: &State, path: &str) -> anyhow::Result<Self> {
        let data = File::open(path)?;
        let sink = SpatialSink::try_new(
            &state.audio.1,
            [0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        )?;

        Ok(Self { data, sink })
    }

    /// Play the track.
    #[rune::function]
    fn play(&mut self) -> anyhow::Result<()> {
        self.sink.stop();
        self.sink
            .append(rodio::Decoder::new(self.data.try_clone()?)?);
        self.sink.play();

        Ok(())
    }

    /// Stop the track.
    #[rune::function]
    fn stop(&mut self) {
        self.sink.stop();
    }

    /// Pause the track.
    #[rune::function]
    fn pause(&mut self) {
        self.sink.pause();
    }

    /// Resume the track.
    #[rune::function]
    fn resume(&mut self) {
        self.sink.play();
    }

    /// Get current playing state. If true, track is currently playing, false otherwise.
    #[rune::function]
    fn get_state(&mut self) -> bool {
        !self.sink.is_paused()
    }

    /// Set track volume.
    #[rune::function]
    fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
    }

    /// Set track speed.
    #[rune::function]
    fn set_speed(&mut self, speed: f32) {
        self.sink.set_speed(speed);
    }

    /// Set track panning, with `point_source` being the source of the emitter, and `point_target` being the listener's point.
    #[rune::function]
    fn set_point(&mut self, point_source: &Vec2, point_target: &Vec2) {
        self.sink
            .set_emitter_position([point_source.x, point_source.y, 0.0]);
        self.sink
            .set_left_ear_position([point_target.x - 1.0, point_target.y, 0.0]);
        self.sink
            .set_right_ear_position([point_target.x + 1.0, point_target.y, 0.0]);
    }
}

//================================================================

#[rune::module(::audio)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    Sound::module(&mut module)?;
    Music::module(&mut module)?;

    Ok(module)
}
