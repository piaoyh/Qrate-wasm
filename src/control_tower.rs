// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;
use qrate::{ QBDB, QBank, SBDB, SBank, SQLiteDB, Student, Question, Generator,
                SelfStudy, ScoringRule, UserAnswer };
use crate::{ AbstractDB, ChoiceMark, NameId, QuestionData, ErrorMessage };



#[wasm_bindgen]
pub struct ControlTower
{
    question_db: AbstractDB,
    student_db: AbstractDB,
    qbank: Option<QBank>,
    sbank: Option<SBank>,
    self_study: Option<SelfStudy>,
}

#[wasm_bindgen]
impl ControlTower
{
    // pub fn new() -> Self
    /// Creates a new instance of `ControlTower` with default values.
    /// 
    /// The `question_db` and `student_db` fields are initialized
    /// to `AbstractDB::None`, and the `qbank`, `sbank`, and `self_study`
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
    /// assert!(control_tower.self_study.is_none());
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
            self_study: None,
        }
    }

    /// Clears all database and bank data.
    pub fn clear_all(&mut self)
    {
        self.question_db = AbstractDB::None;
        self.student_db = AbstractDB::None;
        self.qbank = None;
        self.sbank = None;
        self.self_study = None;
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
        self.clear_qbank();
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.qbank = db.read_qbank();
            if let Some(qbank) = &self.qbank
            {
                if qbank.is_higher_version()
                    { return Err(ErrorMessage::InvalidVersion); }
                self.question_db = AbstractDB::SQLite(db);
                return Ok(());
            }
        }
        Err(ErrorMessage::FailedToReceiveQBankFromMemory)
    }

    // pub fn write_qbank_to_bytes_in_sqlite(&self) -> Result<Vec<u8>, ErrorMessage>
    /// Writes the question bank (QBank) to a byte vector containing SQLite
    /// database data.
    /// 
    /// This method creates an in-memory SQLite database, writes the QBank to it,
    /// and then saves the database to a byte vector.
    /// 
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the SQLite database data on success
    /// - `Err(ErrorMessage)` describing the failure on error.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// match control_tower.write_qbank_to_bytes_in_sqlite()
    /// {
    ///     Ok(data) => println!("QBank written to bytes successfully, size: {}", data.len()),
    ///     Err(e) => println!("Failed to write QBank to bytes: {:?}", e),
    /// }
    /// ```
    pub fn write_qbank_to_bytes_in_sqlite(&mut self) -> Result<Vec<u8>, ErrorMessage>
    {
        if let Some(qbank) = &mut self.qbank
        {
            qbank.determine_categories();
            if let Some(mut db) = SQLiteDB::open_empty_in_memory()
            {
                if db.write_qbank(qbank).is_ok()
                    { return db.save_in_memory().map_err(|_| ErrorMessage::FailedToWriteQBankToMemory); }
            }
        }
        Err(ErrorMessage::FailedToWriteQBankToMemory)
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
    ///   for the student bank
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
        self.clear_sbank();
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.sbank = db.read_sbank();
            if let Some(sbank) = &self.sbank
            {
                if sbank.is_higher_version()
                    { return Err(ErrorMessage::InvalidVersion); }
                self.student_db = AbstractDB::SQLite(db);
                return Ok(());
            }
        }
        Err(ErrorMessage::FailedToReceiveSBankFromMemory)
    }

    // pub fn write_sbank_to_bytes_in_sqlite(&self) -> Result<Vec<u8>, ErrorMessage>
    /// Writes the student bank (SBank) to a byte vector containing SQLite
    /// database data.
    /// 
    /// This method creates an in-memory SQLite database, writes the SBank to it,
    /// and then saves the database to a byte vector.
    /// 
    /// # Returns
    /// - `Ok(Vec<u8>)` containing the SQLite database data on success
    /// - `Err(ErrorMessage)` describing the failure on error.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// match control_tower.write_sbank_to_bytes_in_sqlite()
    /// {
    ///     Ok(data) => println!("SBank written to bytes successfully, size: {}", data.len()),
    ///     Err(e) => println!("Failed to write SBank to bytes: {:?}", e),
    /// }
    /// ```
    pub fn write_sbank_to_bytes_in_sqlite(&self) -> Result<Vec<u8>, ErrorMessage>
    {
        if let Some(sbank) = &self.sbank
        {
            if let Some(mut db) = SQLiteDB::open_empty_in_memory()
            {
                if db.write_sbank(sbank).is_ok()
                    { return db.save_in_memory().map_err(|_| ErrorMessage::FailedToWriteSBankToMemory); }
            }
        }
        Err(ErrorMessage::FailedToWriteSBankToMemory)
    }

    // pub fn push_an_empty_question(&mut self)
    /// Pushes an empty question to the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.push_an_empty_question();
    /// assert_eq!(control_tower.get_question_length(), 1);
    /// ```
    pub fn push_an_empty_question(&mut self)
    {
        let mut question = Question::new_empty();
        match &mut self.qbank
        {
            Some(qbank) => {
                let id = qbank.get_length() + 1; // Assuming question numbers are 1-based
                question.set_id(id as u16); // Set the question ID to the new question number
                qbank.push_question(question);
            },
            None => {
                let mut qbank = QBank::new_empty();
                question.set_id(1); // Set the question ID to 1 for the first question
                qbank.push_question(question);
                self.qbank = Some(qbank);
            }
        }
    }

    // pub fn determine_category(&mut self, question_number: usize) -> bool
    /// Determines the category for a given question number in the QBank.
    /// 
    /// This method uses the `determine_category` function of the QBank to
    /// determine the category for the specified question.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question for which to determine the category.
    /// 
    /// # Returns
    /// * `true` if the category was successfully determined and set.
    /// * `false` if the question number is out of bounds or the QBank is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.push_an_empty_question();
    /// control_tower.determine_category(1);
    /// assert!(control_tower.get_category(1), 4);
    /// ```
    pub fn determine_category(&mut self, question_number: usize) -> bool
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.determine_category(question_number) }
        else
            { false}
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
        if let Some(qbank) = &self.qbank
        {
            if let Some(question) = qbank.get_question(question_number)
                { return question.get_question().clone(); }
            
        }
        String::new()
    }

    // pub fn get_question_data(&mut self, num: u16) -> Option<QuestionData>
    /// Returns the question data from question bank (QBank)
    /// for the specified question number.
    /// 
    /// # Arguments
    /// * `num` - The question number to retrieve.
    /// 
    /// # Returns
    /// - `Some(QuestionData)` if the question data is retrieved successfully.
    /// - `None` if the QBank is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate::QBank;
    /// let qbank = QBank::new();
    /// if let Some(qdata) = qbank.get_question_data(1)
    ///     { println!("Question data retrieved successfully"); }
    /// else
    ///     { println!("Failed to retrieve question data"); }
    /// ```
    pub fn get_question_data(&mut self, num: u16) -> Option<QuestionData>
    {
        if let Some(qbank) = &mut self.qbank
        {
            if let Some((num, cat_id, cat_str, question, choices)) = qbank.get_question_data(num)
            {
                let mut qdata = QuestionData::new(num, cat_id, cat_str, question);
                for (c_text, c_correct) in choices
                    { qdata.push_choice(c_text, c_correct); }
                return Some(qdata);
            }
        }
        None
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
    pub fn set_question(&mut self, question_number: usize, txt: String) -> bool
    {
        if let Some(qbank) = &mut self.qbank
        {
            if let Some(question) = qbank.get_question_mut(question_number)
            {
                question.set_question(txt);
                return true;
            }
        }
        false
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

    // pub fn get_choice(&self, question_number: usize, choice_number: usize) -> ChoiceMark
    /// Retrieves the text of a specific choice for a given question number
    /// from the QBank.
    ///
    ///  If the QBank is not loaded, the question number is out of bounds,
    ///  or the choice number is out of bounds, it returns a `ChoiceMark`
    /// instance with an empty text and `false` correctness flag.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve the choice for (1-based).
    /// * `choice_number` - The index of the choice to retrieve (1-based).
    /// 
    /// # Returns
    /// A `ChoiceMark` instance containing the text and correctness flag of
    /// the specified choice.
    /// - If the QBank is not loaded, the question number is invalid,
    ///   or the choice number is invalid,
    ///   it returns a `ChoiceMark` instance with an empty text and
    ///   `false` correctness flag.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_choice(1, 1), ChoiceMark::new(String::new(), false));
    /// // After loading a QBank with a question at index 0 that has a choice at index 0 with text "4"
    /// // assert_eq!(control_tower.get_choice(1, 1), ChoiceMark::new("4".to_string(), true));
    /// ```
    pub fn get_choice(&self, question_number: usize, choice_number: usize) -> ChoiceMark
    {
        if let Some(qbank) = &self.qbank
        {
            if let Some(choice) = qbank.get_choice(question_number, choice_number)
                { return ChoiceMark::new(choice.0.clone(), choice.1); }
        }
        ChoiceMark::new(String::new(), false)
    }

    // pub fn set_choice(&mut self, question_number: usize, choice_number: usize,  choice_answer: ChoiceMark) -> bool
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
    /// * `choice_answer` - A `ChoiceMark` instance containing the new text
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
    /// assert_eq!(control_tower.set_choice(1, 1, ChoiceMark::new("4".to_string(), true)), false);
    /// // After loading a QBank with a question at index 0 that has a choice at index 0
    /// // assert_eq!(control_tower.set_choice(1, 1, ChoiceMark::new("4".to_string(), true)), true);
    /// ```
    pub fn set_choice(&mut self, question_number: usize, choice_number: usize,  choice_answer: ChoiceMark) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.set_choice(question_number, choice_number, (choice_answer.get_text(), choice_answer.is_correct())),
            None => false
        }
    }

    // pub fn push_choice(&mut self, question_number: usize, choice: String, answer: bool) -> bool
    /// Adds a new choice to a specific question in the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it adds the choice and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to add the choice to (1-based).
    /// * `choice` - The text of the new choice.
    /// * `answer` - The correctness flag for the new choice.
    /// 
    /// # Returns
    /// - `true` if the choice was successfully added.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.push_choice(1, "4".to_string(), true), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.push_choice(1, "4".to_string(), true), true);
    /// ```
    pub fn push_choice(&mut self, question_number: usize, choice: String, answer: bool) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.push_choice(question_number, (choice, answer)),
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

    // pub fn set_group(&mut self, question_number: usize, group: u16) -> bool
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
    pub fn set_group(&mut self, question_number: usize, group: u16) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.set_group(question_number, group),
            None => false
        }
    }

    // pub fn get_category(&self, question_number: usize) -> u8
    /// Retrieves the category number for a given question number from the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `0`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to retrieve the category for (1-based).
    /// 
    /// # Returns
    /// - The category number for the specified question if the QBank is loaded
    ///   and the question number is valid.
    /// - `0` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_category(1), 0);
    /// // After loading a QBank with a question at index 0 that belongs to category
    /// // assert_eq!(control_tower.get_category(1), 1);
    /// // for single answer of multiple-choice
    /// ```
    pub fn get_category(&self, question_number: usize) -> u8
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_category(question_number),
            None => 0
        }
    }

    // pub fn set_category(&mut self, question_number: usize, category: u8) -> bool
    /// Sets the category number for a given question number in the QBank.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it updates the category number and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The index of the question to set the category for (1-based).
    /// * `category` - The new category number to set for the specified question.
    /// 
    /// # Returns
    /// - `true` if the category number was successfully updated.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.set_category(1, 1), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.set_category(1, 1), true);
    /// ```
    pub fn set_category(&mut self, question_number: usize, category: u8) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.set_category(question_number, category),
            None => false
        }
    }

    // pub fn remove_question(&mut self, question_number: usize) -> bool
    /// Removes a question from the QBank by its 1-based index.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it removes the question and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to remove.
    /// 
    /// # Returns
    /// - `true` if the question was successfully removed.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.remove_question(1), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.remove_question(1), true);
    /// ```
    pub fn remove_question(&mut self, question_number: usize) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.remove_question(question_number),
            None => false
        }
    }

    // pub fn remove_choice(&mut self, question_number: usize, choice_number: usize) -> bool
    /// Removes a choice from the Quetion by its 1-based index.
    /// 
    /// If the QBank is not loaded or the question number is out of bounds,
    /// it returns `false`. Otherwise, it removes the choice and returns `true`.
    /// 
    /// # Arguments
    /// * `question_number` - The 1-based index of the question to remove.
    /// * `choice_number` - The 1-based index of the choice to remove.
    /// 
    /// # Returns
    /// - `true` if the choice was successfully removed.
    /// - `false` if the QBank is not loaded or the question number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.remove_question(1), false);
    /// // After loading a QBank with a question at index 0
    /// // assert_eq!(control_tower.remove_question(1), true);
    /// ```
    pub fn remove_choice(&mut self, question_number: usize, choice_number: usize) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.remove_choice(question_number, choice_number),
            None => false
        }
    }

    // pub fn optimize_qbank(&mut self)
    /// Optimizes the question bank (QBank) by calling its `optimize` method.
    /// 
    /// If the QBank is not loaded, this method does nothing.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.optimize_qbank();
    /// // After loading a QBank, it will be optimized
    /// control_tower.optimize_qbank();
    /// ```
    pub fn optimize_qbank(&mut self)
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.optimize(); }
    }

    // pub fn get_title(&self) -> String
    /// Retrieves the title of the QBank.
    /// 
    /// If the QBank is not loaded, it returns an empty string.
    /// 
    /// # Returns
    /// - `String`: The title of the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_title(), "");
    /// // After loading a QBank with a title
    /// // assert_eq!(control_tower.get_title(), "My QBank");
    /// ```
    pub fn get_title(&self) -> String
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_header().get_title().clone(),
            None => String::new()
        }
    }

    // pub fn set_title(&mut self, title: String)
    /// Sets the title of the QBank.
    /// 
    /// If the QBank is not loaded, this method does nothing.
    /// 
    /// # Arguments
    /// * `title` - The new title to set for the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.set_title("New Title".to_string());
    /// // After loading a QBank, its title will be updated
    /// control_tower.set_title("Updated Title".to_string());
    /// ```
    pub fn set_title(&mut self, title: String)
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.get_header_mut().set_title(title); }
    }

    // pub fn get_name(&self) -> String
    /// Retrieves the name of the QBank.
    /// 
    /// If the QBank is not loaded, it returns an empty string.
    /// 
    /// # Returns
    /// - `String`: The name of the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_name(), "");
    /// // After loading a QBank with a name
    /// // assert_eq!(control_tower.get_name(), "My QBank");
    /// ```
    pub fn get_name(&self) -> String
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_header().get_name().clone(),
            None => String::new()
        }
    }

    // pub fn set_name(&mut self, name: String)
    /// Sets the name of the QBank.
    /// 
    /// If the QBank is not loaded, this method does nothing.
    /// 
    /// # Arguments
    /// * `name` - The new name to set for the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.set_name("New Name".to_string());
    /// // After loading a QBank, its name will be updated
    /// control_tower.set_name("Updated Name".to_string());
    /// ```
    pub fn set_name(&mut self, name: String)
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.get_header_mut().set_name(name); }
    }

    // pub fn get_id(&self) -> String
    /// Retrieves the ID of the QBank.
    /// 
    /// If the QBank is not loaded, it returns an empty string.
    /// 
    /// # Returns
    /// - `String`: The ID of the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_id(), "");
    /// // After loading a QBank with an ID
    /// // assert_eq!(control_tower.get_id(), "QBank123");
    /// ```
    pub fn get_id(&self) -> String
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_header().get_id().clone(),
            None => String::new()
        }
    }

    // pub fn set_id(&mut self, id: String)
    /// Sets the ID of the QBank.
    /// 
    /// If the QBank is not loaded, this method does nothing.
    /// 
    /// # Arguments
    /// * `id` - The new ID to set for the QBank.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.set_id("New ID".to_string());
    /// // After loading a QBank, its ID will be updated
    /// control_tower.set_id("Updated ID".to_string());
    /// ```
    pub fn set_id(&mut self, id: String)
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.get_header_mut().set_id(id); }
    }

    // pub fn get_notice(&self) -> String
    /// Retrieves the notice string from the QBank's header.
    /// 
    /// If the QBank is not loaded, it returns an empty string.
    /// 
    /// # Returns
    /// - `String`: The notice string from the QBank's header.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_notice(), "");
    /// // After loading a QBank with a notice
    /// // assert_eq!(control_tower.get_notice(), "Important Notice!");
    /// ```
    pub fn get_notice(&self) -> String
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_header().get_notice().clone(),
            None => String::new()
        }
    }

    // pub fn set_notice(&mut self, notice: String)
    /// Sets the notice string in the QBank's header.
    /// 
    /// If the QBank is not loaded, this method does nothing.
    /// 
    /// # Arguments
    /// * `notice` - The new notice string to set in the QBank's header.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.set_notice("New Notice".to_string());
    /// // After loading a QBank, its notice will be updated
    /// control_tower.set_notice("Updated Notice".to_string());
    /// ```
    pub fn set_notice(&mut self, notice: String)
    {
        if let Some(qbank) = &mut self.qbank
            { qbank.get_header_mut().set_notice(notice); }
    }

    // pub fn get_header_categories_length(&self) -> usize
    /// Retrieves the number of categories in the QBank's header.
    /// 
    /// If the QBank is not loaded, it returns `0`.
    /// 
    /// # Returns
    /// - `usize`: The number of categories in the QBank's header.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_header_categories_length(), 0);
    /// // After loading a QBank with categories
    /// // assert_eq!(control_tower.get_header_categories_length(), 2);
    /// ```
    pub fn get_header_categories_length(&self) -> usize
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_header().get_categories().len(),
            None => 0
        }
    }

    // pub fn get_header_category(&self, index: usize) -> String
    /// Retrieves a specific category from the QBank's header by index.
    /// 
    /// If the QBank is not loaded or the index is out of bounds, it returns an empty string.
    /// 
    /// # Arguments
    /// * `index` - The zero-based index of the category to retrieve.
    /// 
    /// # Returns
    /// - `String`: The category string at the specified index.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_header_category(0), "");
    /// // After loading a QBank with categories
    /// // assert_eq!(control_tower.get_header_category(0), "Category 1");
    /// // assert_eq!(control_tower.get_header_category(1), "Category 2");
    /// ```
    pub fn get_header_category(&self, index: usize) -> String
    {
        if let Some(qbank) = &self.qbank
        {
            if let Some(cat) = qbank.get_header().get_category(index as u8)
                { return cat.clone(); }
        }
        String::new()
    }

    // pub fn set_header_category(&mut self, index: usize, category: String)
    /// Sets a specific category in the QBank's header by index.
    /// 
    /// If the QBank is not loaded or the index is out of bounds,
    /// this method does nothing.
    /// 
    /// # Arguments
    /// * `index` - The zero-based index of the category to set.
    /// * `category` - The new category string to set at the specified index.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.set_header_category(0, "New Category".to_string());
    /// // After loading a QBank, its categories will be updated
    /// control_tower.set_header_category(0, "Updated Category".to_string());
    /// ```
    pub fn set_header_category(&mut self, index: usize, category: String)
    {
        if let Some(qbank) = &mut self.qbank
        {
            let header = qbank.get_header_mut();
            let mut cats = header.get_categories().clone();
            if index > 0 && index <= cats.len()
            {
                cats[index - 1] = category;
                header.set_categories(cats);
            }
            else if index == cats.len() + 1
            {
                header.push_category(category);
            }
        }
    }

    // pub fn clear_qbank(&mut self)
    /// Clears the question bank (QBank) by setting it to `None` and resetting
    /// the `question_db` to `AbstractDB::None`.
    /// 
    /// After calling this method, the QBank will be unloaded and any associated
    /// database will be reset.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.clear_qbank();
    /// assert!(control_tower.qbank.is_none());
    /// assert!(matches!(control_tower.question_db, AbstractDB::None));
    /// ```
    pub fn clear_qbank(&mut self)
    {
        self.qbank = None;
        self.question_db = AbstractDB::None;
    }

    // pub fn get_student_length(&self) -> usize
    /// Retrieves the number of students in the student bank (SBank).
    /// 
    /// If the SBank is not loaded, it returns 0.
    /// 
    /// # Returns
    /// - The number of students in the SBank if it is loaded and has students.
    /// - `0` if the SBank has no students or is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_student_length(), 0);
    /// // After loading an SBank with 30 students
    /// // assert_eq!(control_tower.get_student_length(), 30);
    /// ```
    pub fn get_student_length(&self) -> usize
    {
        match &self.sbank
        {
            Some(sbank) => sbank.get_length(),
            None => 0
        }
    }

    // pub fn get_student(&self, student_number: usize) -> NameId
    /// Retrieves the name and ID of a student by their 1-based index from the SBank.
    /// 
    /// If the SBank is not loaded or the student number is out of bounds,
    /// it returns a tuple of two empty strings.
    /// 
    /// # Arguments
    /// * `student_number` - The 1-based index of the student to retrieve.
    /// 
    /// # Returns
    /// - A tuple containing the name and ID of the student if the SBank is
    ///   loaded and the student number is valid.
    /// - A tuple of two empty strings if the SBank is not loaded or the student
    ///   number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.get_student(1), NameId::new(String::new(), String::new()));
    /// // After loading an SBank with a student at index 0 with name "Alice" and ID "s123"
    /// // assert_eq!(control_tower.get_student(1), NameId::new("Alice".to_string(), "s123".to_string()));
    /// ```
    pub fn get_student(&self, student_number: u16) -> NameId
    {
        if let Some(sbank) = &self.sbank
        {
            if let Some(student) = sbank.get_student(student_number)
                { return NameId::new(student.get_name(), student.get_id()); }
        }
        NameId::new_empty()
    }

    // pub fn set_student(&self, student_number: usize, name_id: NameId) -> bool
    /// Sets the name and ID of a student by their 1-based index in the SBank.
    /// 
    /// If the SBank is not loaded or the student number is out of bounds,
    /// it returns `false`. Otherwise, it updates the student's name and ID and returns `true`.
    /// 
    /// # Arguments
    /// * `student_number` - The 1-based index of the student to set.
    /// * `name_id` - A `NameId` containing the new name and ID for the student.
    /// 
    /// # Returns
    /// - `true` if the student's name and ID were successfully updated.
    /// - `false` if the SBank is not loaded or the student number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// assert_eq!(control_tower.set_student(1, NameId::new("Alice".to_string(), "s123".to_string())), false);
    /// // After loading an SBank with a student at index 0
    /// // assert_eq!(control_tower.set_student(1, NameId::new("Alice".to_string(), "s123".to_string())), true);
    /// ```
    pub fn set_student(&mut self, student_number: u16, name_id: NameId) -> bool
    {
        match &mut self.sbank
        {
            Some(sbank) => sbank.set_student(student_number, Student::new(name_id.get_name(), name_id.get_id())),
            None => false
        }
    }

    // pub fn push_student(&mut self, name_id: NameId)
    /// Adds a new student to the end of the SBank using the provided `NameId`.
    /// 
    /// If the SBank is not loaded, this method does nothing.
    /// 
    /// # Arguments
    /// * `name_id` - A `NameId` containing the name and ID of the student to add.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.push_student(NameId::new("Alice".to_string(), "s123".to_string()));
    /// // After loading an SBank, the student "Alice" with ID "s123" should be added to the end of the SBank.
    /// ```
    pub fn push_student(&mut self, name_id: NameId)
    {
        match &mut self.sbank
        {
            Some(sbank) => sbank.push_student(Student::new(name_id.get_name(), name_id.get_id())),
            None => {}
        }
    }

    // pub fn push_an_empty_student(&mut self)
    /// Pushes an empty student (with empty name and ID) to the end of the SBank.
    /// 
    /// If the SBank is not loaded, this method initializes a new SBank and adds
    /// the empty student to it.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.push_an_empty_student();
    /// assert_eq!(control_tower.get_student_length(), 1);
    /// assert_eq!(control_tower.get_student(1), NameId::new_empty());
    /// ```
    pub fn push_an_empty_student(&mut self)
    {
        let student = Student::new_empty();
        match &mut self.sbank
        {
            Some(sbank) => sbank.push_student(student),
            None => {
                let mut sbank = SBank::new();
                sbank.push_student(student);
                self.sbank = Some(sbank);
            }
        }
    }

    // pub fn remove_student(&mut self, student_number: usize) -> bool
    /// Removes a student from the SBank by their 1-based index.
    /// 
    /// If the SBank is not loaded or the student number is out of bounds,
    /// it returns `false`. Otherwise, it removes the student and returns `true`.
    /// 
    /// # Arguments
    /// * `student_number` - The 1-based index of the student to remove.
    /// 
    /// # Returns
    /// - `true` if the student was successfully removed.
    /// - `false` if the SBank is not loaded or the student number is invalid.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// assert_eq!(control_tower.remove_student(1), false);
    /// // After loading an SBank with a student at index 0
    /// // assert_eq!(control_tower.remove_student(1), true);
    /// ```
    pub fn remove_student(&mut self, student_number: usize) -> bool
    {
        match &mut self.sbank
        {
            Some(sbank) => sbank.remove_student(student_number),
            None => false
        }
    }

    // pub fn optimize_sbank(&mut self)
    /// Optimizes the student bank (SBank) by calling its `optimize` method.
    /// 
    /// If the SBank is not loaded, this method does nothing.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.optimize_sbank();
    /// // After loading an SBank, it will be optimized
    /// control_tower.optimize_sbank();
    /// ```
    pub fn optimize_sbank(&mut self)
    {
        if let Some(sbank) = &mut self.sbank
            { sbank.optimize(); }
    }

    // pub fn clear_sbank(&mut self)
    /// Clears the student bank (SBank) by setting it to `None` and resetting
    /// the `student_db` to `AbstractDB::None`.
    /// 
    /// After calling this method, the SBank will be unloaded and any associated
    /// database will be reset.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let mut control_tower = ControlTower::new();
    /// control_tower.clear_sbank();
    /// assert!(control_tower.sbank.is_none());
    /// assert!(matches!(control_tower.student_db, AbstractDB::None));
    /// ```
    pub fn clear_sbank(&mut self)
    {
        self.sbank = None;
        self.student_db = AbstractDB::None;
    }

    // pub fn generate_exam_in_txt(&self, start: u16, end: u16, selected: usize, seeds:  &[u64]) -> Vec<u8>
    /// Generates a shuffled exam in plain text format based on the questions
    /// in the QBank and the students in the SBank.
    /// 
    /// This method creates a `Generator` instance using the loaded QBank and
    /// SBank, and then calls the `export_shuffled_exams_in_txt()` method of
    /// the generator to generate the exam in text format. The generated exam
    /// is returned as a byte vector.
    /// 
    /// If the QBank or SBank is not loaded, it returns an empty byte vector.
    /// 
    /// # Arguments
    /// * `start` - The starting group number for the exam generation.
    /// * `end` - The ending group number for the exam generation.
    /// * `selected` - The number of questions to select for each student.
    /// * `seeds` - A seed array, each element of which is of u64.
    /// 
    /// # Returns
    /// - A byte vector containing the generated exam in plain text format
    ///   if the QBank and SBank are loaded.
    /// - An empty byte vector if the QBank or SBank is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// let exam_data = control_tower.generate_exam_in_txt(1, 5, 10);
    /// if exam_data.is_empty()
    ///     { println!("Failed to generate exam: QBank or SBank not loaded"); }
    /// else
    ///     { println!("Exam generated successfully, size: {}", exam_data.len()); }
    /// ```
    pub fn generate_exam_in_txt(&self, start: u16, end: u16, selected: usize, seeds: &[u64]) -> Vec<u8>
    {
        if let (Some(qbank), Some(sbank)) = (&self.qbank, &self.sbank)
        {
            let mut seed_array = [0u64; 16];
            for i in 0..16
                { seed_array[i] = seeds[i]; }
            if let Some(g) = Generator::new_with_seeds(qbank, start, end, selected, sbank, seed_array)
                { return g.export_shuffled_exams_in_txt(); }
        }
        Vec::new()
    }

    // pub fn generate_exam_in_docx(&self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> Result<Vec<u8>, ErrorMessage>
    /// Generates a shuffled exam in DOCX format based on the questions
    /// in the QBank and the students in the SBank.
    /// 
    /// This method creates a `Generator` instance using the loaded QBank and
    /// SBank, and then calls the `export_shuffled_exams_in_docx()` method of
    /// the generator to generate the exam in DOCX format. The generated exam
    /// is returned as a byte vector.
    /// 
    /// If the QBank or SBank is not loaded, it returns `FailedToGenerateExam`.
    /// 
    /// # Arguments
    /// * `start` - The starting group number for the exam generation.
    /// * `end` - The ending group number for the exam generation.
    /// * `number_of_questions` - The number of questions to select
    ///   for each student.
    /// * `seeds` - A seed array, each element of which is of u64.
    /// 
    /// # Returns
    /// - A `Result` containing a byte vector with the generated exam
    ///   in DOCX format if the QBank and SBank are loaded.
    /// - `ErrorMessage::FailedToGenerateExam` if the QBank or SBank is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// if let Ok(exam_data) = control_tower.generate_exam_in_docx(1, 5, 10)
    ///     { println!("Exam generated successfully, size: {}", exam_data.len()); }
    /// else
    ///     { println!("Failed to generate exam: QBank or SBank not loaded"); }
    /// ```
    pub fn generate_exam_in_docx(&self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> Result<Vec<u8>, ErrorMessage>
    {
        if let (Some(qbank), Some(sbank)) = (&self.qbank, &self.sbank)
        {
            let mut seed_array = [0u64; 16];
            for i in 0..16
                { seed_array[i] = seeds[i]; }
            if let Some(g) = Generator::new_with_seeds(qbank, start, end, number_of_questions as usize, sbank, seed_array)
                { return g.export_shuffled_exams_in_docx().map_err(|_| ErrorMessage::FailedToGenerateExam); }
        }
        Err(ErrorMessage::FailedToGenerateExam)
    }

    // pub fn generate_exam_in_pdf(&self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> Result<Vec<u8>, ErrorMessage>
    /// Generates a shuffled exam in PDF format based on the questions
    /// in the QBank and the students in the SBank.
    /// 
    /// This method creates a `Generator` instance using the loaded QBank and
    /// SBank, and then calls the `export_shuffled_exams_in_pdf()` method of
    /// the generator to generate the exam in PDF format. The generated exam
    /// is returned as a byte vector.
    /// 
    /// If the QBank or SBank is not loaded, it returns `FailedToGenerateExam`.
    /// 
    /// # Arguments
    /// * `start` - The starting group number for the exam generation.
    /// * `end` - The ending group number for the exam generation.
    /// * `number_of_questions` - The number of questions to select
    ///   for each student.
    /// * `seeds` - A seed array, each element of which is of u64.
    /// 
    /// # Returns
    /// - A `Result` containing a byte vector with the generated exam
    ///   in PDF format if the QBank and SBank are loaded.
    /// - `ErrorMessage::FailedToGenerateExam` if the QBank or SBank is not loaded.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// if let Ok(exam_data) = control_tower.generate_exam_in_pdf(1, 5, 10)
    ///     { println!("Exam generated successfully, size: {}", exam_data.len()); }
    /// else
    ///     { println!("Failed to generate exam: QBank or SBank not loaded"); }
    /// ```
    #[wasm_bindgen]
    pub fn generate_exam_in_pdf(&self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> Result<Vec<u8>, JsValue>
    {
        if let (Some(qbank), Some(sbank)) = (&self.qbank, &self.sbank)
        {
            let mut seed_array = [0u64; 16];
            for i in 0..16
                { seed_array[i] = seeds[i]; }
            if let Some(g) = Generator::new_with_seeds(qbank, start, end, number_of_questions as usize, sbank, seed_array)
            {
                return g.export_shuffled_exams_in_pdf()
                    .map_err(|e| JsValue::from_str(&e));
            }
        }
        Err(JsValue::from_str("FailedToGenerateExam"))
    }

    // pub fn start_self_study(&mut self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> bool
    /// Starts a self-study session with the specified parameters.
    /// 
    /// This method creates a `SelfStudy` instance using the loaded QBank and
    /// starts the self-study session with the specified parameters.
    /// 
    /// If the QBank is not loaded, it returns `false`.
    /// 
    /// # Arguments
    /// * `start` - The starting question number for the self-study session.
    /// * `end` - The ending question number for the self-study session.
    /// * `number_of_questions` - The number of questions to select
    ///   for user student.
    /// * `seeds` - A seed array, each element of which is of u64.
    /// 
    /// # Returns
    /// - `true` if the self-study session is started successfully.
    pub fn start_self_study(&mut self, start: u16, end: u16, number_of_questions: u16, seeds: &[u64]) -> bool
    {
        if let Some(qbank) = &self.qbank
        {
            let mut seed_array = [0u64; 16];
            for i in 0..16
                { seed_array[i] = seeds[i]; }
            self.self_study = SelfStudy::new(qbank, start, end, number_of_questions as usize, seed_array);
            return self.self_study.is_some();
        }
        false
    }

    // pub fn get_self_study_number_of_questions(&self) -> u16
    /// Returns the number of questions in the self-study session.
    /// 
    /// This method returns the number of questions in the self-study session
    /// from the `SelfStudy` instance. If the `SelfStudy` instance is not
    /// initialized, it returns `0`.
    /// 
    /// # Returns
    /// - The number of questions in the self-study session.
    /// - `0` if the `SelfStudy` instance is not initialized.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// let num_questions = control_tower.get_self_study_number_of_questions();
    /// println!("Number of questions in self-study session: {}", num_questions);
    /// ```
    pub fn get_self_study_number_of_questions(&self) -> u16
    {
        if let Some(ss) = &self.self_study
            { ss.get_number_of_questions() as u16 }
        else
            { 0 }
    }

    // pub fn get_self_study_question(&mut self, num: u16) -> Option<QuestionData>
    /// Returns the question data for the specified question number.
    /// 
    /// This method returns the question data for the specified question number
    /// from the `SelfStudy` instance. If the `SelfStudy` instance is not
    /// initialized, it returns `None`.
    /// 
    /// # Arguments
    /// * `num` - The question number to retrieve.
    /// 
    /// # Returns
    /// - `Some(QuestionData)` if the question data is retrieved successfully.
    /// - `None` if the `SelfStudy` instance is not initialized.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// if let Some(qdata) = control_tower.get_self_study_question(1)
    ///     { println!("Question data retrieved successfully"); }
    /// else
    ///     { println!("Failed to retrieve question data"); }
    /// ```
    pub fn get_self_study_question(&mut self, num: u16) -> Option<QuestionData>
    {
        if let Some(ss) = &mut self.self_study
        {
            if let Some((num, cat_id, cat_str, question, choices)) = ss.get_question_by_number(num)
            {
                let mut qdata = QuestionData::new(num, cat_id, cat_str, question);
                for (c_text, c_correct) in choices
                    { qdata.push_choice(c_text, c_correct); }
                return Some(qdata);
            }
        }
        None
    }

    // pub fn get_next_self_study_question(&mut self) -> Option<QuestionData>
    /// Returns the next question data in the self-study session.
    /// 
    /// This method returns the next question data in the self-study session
    /// from the `SelfStudy` instance. If the `SelfStudy` instance is not
    /// initialized, it returns `None`.
    /// 
    /// # Returns
    /// - `Some(QuestionData)` if the next question data is retrieved successfully.
    /// - `None` if the `SelfStudy` instance is not initialized.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// if let Some(qdata) = control_tower.get_next_self_study_question()
    ///     { println!("Next question data retrieved successfully"); }
    /// else
    ///     { println!("Failed to retrieve next question data"); }
    /// ```
    pub fn get_next_self_study_question(&mut self) -> Option<QuestionData>
    {
        if let Some(ss) = &mut self.self_study
        {
            if let Some((num, cat_id, cat_str, question, choices)) = ss.next()
            {
                let mut qdata = QuestionData::new(num, cat_id, cat_str, question);
                for (c_text, c_correct) in choices
                    { qdata.push_choice(c_text, c_correct); }
                return Some(qdata);
            }
        }
        None
    }

    // pub fn get_prev_self_study_question(&mut self) -> Option<QuestionData>
    /// Returns the previous question data in the self-study session.
    /// 
    /// This method returns the previous question data in the self-study session
    /// from the `SelfStudy` instance. If the `SelfStudy` instance is not
    /// initialized, it returns `None`.
    /// 
    /// # Returns
    /// - `Some(QuestionData)` if the previous question data is retrieved successfully.
    /// - `None` if the `SelfStudy` instance is not initialized.
    /// 
    /// # Examples
    /// ```
    /// use qrate_wasm::ControlTower;
    /// let control_tower = ControlTower::new();
    /// if let Some(qdata) = control_tower.get_prev_self_study_question()
    ///     { println!("Previous question data retrieved successfully"); }
    /// else
    ///     { println!("Failed to retrieve previous question data"); }
    /// ```
    pub fn get_prev_self_study_question(&mut self) -> Option<QuestionData>
    {
        if let Some(ss) = &mut self.self_study
        {
            if let Some((num, cat_id, cat_str, question, choices)) = ss.prev()
            {
                let mut qdata = QuestionData::new(num, cat_id, cat_str, question);
                for (c_text, c_correct) in choices
                    { qdata.push_choice(c_text, c_correct); }
                return Some(qdata);
            }
        }
        None
    }

    pub fn set_self_study_scoring_rule(&mut self, rule: String)
    {
        if let Some(ss) = &mut self.self_study
            { ss.set_scoring_rule(ScoringRule::from_str(&rule)); }
    }

    pub fn set_self_study_choices_answer(&mut self, num: u16, answers: Vec<u8>)
    {
        if let Some(ss) = &mut self.self_study
        {
            let bool_answers = answers.into_iter().map(|v| v != 0).collect();
            ss.set_answer(num, UserAnswer::Choices(bool_answers));
        }
    }

    pub fn set_self_study_short_answer(&mut self, num: u16, answer: String)
    {
        if let Some(ss) = &mut self.self_study
            { ss.set_answer(num, UserAnswer::ShortAnswer(answer)); }
    }

    pub fn get_self_study_score(&self) -> f64
    {
        if let Some(ss) = &self.self_study
            { ss.score() }
        else
            { 0.0 }
    }
}