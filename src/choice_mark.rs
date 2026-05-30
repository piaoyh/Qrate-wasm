// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use wasm_bindgen::prelude::*;

/// Represents a choice answer for a quiz question, containing the text of the
/// choice and a flag indicating whether it is the correct answer. This struct
/// provides methods to create a new choice answer, retrieve the text, and check
/// if it is correct. The `ChoiceMark` struct is designed to be used in a
/// WebAssembly context, allowing it to be easily manipulated from JavaScript.
#[wasm_bindgen]
#[derive(Clone)]
pub struct ChoiceMark
{
    text: String,       // The text of the choice answer.
    is_correct: bool,   // A flag indicating whether this choice is the correct answer.
}


#[wasm_bindgen]
impl ChoiceMark
{
    // pub fn new(text: String, is_correct: bool) -> Self
    /// Creates a new `ChoiceAnswer` instance.
    ///
    /// # Arguments
    /// * `text` - The text of the choice answer.
    /// * `is_correct` - A boolean indicating whether this choice is the correct answer.
    ///
    /// # Returns
    /// `Self` - A new instance of `ChoiceAnswer`.
    ///
    /// # Examples
    /// ```
    /// use qrate::ChoiceAnswer;
    /// let choice = ChoiceAnswer::new("Option A".to_string(), true);
    /// assert_eq!(choice.text, "Option A");
    /// assert!(choice.is_correct);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(text: String, is_correct: bool) -> Self
    {
        Self { text, is_correct }
    }

    // pub fn get_text(&self) -> String
    /// Retrieves the text of the choice answer.
    /// 
    /// # Returns
    /// A `String` containing the text of the choice answer.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ChoiceAnswer;
    /// let choice = ChoiceAnswer::new("Option A".to_string(), true);
    /// assert_eq!(choice.get_text(), "Option A");
    /// ```
    pub fn get_text(&self) -> String
    {
        self.text.clone()   // Return a clone to avoid ownership issue
    }

    // pub fn is_correct(&self) -> bool
    /// Retrieves the correctness flag of the choice answer.
    /// 
    /// # Returns
    /// A `bool` indicating whether this choice is the correct answer.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ChoiceAnswer;
    /// let choice = ChoiceAnswer::new("Option A".to_string(), true);
    /// assert!(choice.is_correct());
    /// ```
    pub fn is_correct(&self) -> bool
    {
        self.is_correct
    }
}
