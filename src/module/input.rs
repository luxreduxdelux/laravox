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

use crate::script::{Button, State};

//================================================================

use rune::{Any, Module};

use super::general::Vec2;

//================================================================

#[derive(Any)]
#[rune(item = ::input)]
struct Board {}

impl Board {
    const KEY_LIST: [&str; 163] = [
        "BOARD_1",
        "BOARD_2",
        "BOARD_3",
        "BOARD_4",
        "BOARD_5",
        "BOARD_6",
        "BOARD_7",
        "BOARD_8",
        "BOARD_9",
        "BOARD_0",
        "BOARD_A",
        "BOARD_B",
        "BOARD_C",
        "BOARD_D",
        "BOARD_E",
        "BOARD_F",
        "BOARD_G",
        "BOARD_H",
        "BOARD_I",
        "BOARD_J",
        "BOARD_K",
        "BOARD_L",
        "BOARD_M",
        "BOARD_N",
        "BOARD_O",
        "BOARD_P",
        "BOARD_Q",
        "BOARD_R",
        "BOARD_S",
        "BOARD_T",
        "BOARD_U",
        "BOARD_V",
        "BOARD_W",
        "BOARD_X",
        "BOARD_Y",
        "BOARD_Z",
        "BOARD_ESCAPE",
        "BOARD_F1",
        "BOARD_F2",
        "BOARD_F3",
        "BOARD_F4",
        "BOARD_F5",
        "BOARD_F6",
        "BOARD_F7",
        "BOARD_F8",
        "BOARD_F9",
        "BOARD_F10",
        "BOARD_F11",
        "BOARD_F12",
        "BOARD_F13",
        "BOARD_F14",
        "BOARD_F15",
        "BOARD_F16",
        "BOARD_F17",
        "BOARD_F18",
        "BOARD_F19",
        "BOARD_F20",
        "BOARD_F21",
        "BOARD_F22",
        "BOARD_F23",
        "BOARD_F24",
        "BOARD_SNAPSHOT",
        "BOARD_SCROLL",
        "BOARD_PAUSE",
        "BOARD_INSERT",
        "BOARD_HOME",
        "BOARD_DELETE",
        "BOARD_END",
        "BOARD_PAGE_DOWN",
        "BOARD_PAGE_UP",
        "BOARD_LEFT",
        "BOARD_UP",
        "BOARD_RIGHT",
        "BOARD_DOWN",
        "BOARD_BACK",
        "BOARD_RETURN",
        "BOARD_SPACE",
        "BOARD_COMPOSE",
        "BOARD_CARET",
        "BOARD_NUMBER_LOCK",
        "BOARD_NUMBER_PAD_0",
        "BOARD_NUMBER_PAD_1",
        "BOARD_NUMBER_PAD_2",
        "BOARD_NUMBER_PAD_3",
        "BOARD_NUMBER_PAD_4",
        "BOARD_NUMBER_PAD_5",
        "BOARD_NUMBER_PAD_6",
        "BOARD_NUMBER_PAD_7",
        "BOARD_NUMBER_PAD_8",
        "BOARD_NUMBER_PAD_9",
        "BOARD_NUMBER_PAD_ADD",
        "BOARD_NUMBER_PAD_DIVIDE",
        "BOARD_NUMBER_PAD_DECIMAL",
        "BOARD_NUMBER_PAD_COMMA",
        "BOARD_NUMBER_PAD_ENTER",
        "BOARD_NUMBER_PAD_EQUAL",
        "BOARD_NUMBER_PAD_MULTIPLY",
        "BOARD_NUMBER_PAD_SUBTRACT",
        "BOARD_ABNTC1",
        "BOARD_ABNTC2",
        "BOARD_APOSTROPHE",
        "BOARD_APPS",
        "BOARD_ASTERISK",
        "BOARD_AT",
        "BOARD_AX",
        "BOARD_BACKSLASH",
        "BOARD_CALCULATOR",
        "BOARD_CAPITAL",
        "BOARD_COLON",
        "BOARD_COMMA",
        "BOARD_CONVERT",
        "BOARD_EQUALS",
        "BOARD_GRAVE",
        "BOARD_KANA",
        "BOARD_KANJI",
        "BOARD_LEFT_ALTERNATE",
        "BOARD_LEFT_BRACKET",
        "BOARD_LEFT_CONTROL",
        "BOARD_LEFT_SHIFT",
        "BOARD_LEFT_SUPER",
        "BOARD_MAIL",
        "BOARD_MEDIA_SELECT",
        "BOARD_MEDIA_STOP",
        "BOARD_MINUS",
        "BOARD_MUTE",
        "BOARD_MY_COMPUTER",
        "BOARD_NAVIGATE_FORWARD",
        "BOARD_NAVIGATE_BACKWARD",
        "BOARD_NEXT_TRACK",
        "BOARD_NO_CONVERT",
        "BOARD_OEM102",
        "BOARD_PERIOD",
        "BOARD_PLAY_PAUSE",
        "BOARD_PLUS",
        "BOARD_POWER",
        "BOARD_PREVIOUS_TRACK",
        "BOARD_RIGHT_ALTERNATE",
        "BOARD_RIGHT_BRACKET",
        "BOARD_RIGHT_CONTROL",
        "BOARD_RIGHT_SHIFT",
        "BOARD_RIGHT_SUPER",
        "BOARD_SEMICOLON",
        "BOARD_SLASH",
        "BOARD_SLEEP",
        "BOARD_STOP",
        "BOARD_SYSRQ",
        "BOARD_TAB",
        "BOARD_UNDERLINE",
        "BOARD_NO_LABEL",
        "BOARD_VOLUME_DOWN",
        "BOARD_VOLUME_UP",
        "BOARD_WAKE",
        "BOARD_WEB_BACK",
        "BOARD_WEB_FAVORITES",
        "BOARD_WEB_FORWARD",
        "BOARD_WEB_HOME",
        "BOARD_WEB_REFRESH",
        "BOARD_WEB_SEARCH",
        "BOARD_WEB_STOP",
        "BOARD_YEN",
        "BOARD_COPY",
        "BOARD_PASTE",
        "BOARD_CUT",
    ];

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;

