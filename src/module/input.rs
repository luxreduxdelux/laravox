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

use rune::{Any, Module, docstring};

use super::general::Vec2;

//================================================================

/// Accessor for getting data about the current board.
#[derive(Any)]
#[rune(item = ::input)]
struct Board {}

impl Board {
    const LIST_KEY: [&str; 163] = [
        "KEY_1",
        "KEY_2",
        "KEY_3",
        "KEY_4",
        "KEY_5",
        "KEY_6",
        "KEY_7",
        "KEY_8",
        "KEY_9",
        "KEY_0",
        "KEY_A",
        "KEY_B",
        "KEY_C",
        "KEY_D",
        "KEY_E",
        "KEY_F",
        "KEY_G",
        "KEY_H",
        "KEY_I",
        "KEY_J",
        "KEY_K",
        "KEY_L",
        "KEY_M",
        "KEY_N",
        "KEY_O",
        "KEY_P",
        "KEY_Q",
        "KEY_R",
        "KEY_S",
        "KEY_T",
        "KEY_U",
        "KEY_V",
        "KEY_W",
        "KEY_X",
        "KEY_Y",
        "KEY_Z",
        "KEY_ESCAPE",
        "KEY_F1",
        "KEY_F2",
        "KEY_F3",
        "KEY_F4",
        "KEY_F5",
        "KEY_F6",
        "KEY_F7",
        "KEY_F8",
        "KEY_F9",
        "KEY_F10",
        "KEY_F11",
        "KEY_F12",
        "KEY_F13",
        "KEY_F14",
        "KEY_F15",
        "KEY_F16",
        "KEY_F17",
        "KEY_F18",
        "KEY_F19",
        "KEY_F20",
        "KEY_F21",
        "KEY_F22",
        "KEY_F23",
        "KEY_F24",
        "KEY_SNAPSHOT",
        "KEY_SCROLL",
        "KEY_PAUSE",
        "KEY_INSERT",
        "KEY_HOME",
        "KEY_DELETE",
        "KEY_END",
        "KEY_PAGE_DOWN",
        "KEY_PAGE_UP",
        "KEY_LEFT",
        "KEY_UP",
        "KEY_RIGHT",
        "KEY_DOWN",
        "KEY_BACK",
        "KEY_RETURN",
        "KEY_SPACE",
        "KEY_COMPOSE",
        "KEY_CARET",
        "KEY_NUMBER_LOCK",
        "KEY_NUMBER_PAD_0",
        "KEY_NUMBER_PAD_1",
        "KEY_NUMBER_PAD_2",
        "KEY_NUMBER_PAD_3",
        "KEY_NUMBER_PAD_4",
        "KEY_NUMBER_PAD_5",
        "KEY_NUMBER_PAD_6",
        "KEY_NUMBER_PAD_7",
        "KEY_NUMBER_PAD_8",
        "KEY_NUMBER_PAD_9",
        "KEY_NUMBER_PAD_ADD",
        "KEY_NUMBER_PAD_DIVIDE",
        "KEY_NUMBER_PAD_DECIMAL",
        "KEY_NUMBER_PAD_COMMA",
        "KEY_NUMBER_PAD_ENTER",
        "KEY_NUMBER_PAD_EQUAL",
        "KEY_NUMBER_PAD_MULTIPLY",
        "KEY_NUMBER_PAD_SUBTRACT",
        "KEY_ABNTC1",
        "KEY_ABNTC2",
        "KEY_APOSTROPHE",
        "KEY_APPS",
        "KEY_ASTERISK",
        "KEY_AT",
        "KEY_AX",
        "KEY_BACKSLASH",
        "KEY_CALCULATOR",
        "KEY_CAPITAL",
        "KEY_COLON",
        "KEY_COMMA",
        "KEY_CONVERT",
        "KEY_EQUALS",
        "KEY_GRAVE",
        "KEY_KANA",
        "KEY_KANJI",
        "KEY_LEFT_ALTERNATE",
        "KEY_LEFT_BRACKET",
        "KEY_LEFT_CONTROL",
        "KEY_LEFT_SHIFT",
        "KEY_LEFT_SUPER",
        "KEY_MAIL",
        "KEY_MEDIA_SELECT",
        "KEY_MEDIA_STOP",
        "KEY_MINUS",
        "KEY_MUTE",
        "KEY_MY_COMPUTER",
        "KEY_NAVIGATE_FORWARD",
        "KEY_NAVIGATE_BACKWARD",
        "KEY_NEXT_TRACK",
        "KEY_NO_CONVERT",
        "KEY_OEM102",
        "KEY_PERIOD",
        "KEY_PLAY_PAUSE",
        "KEY_PLUS",
        "KEY_POWER",
        "KEY_PREVIOUS_TRACK",
        "KEY_RIGHT_ALTERNATE",
        "KEY_RIGHT_BRACKET",
        "KEY_RIGHT_CONTROL",
        "KEY_RIGHT_SHIFT",
        "KEY_RIGHT_SUPER",
        "KEY_SEMICOLON",
        "KEY_SLASH",
        "KEY_SLEEP",
        "KEY_STOP",
        "KEY_SYSRQ",
        "KEY_TAB",
        "KEY_UNDERLINE",
        "KEY_NO_LABEL",
        "KEY_VOLUME_DOWN",
        "KEY_VOLUME_UP",
        "KEY_WAKE",
        "KEY_WEB_BACK",
        "KEY_WEB_FAVORITES",
        "KEY_WEB_FORWARD",
        "KEY_WEB_HOME",
        "KEY_WEB_REFRESH",
        "KEY_WEB_SEARCH",
        "KEY_WEB_STOP",
        "KEY_YEN",
        "KEY_COPY",
        "KEY_PASTE",
        "KEY_CUT",
    ];

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;
        module.function_meta(Self::last_press)?;
        module.function_meta(Self::last_release)?;
        module.function_meta(Self::key_name)?;

