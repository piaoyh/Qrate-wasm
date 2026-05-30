// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

/// A module for generating exams based on a question bank and student bank.
pub mod control_tower;

/// A module containing error messages that can occur in the Qrate application.
pub mod error_messages;

/// A module defining an abstract database structure for storing questions and students.
pub mod abstract_db;

/// A module representing a choice answer for a quiz question, containing the text of the
pub mod choice_mark;

/// A module representing a name and ID pair, commonly used for students and questions.
pub mod name_id;

/// A module representing the data structure for a quiz question, including the question text,
pub mod question_data;

pub use control_tower::ControlTower;
pub use error_messages::ErrorMessage;
pub use abstract_db::AbstractDB;
pub use choice_mark::ChoiceMark;
pub use name_id::NameId;
pub use question_data::QuestionData;

use wasm_bindgen::prelude::*;


/// A simple function to demonstrate WebAssembly integration. This function
/// takes a string as input and returns a formatted string indicating that
/// an exam has been generated with the given input data. This function is
/// designed to be called from JavaScript in a WebAssembly context.
/// 
/// # Arguments
/// * `input_data` - A string containing the input data for generating the exam.
/// 
/// # Returns
/// A `String` indicating that the exam has been generated with the provided input data.
/// 
/// # Examples
/// ```
/// use qrate::generate_exam_wasm;
/// let result = generate_exam_wasm("Sample Data");
/// assert_eq!(result, "Generated Exam Sample Data");
/// ```
#[wasm_bindgen]
pub fn generate_exam_wasm(input_data: &str) -> String
{
    format!("Generated Exam {}", input_data)
}
