// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

pub mod control_tower;
pub mod error_messages;
pub mod abstract_db;
pub mod choice_answer;
pub mod name_id;

pub use control_tower::ControlTower;
pub use error_messages::ErrorMessage;
pub use abstract_db::AbstractDB;
pub use choice_answer::ChoiceAnswer;
pub use name_id::NameId;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_exam_wasm(input_data: &str) -> String
{
    format!("Generated Exam {}", input_data)
}
