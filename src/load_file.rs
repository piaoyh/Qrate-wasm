// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use qrate::{ SQLiteDB, QBank, SBank, ErrorMessage, SBDB };

pub struct LoadFile;

impl LoadFile
{
    pub async fn load_qbank_from_bytes(data: &[u8]) -> Result<QBank, ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            if let Some(qb) = db.read_qbank()
                { Ok(qb) }
            else
                { Err(ErrorMessage::FailedToOpenQBank) }
        }
        else
        {
            Err(ErrorMessage::FailedToOpenQBank)
        }
    }

    pub async fn load_sbank_from_bytes(data: &[u8], extension: &str) -> Result<SBank, ErrorMessage>
    {
        if let Some(db) = SQLiteDB::open_in_memory(data)
        {
            if let Some(qb) = db.read_sbank()
                { Ok(qb) }
            else
                { Err(ErrorMessage::FailedToOpenSBank) }
        }
        else
        {
            Err(ErrorMessage::FailedToOpenSBank)
        }
    }
}
