// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;
use crate::ChoiceMark;


/// Represents the data for a quiz question, including the question number,
/// category, question text, and a list of choices. This struct provides methods
/// to create a new `QuestionData` instance, retrieve the question number,
/// category, question text, and choices. The `QuestionData` struct is designed
/// to be used in a WebAssembly context, allowing it to be easily
#[wasm_bindgen]
pub struct QuestionData
{
    num: u16,                   // The question number.
    category_id: u8,            // The category ID of the question.
    category_str: String,       // The category string of the question.
    question: String,           // The text of the question.
    choices: Vec<ChoiceMark>,   // A list of choices for the question.
}

#[wasm_bindgen]
impl QuestionData
{
    // pub fn new(num: u16, category_id: u8, category_str: String, question: String) -> Self
    /// Creates a new `QuestionData` instance.
    /// 
    /// # Arguments
    /// * `num` - The question number.
    /// * `category_id` - The category ID of the question.
    /// * `category_str` - The category string of the question.
    /// * `question` - The text of the question.
    /// 
    /// # Returns
    /// A new `QuestionData` instance.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(num: u16, category_id: u8, category_str: String, question: String) -> Self
    {
        Self { num, category_id, category_str, question, choices: Vec::new() }
    }

    // pub fn get_num(&self) -> u16
    /// Retrieves the question number.
    /// 
    /// # Returns
    /// - `u16`: The question number.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_num(), 1);
    /// ```
    pub fn get_num(&self) -> u16
    {
        self.num
    }

    // pub fn get_category_id(&self) -> u8
    /// Retrieves the category of the question.
    /// 
    /// # Returns
    /// - `u8`: The category ID of the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, 1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_category_id(), 1);
    /// ```
    pub fn get_category_id(&self) -> u8
    {
        self.category_id
    }

    // pub fn get_category_string(&self) -> String
    /// Retrieves the category of the question.
    /// 
    /// # Returns
    /// - `String`: The category string of the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, 1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_category_string(), "Category");
    /// ```
    pub fn get_category_string(&self) -> String
    {
        self.category_str.clone()
    }

    // pub fn get_question(&self) -> String
    /// Retrieves the text of the question.
    /// 
    /// # Returns
    /// - `String`: The text of the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_text(), "Text");
    /// ```
    pub fn get_question(&self) -> String
    {
        self.question.clone()
    }
    
    // pub fn get_choices_length(&self) -> usize
    /// Retrieves the number of choices in the question.
    /// 
    /// # Returns
    /// - `usize`: The number of choices in the question.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_choices_length(), 0);
    /// ```
    pub fn get_choices_length(&self) -> usize
    {
        self.choices.len()
    }

    // pub fn get_choice(&self, index: usize) -> ChoiceMark
    /// Retrieves a specific choice from the question.
    /// 
    /// # Arguments
    /// * `index` - The index of the choice to retrieve (0-based).
    /// 
    /// # Returns
    /// - `ChoiceMark`: The choice at the specified index.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// assert_eq!(qdata.get_choice(0), ChoiceMark::new(String::new(), false));
    /// ```
    pub fn get_choice(&self, index: usize) -> ChoiceMark
    {
        self.choices[index].clone()
    }
    
    // pub fn push_choice(&mut self, text: String, is_correct: bool)
    /// Adds a new choice to the question.
    /// 
    /// # Arguments
    /// * `text` - The text of the new choice.
    /// * `is_correct` - The correctness flag for the new choice.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::QuestionData;
    /// let mut qdata = QuestionData::new(1, "Category".to_string(), "Text".to_string());
    /// qdata.push_choice("4".to_string(), true);
    /// assert_eq!(qdata.get_choice(0), ChoiceMark::new("4".to_string(), true));
    /// ```
    pub(crate) fn push_choice(&mut self, text: String, is_correct: bool)
    {
        self.choices.push(ChoiceMark::new(text, is_correct));
    }
}