        for (i, key) in Self::LIST_KEY.iter().enumerate() {
            module
                .constant(*key, i)
                .build_associated::<Self>()?
                .docs(docstring! {
                    /// Board key.
                })?;
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

    /// Get the state of a board input (up).
    #[rune::function(path = Self::up)]
    fn up(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, index)?.down)
    }

    /// Get the state of a board input (down).
    #[rune::function(path = Self::down)]
    fn down(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.down)
    }

    /// Get the state of a board input (press).
    #[rune::function(path = Self::press)]
    fn press(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.press)
    }

    /// Get the state of a board input (release).
    #[rune::function(path = Self::release)]
    fn release(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.release)
    }

    /// Get the last key press.
    #[rune::function(path = Self::last_press)]
    fn last_press(state: &State) -> Option<usize> {
        state.input.board.last_press
    }

    /// Get the last key release.
    #[rune::function(path = Self::last_release)]
    fn last_release(state: &State) -> Option<usize> {
        state.input.board.last_release
    }

    /// Get a human-readable name for a key.
    #[rune::function(path = Self::key_name)]
    fn key_name(key: usize) -> anyhow::Result<String> {
        if let Some(name) = Self::LIST_KEY.get(key) {
            Ok(name.to_string())
        } else {
            Err(anyhow::Error::msg(format!(
                "Board::key_name(): Invalid index for board button: {key}"
            )))
        }
    }
}

//================================================================

/// Accessor for getting data about the current mouse.
#[derive(Any)]
#[rune(item = ::input)]
struct Mouse {}

impl Mouse {
    const LIST_KEY: [&str; 5] = [
        "KEY_LEFT",
        "KEY_RIGHT",
        "KEY_MIDDLE",
        "KEY_BACK",
        "KEY_FORWARD",
    ];

