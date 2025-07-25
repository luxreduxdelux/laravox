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

use rune::Module;
use std::fs;

//================================================================

#[rune::module(::file)]
pub fn module() -> anyhow::Result<Module> {
    let mut module = Module::from_meta(self::module_meta)?;

    module.function_meta(canonicalize)?;
    module.function_meta(copy)?;
    module.function_meta(create_path)?;
    module.function_meta(create_path_all)?;
    module.function_meta(check)?;
    module.function_meta(read)?;
    //module.function_meta(read_dir)?;
    //module.function_meta(read_link)?;
    module.function_meta(read_to_string)?;
    module.function_meta(remove_path)?;
    module.function_meta(remove_path_all)?;
    module.function_meta(remove_file)?;
    module.function_meta(rename)?;
    //module.function_meta(set_permissions)?;
    //module.function_meta(symlink_metadata)?;
    //module.function_meta(write)?;

    Ok(module)
}

//================================================================

/// Canonicalize a path.
#[rune::function]
#[inline]
fn canonicalize(path: String) -> anyhow::Result<String> {
    Ok(fs::canonicalize(&path).map(|x| x.display().to_string())?)
}

/// Copy a file to a different path.
#[rune::function]
#[inline]
fn copy(from: String, to: String) -> anyhow::Result<u64> {
    Ok(fs::copy(&from, &to)?)
}

/// Create a new folder.
#[rune::function]
#[inline]
fn create_path(path: String) -> anyhow::Result<()> {
    Ok(fs::create_dir(&path)?)
}

/// Create a new folder, recursively creating a folder (or more) inside of the first folder.
#[rune::function]
#[inline]
fn create_path_all(path: String) -> anyhow::Result<()> {
    Ok(fs::create_dir_all(&path)?)
}

/// Check if a path does exist.
#[rune::function]
#[inline]
fn check(path: String) -> anyhow::Result<bool> {
    Ok(fs::exists(&path)?)
}

/// Read the data of a file as binary data.
#[rune::function]
#[inline]
fn read(path: String) -> anyhow::Result<Vec<u8>> {
    Ok(fs::read(&path)?)
}

/// Read the data of a file as a string.
#[rune::function]
#[inline]
fn read_to_string(path: String) -> anyhow::Result<String> {
    Ok(fs::read_to_string(&path)?)
}

/// Remove a folder, which is empty.
#[rune::function]
#[inline]
fn remove_path(path: String) -> anyhow::Result<()> {
    Ok(fs::remove_dir(&path)?)
}

/// Remove a folder, which may not be empty.
#[rune::function]
#[inline]
fn remove_path_all(path: String) -> anyhow::Result<()> {
    Ok(fs::remove_dir_all(&path)?)
}

/// Remove a file.
#[rune::function]
#[inline]
fn remove_file(path: String) -> anyhow::Result<()> {
    Ok(fs::remove_file(&path)?)
}

/// Rename a file, or folder.
#[rune::function]
#[inline]
fn rename(from: String, to: String) -> anyhow::Result<()> {
    Ok(fs::rename(&from, &to)?)
}
