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
use crate::{ AbstractDB, ErrorMessage };



#[wasm_bindgen]
pub struct ControlTower
{
    db: AbstractDB,
    qbank: Option<QBank>,
    sbank: Option<SBank>,
    generator: Option<Generator>,
}

#[wasm_bindgen]
impl ControlTower
{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self
    {
        ControlTower
        {
            db: AbstractDB::None,
            qbank: None,
            sbank: None,
            generator: None,
        }
    }

    pub fn set_qbank_from_bytes(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.qbank = db.read_qbank();
            if self.qbank.is_some()
            {
                self.db = AbstractDB::SQLite(db);
                return Ok(());
            }
        }
        Err(ErrorMessage::FailedToRecevieQBankFromMemory)
    }

    pub fn set_sbank_from_bytes(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            self.sbank = db.read_sbank();
            if self.sbank.is_some()
            {
                self.db = AbstractDB::SQLite(db);
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

    pub fn get_question_length(&self) -> usize
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_length(),
            None => 0
        }
    }

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

    pub fn get_choices_length(&self, question_number: usize) -> usize
    {
        match &self.qbank
        {
            Some(qbank) => qbank.get_choices_length(question_number),
            None => 0
        }
    }
}