    const LIST_ICON: [&str; 34] = [
        "ICON_DEFAULT",
        "ICON_CONTEXT_MENU",
        "ICON_HELP",
        "ICON_POINTER",
        "ICON_PROGRESS",
        "ICON_WAIT",
        "ICON_CELL",
        "ICON_CROSSHAIR",
        "ICON_TEXT",
        "ICON_VERTICAL_TEXT",
        "ICON_ALIAS",
        "ICON_COPY",
        "ICON_MOVE",
        "ICON_NO_DROP",
        "ICON_NOT_ALLOWED",
        "ICON_GRAB",
        "ICON_GRABBING",
        "ICON_E_RESIZE",
        "ICON_N_RESIZE",
        "ICON_NE_RESIZE",
        "ICON_NW_RESIZE",
        "ICON_SR_ESIZE",
        "ICON_SE_RESIZE",
        "ICON_SW_RESIZE",
        "ICON_W_RESIZE",
        "ICON_EW_RESIZE",
        "ICON_NS_RESIZE",
        "ICON_NE_SW_RESIZE",
        "ICON_NW_SERESIZE",
        "ICON_COL_RESIZE",
        "ICON_ROW_RESIZE",
        "ICON_ALL_SCROLL",
        "ICON_ZOOM_IN",
        "ICON_ZOOM_OUT",
    ];

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;
        module.function_meta(Self::last_press)?;
        module.function_meta(Self::last_release)?;
        module.function_meta(Self::point)?;
        module.function_meta(Self::delta)?;
        module.function_meta(Self::wheel)?;
        module.function_meta(Self::icon)?;
        module.function_meta(Self::show)?;
        module.function_meta(Self::lock)?;

        for (i, key) in Self::LIST_KEY.iter().enumerate() {
            module
                .constant(*key, i)
                .build_associated::<Self>()?
                .docs(docstring! {
                    /// Mouse key.
                })?;
        }

        for (i, key) in Self::LIST_ICON.iter().enumerate() {
            module
                .constant(*key, i)
                .build_associated::<Self>()?
                .docs(docstring! {
                    /// Mouse icon.
                })?;
        }

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

    /// Get the state of a mouse input (up).
    #[rune::function(path = Self::up)]
    fn up(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, index)?.down)
    }

    /// Get the state of a mouse input (down).
    #[rune::function(path = Self::down)]
    fn down(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.down)
    }

    /// Get the state of a mouse input (press).
    #[rune::function(path = Self::press)]
    fn press(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.press)
    }

    /// Get the state of a mouse input (release).
    #[rune::function(path = Self::release)]
    fn release(state: &State, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, index)?.release)
    }

    /// Get the last key press.
    #[rune::function(path = Self::last_press)]
    fn last_press(state: &State) -> Option<usize> {
        state.input.mouse.last_press
    }

    /// Get the last key release.
    #[rune::function(path = Self::last_release)]
    fn last_release(state: &State) -> Option<usize> {
        state.input.mouse.last_release
    }

    /// Get the point of the mouse cursor on screen.
    #[rune::function(path = Self::point)]
    fn point(state: &State) -> Vec2 {
        state.input.mouse.point
    }

    /// Get the delta of the mouse cursor in the last frame.
    #[rune::function(path = Self::delta)]
    fn delta(state: &State) -> Vec2 {
        state.input.mouse.delta
    }

    /// Get the delta of the mouse wheel in the last frame.
    #[rune::function(path = Self::wheel)]
    fn wheel(state: &State) -> Vec2 {
        state.input.mouse.wheel
    }

    /// Set the mouse cursor icon. Index must be in the range of [0, 33]. Refer to the Mouse::ICON_* constant family.
    #[rune::function(path = Self::icon)]
    fn icon(state: &mut State, index: usize) -> anyhow::Result<()> {
        if index < Self::LIST_ICON.len() {
            state.input.window_set.cursor_icon = Some(index);
            return Ok(());
        }

        Err(anyhow::Error::msg(format!(
            "Mouse::icon(): Invalid index for mouse cursor: {index}"
        )))
    }

    /// Show the mouse cursor.
    #[rune::function(path = Self::show)]
    fn show(state: &mut State, value: bool) {
        state.input.window_set.cursor_show = Some(value);
    }

    /// Lock the mouse cursor.
    #[rune::function(path = Self::lock)]
    fn lock(state: &mut State, value: bool) {
        state.input.window_set.cursor_lock = Some(value);
    }
}

//================================================================

/// Accessor for getting data about a pad.
#[derive(Any)]
#[rune(item = ::input)]
struct Pad {}