        for (i, key) in Self::KEY_LIST.iter().enumerate() {
            module.constant(key, i).build()?;
        }

        Ok(())
    }

    //================================================================

    fn get_index(state: &State, index: usize) -> anyhow::Result<&Button> {
        if let Some(button) = state.input.board.data.get(index) {
            Ok(button)
        } else {
            Err(anyhow::Error::msg(format!(
                "Board(): Invalid index for board button: {index}"
            )))
        }
    }

    #[rune::function(path = Self::up)]
    /// Get the state of a key-board input (up).
    fn up(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::down)]
    /// Get the state of a key-board input (down).
    fn down(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::press)]
    /// Get the state of a key-board input (press).
    fn press(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.press)
    }

    #[rune::function(path = Self::release)]
    /// Get the state of a key-board input (release).
    fn release(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.release)
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::input)]
struct Mouse {}

impl Mouse {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;
        module.function_meta(Self::point)?;
        module.function_meta(Self::delta)?;
        module.function_meta(Self::wheel)?;
        module.function_meta(Self::show)?;
        module.function_meta(Self::lock)?;

        Ok(())
    }

    //================================================================

    fn get_index(state: &State, index: usize) -> anyhow::Result<&Button> {
        if let Some(button) = state.input.mouse.data.get(index) {
            Ok(button)
        } else {
            Err(anyhow::Error::msg(format!(
                "Mouse(): Invalid index for mouse button: {index}"
            )))
        }
    }

    #[rune::function(path = Self::up)]
    /// Get the state of a mouse input (up).
    fn up(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::down)]
    /// Get the state of a mouse input (down).
    fn down(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::press)]
    /// Get the state of a mouse input (press).
    fn press(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.press)
    }

    #[rune::function(path = Self::release)]
    /// Get the state of a mouse input (release).
    fn release(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.release)
    }

    #[rune::function(path = Self::point)]
    fn point(state: &State) -> Vec2 {
        state.input.mouse.point
    }

    #[rune::function(path = Self::delta)]
    fn delta(state: &State) -> Vec2 {
        state.input.mouse.delta
    }

    #[rune::function(path = Self::wheel)]
    fn wheel(state: &State) -> Vec2 {
        state.input.mouse.wheel
    }

    #[rune::function(path = Self::show)]
    fn show(state: &mut State, value: bool) {
        state.input.window_set.cursor_show = Some(value);
    }

    #[rune::function(path = Self::lock)]
    fn lock(state: &mut State, value: bool) {
        state.input.window_set.cursor_lock = Some(value);
    }
}

//================================================================

#[derive(Any)]
#[rune(item = ::input)]
struct Pad {}

impl Pad {
    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;

        Ok(())
    }

    //================================================================

    fn get_index(state: &State, index: usize) -> anyhow::Result<&Button> {
        if let Some(button) = state.input.pad.data.get(index) {
            Ok(button)
        } else {
            Err(anyhow::Error::msg(format!(
                "Pad(): Invalid index for pad button: {index}"
            )))
        }
    }

    #[rune::function(path = Self::up)]
    /// Get the state of a pad input (up).
    fn up(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::down)]
    /// Get the state of a pad input (down).
    fn down(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.down)
    }

    #[rune::function(path = Self::press)]
    /// Get the state of a pad input (press).
    fn press(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.press)
    }

    #[rune::function(path = Self::release)]
    /// Get the state of a pad input (release).
    fn release(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.release)
    }
}

//================================================================

#[rune::module(::input)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    Board::module(&mut module)?;
    Mouse::module(&mut module)?;
    Pad::module(&mut module)?;

    Ok(module)
}
