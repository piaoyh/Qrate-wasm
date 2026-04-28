// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;
use qrate::{ QBDB, QBank, SBDB, SBank, SBankHelper, SQLiteDB,
            Student, Question, ChoiceAnswer, Generator };
use crate::{ AbstractDB, ChoiceAnswer, NameId, ErrorMessage };



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
    pub fn write_sbank_to_bytes_in_sqlite(&self) -> Result<Vec<u8>, ErrorMessage>
    {
        if let Some(sbank) = &self.sbank
        {
            if let Some(mut db) = SQLiteDB::open_empty_in_memory()
            {
                if db.write_sbank(sbank).is_ok()
                {
                    return db.save_in_memory().map_err(|_| ErrorMessage::FailedToWriteSBankToMemory);
                }
            }
        }
        Err(ErrorMessage::FailedToWriteSBankToMemory)
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
        let question = Question::new_empty();
        match &mut self.qbank
        {
            Some(qbank) => qbank.push_question(question),
            None => ()
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
        {
            qbank.determine_category(question_number)
        }
        else
        {
            false
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
    pub fn set_question(&mut self, question_number: usize, txt: String) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => {
                match qbank.get_question_mut(question_number)
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

    // pub fn set_choice(&mut self, question_number: usize, choice_number: usize,  choice_answer: ChoiceAnswe) -> bool
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
    pub fn set_choice(&mut self, question_number: usize, choice_number: usize,  choice_answer: ChoiceAnswer) -> bool
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

    pub fn remove_question(&mut self, question_number: usize) -> bool
    {
        match &mut self.qbank
        {
            Some(qbank) => qbank.remove_question(question_number),
            None => false
        }
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
    pub fn get_student(&self, student_number: usize) -> NameId
    {
        match &self.sbank
        {
            Some(sbank) => {
                if let Some(student) = sbank.get_student(student_number)
                {
                    NameId::new(student.get_name(), student.get_id())
                }
                else
                {
                    NameId::new_empty()
                }
            },
            None => NameId::new_empty()
        }
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
    pub fn set_student(&mut self, student_number: usize, name_id: NameId) -> bool
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
}