impl Pad {
    const LIST_KEY: [&str; 20] = [
        "KEY_SOUTH",
        "KEY_EAST",
        "KEY_NORTH",
        "KEY_WEST",
        "KEY_C",
        "KEY_Z",
        "KEY_LEFT_BUMPER",
        "KEY_LEFT_TRIGGER",
        "KEY_RIGHT_BUMPER",
        "KEY_RIGHT_TRIGGER",
        "KEY_SELECT",
        "KEY_START",
        "KEY_MODE",
        "KEY_LEFT_THUMB",
        "KEY_RIGHT_THUMB",
        "KEY_UP",
        "KEY_DOWN",
        "KEY_LEFT",
        "KEY_RIGHT",
        "KEY_UNKNOWN",
    ];

    const LIST_AXIS: [&str; 9] = [
        "AXIS_LEFT_STICK_X",
        "AXIS_LEFT_STICK_Y",
        "AXIS_LEFT_TRIGGER",
        "AXIS_RIGHT_STICK_X",
        "AXIS_RIGHT_STICK_Y",
        "AXIS_RIGHT_TRIGGER",
        "AXIS_X",
        "AXIS_Y",
        "AXIS_UNKNOWN",
    ];

    fn module(module: &mut Module) -> anyhow::Result<()> {
        module.ty::<Self>()?;

        module.function_meta(Self::up)?;
        module.function_meta(Self::down)?;
        module.function_meta(Self::press)?;
        module.function_meta(Self::release)?;
        module.function_meta(Self::axis)?;
        module.function_meta(Self::name)?;

        for (i, key) in Self::LIST_KEY.iter().enumerate() {
            module
                .constant(*key, i)
                .build_associated::<Self>()?
                .docs(docstring! {
                    /// Pad key.
                })?;
        }

        for (i, key) in Self::LIST_AXIS.iter().enumerate() {
            module
                .constant(*key, i)
                .build_associated::<Self>()?
                .docs(docstring! {
                    /// Pad axis.
                })?;
        }

        Ok(())
    }

    //================================================================

    fn get_index(state: &State, which: usize, index: usize) -> anyhow::Result<&Button> {
        if let Some((_, pad)) = state.input.pad.get_pad(which)
            && let Some(button) = pad.button.get(index)
        {
            Ok(button)
        } else {
            Err(anyhow::Error::msg(format!(
                "Pad(): Invalid index for pad button: {index}"
            )))
        }
    }

    /// Get the state of a pad input (up).
    #[rune::function(path = Self::up)]
    fn up(state: &State, which: usize, index: usize) -> anyhow::Result<bool> {
        Ok(!Self::get_index(state, which, index)?.down)
    }

    /// Get the state of a pad input (down).
    #[rune::function(path = Self::down)]
    fn down(state: &State, which: usize, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, which, index)?.down)
    }

    /// Get the state of a pad input (press).
    #[rune::function(path = Self::press)]
    fn press(state: &State, which: usize, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, which, index)?.press)
    }

    /// Get the state of a pad input (release).
    #[rune::function(path = Self::release)]
    fn release(state: &State, which: usize, index: usize) -> anyhow::Result<bool> {
        Ok(Self::get_index(state, which, index)?.release)
    }

    /// Get the state of a pad axis. `index` must be in the range of [0, 8]. Refer to the Pad::AXIS_* constant family.
    #[rune::function(path = Self::axis)]
    fn axis(state: &State, which: usize, index: usize) -> anyhow::Result<f32> {
        if let Some((_, pad)) = state.input.pad.get_pad(which)
            && let Some(axis) = pad.axis.get(index)
        {
            return Ok(*axis);
        }

        Err(anyhow::Error::msg(format!(
            "Pad(): Invalid index for pad: {which}"
        )))
    }

    /// Get the human-readable name of a pad.
    #[rune::function(path = Self::name)]
    fn name(state: &State, which: usize) -> anyhow::Result<String> {
        if let Some((identifier, _)) = state.input.pad.get_pad(which) {
            let pad = state.input.pad.handle.gamepad(*identifier);

            return Ok(pad.name().to_string());
        }

        Err(anyhow::Error::msg(format!(
            "Pad(): Invalid index for pad: {which}"
        )))
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
