// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;
use qrate::{ SQLiteDB, QBank, SBank, QBDB, SBDB };
use qrate::generator::Generator;
use crate::{ AbstractDB, ChoiceAnswer, ErrorMessage, choice_answer };



#[wasm_bindgen]
pub struct ControlTower
{
    question_db: AbstractDB,
    student_db: AbstractDB,
    qbank: Option<QBank>,
    sbank: Option<SBank>,
    generator: Option<Generator>,
}

#[wasm_bindgen]
impl ControlTower
{
    // pub fn new() -> Self
    /// Creates a new instance of `ControlTower` with default values.
    /// 
    /// The `question_db` and `student_db` fields are initialized
    /// to `AbstractDB::None`, and the `qbank`, `sbank`, and `generator`
    /// fields are initialized to `None`.
    /// 
    /// # Returns
    /// A new `ControlTower` instance with default values.
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert!(control_tower.question_db.is_none());
    /// assert!(control_tower.student_db.is_none());
    /// assert!(control_tower.qbank.is_none());
    /// assert!(control_tower.sbank.is_none());
    /// assert!(control_tower.generator.is_none());
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        ControlTower
        {
            question_db: AbstractDB::None,
            student_db: AbstractDB::None,
            qbank: None,
            sbank: None,
            generator: None,
        }
    }

    // pub fn set_qbank_from_bytes_in_sqlite(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    /// Loads the question bank (QBank) from a byte slice
    /// containing SQLite database data.
    ///
    /// This method attempts to create an in-memory SQLite database
    /// from the provided byte data and read the QBank from it.
    /// 
    /// If successful, it sets the `question_db` field to the new SQLite
    /// database and returns `Ok(())`.
    /// If it fails at any point, it returns an `Err` with an appropriate error.
    /// 
    /// # Arguments
    /// * `data` - A byte slice containing the SQLite database data
    ///  for the question bank
    /// 
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(ErrorMessage)` describing the failure on error.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// use qrate::SQLiteDB;
    /// use std::fs;
    /// 
    /// let mut control_tower = ControlTower::new();
    /// let data = fs::read("path_to_qbank.sqlite").expect("Failed to read file");
    /// match control_tower.set_qbank_from_bytes_in_sqlite(&data)
    /// {
    ///     Ok(()) => println!("QBank loaded successfully"),
    ///     Err(e) => println!("Failed to load QBank: {:?}", e),
    /// }
    /// ```
    pub fn set_qbank_from_bytes_in_sqlite(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.qbank = db.read_qbank();
            if self.qbank.is_some()
            {
                self.question_db = AbstractDB::SQLite(db);
                return Ok(());
            }
        }
        Err(ErrorMessage::FailedToRecevieQBankFromMemory)
    }

    // pub fn set_sbank_from_bytes_in_sqlite(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    /// Loads the student bank (SBank) from a byte slice
    /// containing SQLite database data.
    ///
    /// This method attempts to create an in-memory SQLite database
    /// from the provided byte data and read the SBank from it.
    /// 
    /// If successful, it sets the `student_db` field to the new SQLite
    /// database and returns `Ok(())`.
    /// If it fails at any point, it returns an `Err` with an appropriate error.
    /// 
    /// # Arguments
    /// * `data` - A byte slice containing the SQLite database data
    ///  for the student bank
    /// 
    /// # Returns
    /// - `Ok(())` on success
    /// - `Err(ErrorMessage)` describing the failure on error.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// use qrate::SQLiteDB;
    /// use std::fs;
    /// 
    /// let mut control_tower = ControlTower::new();
    /// let data = fs::read("path_to_sbank.sqlite").expect("Failed to read file");
    /// match control_tower.set_sbank_from_bytes_in_sqlite(&data)
    /// {
    ///     Ok(()) => println!("SBank loaded successfully"),
    ///     Err(e) => println!("Failed to load SBank: {:?}", e),
    /// }
    /// ```
    pub fn set_sbank_from_bytes_in_sqlite(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.sbank = db.read_sbank();
            if self.sbank.is_some()
            {
                self.student_db = AbstractDB::SQLite(db);
                return Ok(());
            }
        }
        Err(ErrorMessage::FailedToRecevieSBankFromMemory)
    }

    pub fn generate_pdf(&self) -> Result<Vec<u8>, String>
    {
        if let (Some(_qbank), Some(_sbank)) = (&self.qbank, &self.sbank)
        {
            // TODO: Implement buffer-based PDF generation
            Ok(vec![])
        }
        else
        {
            Err("QBank or SBank not loaded".to_string())
        }
    }

    // pub fn get_question_length(&self) -> usize
    /// Returns the number of questions in the question bank (QBank).
    /// 
    /// If the QBank is not loaded, it returns 0.
    /// 
    /// # Returns
    /// - The number of questions in the QBank if it is loaded and has questions.
    /// - `0` if the QBank has no questions or is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_question_length(), 0);
    /// // After loading a QBank with 10 questions
    /// // assert_eq!(control_tower.get_question_length(), 10);
    /// ``` 
    pub fn get_question_length(&self) -> usize
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_length(),
            None => 0
        }
    }

    // pub fn get_question(&self, question_number: usize) -> String
    /// Retrieves the question text for a given question number from the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns an empty string.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve (1-based).
    /// 
    /// # Returns
    /// - The question text as a `String` if the QBank is loaded
    ///   and the question number is valid.
    /// - An empty string if the QBank is not loaded or the question number
    ///   is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_question(1), "");
    /// // After loading a QBank with a question at index 0 with text "What is 2+2?"
    /// // assert_eq!(control_tower.get_question(1), "What is 2+2?");
    /// ```
    pub fn get_question(&self, question_number: usize) -> String
    {
        match &self.qbank
        {
            Some(qbank) => {
                match qbank.get_question(question_number)
                {
                    Some(question) => question.get_question().clone(),
                    None => String::new()
                }
            },
            None => String::new()
        }
    }

    // pub fn set_question(&self, question_number: usize, txt: String) -> bool  
    /// Sets the question text for a given question number in the QBank.
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it updates the question text and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to set (1-based).
    /// * `txt` - The new question text to set for the specified question.
    /// 
    /// # Returns
    /// - `true` if the question text was successfully updated.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.set_question(1, "What is 2+2?".to_string()), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.set_question(1, "What is 2+2?".to_string()), true);
    /// ```
    pub fn set_question(&self, question_number: usize, txt: String) -> bool
    {
        match &self.qbank
        {
            Some(qbank) => {
                match qbank.get_question(question_number)
                {
                    Some(question) => {
                        question.set_question(txt);
                        true
                    },
                    None => false
                }
            },
            None => false
        }
    }

    // pub fn get_choices_length(&self, question_number: usize) -> usize
    /// Retrieves the number of choices for a given question number from the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns 0.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve choices for (1-based).
    /// 
    /// # Returns
    /// - The number of choices for the specified question if the QBank is loaded
    ///   and the question number is valid.
    /// - `0` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_choices_length(1), 0);
    /// // After loading a QBank with a question at index 0 that has 4 choices
    /// // assert_eq!(control_tower.get_choices_length(1), 4);
    /// ```
    pub fn get_choices_length(&self, question_number: usize) -> usize
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_choices_length(question_number),
            None => 0
        }
    }

    // pub fn get_choice(&self, question_number: usize, choice_number: usize) -> ChoiceAnswer
    /// Retrieves the text of a specific choice for a given question number
    /// from the QBank.
    ///
    ///  If the QBank is not loaded, the question number is out of bounds,
    ///  or the choice number is out of bounds, it returns a `ChoiceAnswer`
    /// instance with an empty text and `false` correctness flag.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve the choice for (1-based).
    /// * `choice_number` - The index of the choice to retrieve (1-based).
    /// 
    /// # Returns
    /// A `ChoiceAnswer` instance containing the text and correctness flag of
    /// the specified choice.
    /// - If the QBank is not loaded, the question number is invalid,
    ///   or the choice number is invalid,
    ///   it returns a `ChoiceAnswer` instance with an empty text and
    ///   `false` correctness flag.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_choice(1, 1), ChoiceAnswer::new(String::new(), false));
    /// // After loading a QBank with a question at index 0 that has a choice at index 0 with text "4"
    /// // assert_eq!(control_tower.get_choice(1, 1), ChoiceAnswer::new("4".to_string(), true));
    /// ```
    pub fn get_choice(&self, question_number: usize, choice_number: usize) -> ChoiceAnswer
    {
        match &self.qbank
        {
            Some(qbank) => {
                match qbank.get_choice(question_number, choice_number)
                {
                    Some(choice) => { ChoiceAnswer::new(choice.0.clone(), choice.1) }
                    None => ChoiceAnswer::new(String::new(), false)
                }
            },
            None => ChoiceAnswer::new(String::new(), false)
        }
    }

    // pub fn set_choice(&self, question_number: usize, choice_number: usize,  choice_answer: ChoiceAnswe) -> bool
    /// Sets the text and correctness flag of a specific choice for a given question number
    /// in the QBank.
    /// 
    /// If the QBank is not loaded, the question number is out of bounds,
    /// or the choice number is out of bounds, it returns `false`. Otherwise,
    /// it updates the choice and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to set the choice for (1-based).
    /// * `choice_number` - The index of the choice to set (1-based).
    /// * `choice_answer` - A `ChoiceAnswer` instance containing the new text
    ///   and correctness flag for the specified choice.
    /// 
    /// # Returns
    /// - `true` if the choice was successfully updated.
    /// - `false` if the QBank is not loaded, the question number is invalid,
    ///   or the choice number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.set_choice(1, 1, ChoiceAnswer::new("4".to_string(), true)), false);
    /// // After loading a QBank with a question at index 0 that has a choice at index 0
    /// // assert_eq!(control_tower.set_choice(1, 1, ChoiceAnswer::new("4".to_string(), true)), true);
    /// ```
    pub fn set_choice(&self, question_number: usize, choice_number: usize,  choice_answer: ChoiceAnswe) -> bool
    {
        match &self.qbank
        {
            Some(qbank) => qbank.set_choice(question_number, choice_number, (choice_answer.get_text(), choice_answer.is_correct())),
            None => false
        }
    }

    // pub fn get_group(&self, question_number: usize) -> u16
    /// Retrieves the group number for a given question number from the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `0`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve the group for (1-based).
    /// 
    /// # Returns
    /// - The group number for the specified question if the QBank is loaded
    ///   and the question number is valid.
    /// - `0` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_group(1), 0);
    /// // After loading a QBank with a question at index 0 that belongs to group 1
    /// // assert_eq!(control_tower.get_group(1), 1);
    /// ```
    pub fn get_group(&self, question_number: usize) -> u16
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_group(question_number),
            None => 0
        }
    }

    // pub fn set_group(&self, question_number: usize, group: u16) -> bool
    /// Sets the group number for a given question number in the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it updates the group number and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to set the group for (1-based).
    /// * `group` - The new group number to set for the specified question.
    /// 
    /// # Returns
    /// - `true` if the group number was successfully updated.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.set_group(1, 1), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.set_group(1, 1), true);
    /// ```
    pub fn set_group(&self, question_number: usize, group: u16) -> bool
    {
        match &self.qbank
        {
            Some(qbank) => qbank.set_group(question_number, group),
            None => false
        }
    }
}