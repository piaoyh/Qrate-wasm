// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use qrate::{ SQLiteDB, Excel };
pub enum AbstractDB
{
    SQLite(SQLiteDB),
    Excel(Excel),
    None,
}

impl AbstractDB
{
    // pub fn is_sqlite(&self) -> bool
    /// Checks if the current database is an SQLite database.
    /// 
    /// # Returns
    /// `true` if the database is SQLite, `false` otherwise.
    pub fn is_sqlite(&self) -> bool
    {
        matches!(self, AbstractDB::SQLite(_))
    }

    // pub fn is_excel(&self) -> bool
    /// Checks if the current database is an Excel database.
    /// 
    /// # Returns
    /// `true` if the database is Excel, `false` otherwise.
    pub fn is_excel(&self) -> bool
    {
        matches!(self, AbstractDB::Excel(_))
    }

    // pub fn is_none(&self) -> bool
    /// Checks if the current database is None.
    /// 
    /// # Returns
    /// `true` if the database is None, `false` otherwise.
    pub fn is_none(&self) -> bool
    {
        matches!(self, AbstractDB::None)
    }
}