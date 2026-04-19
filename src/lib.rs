// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


/// The core logic and state management for the Qrate-GUI application.
mod control_tower;
mod load_file;

/// Re-exports the main application components for external use.
pub use control_tower::{ ControlTower, Message };
pub use load_file::{ LoadFile, ResultLoadFile };

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_exam_wasm(input_data: &str) -> String
{
    format!("Generated Exam {}", str)
}
