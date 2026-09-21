// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use qrate::ErrorMessage;
use wasm_bindgen::prelude::*;

/// An enumeration of error messages that can occur in the Qrate application.
/// This enum is designed to be used in a WebAssembly context, allowing it to
/// be easily manipulated from TypeScript/JavaScript. Each variant represents a specific
/// error that may occur during the execution of the application, such as
/// issues with opening files, receiving data from memory,
#[wasm_bindgen]
pub enum ErrorMessageForWASM
{
    /// Represents an error where the version of the data is invalid or incompatible.
    InvalidVersion,

    /// Represents an error where the question bank is empty.
    EmptyQBank,

    /// Represents an error where the student bank is empty.
    EmptySBank,

    /// Represents an error where the database cannot be opened in memory.
    FailedToOpenEmptyDatabaseInMemory,

    /// Represents an error where the database cannot be opened.
    FailedToOpenDatabase,

    /// Represents an error where the database cannot be written.
    FailedToWriteDatabase,

    /// Represents an error where the table for the database cannot be written.
    FailedToMakeTableForDatabase,

    /// Represents an error where the database cannot be opened in memory.
    FailedToOpenDatabaseInMemory,

    /// Represents an error where the database cannot be received from memory.
    FailedToReceiveDatabaseFromMemory,

    /// Represents an error where the database cannot be written to memory.
    FailedToWriteDatabaseToMemory,

    /// Represents an error where the database cannot be closed.
    FailedToCloseDatabase,

    /// Represents an error where the database cannot be vacuumed.
    FailedToVacuumDatabase,

    /// Represents an error where the database cannot be opened.
    FailedToOpenEmptyQBankInMemory,

    /// Represents an error where the data format `QBank` is invalid or cannot be parsed.
    FailedToOpenQBank,

    /// Represents an error where the header for `QBank` cannot be read.
    FailedToReadHeaderForQBank,

    /// Represents an error where the `QBank` cannot be written to the database.
    FailedToWriteQBank,

    /// Represents an error where the header for `QBank` cannot be written.
    FailedToWriteHeaderForQBank,

    /// Represents an error where the table for `QBank` cannot be written to the database.
    FailedToMakeTableForQBank,

    /// Represents an error where the header for `QBank` cannot be created.
    FailedToCreateHeaderForQBank,

    /// Represents an error where the data format `SBank` is invalid or cannot be parsed.
    FailedToOpenSBank,

    /// Represents an error where the header for `SBank` cannot be read.
    FailedToReadHeaderForSBank,

    /// Represents an error where the `SBank` cannot be written to the database.
    FailedToWriteSBank,

    /// Represents an error where the table for `SBank` cannot be written to the database.
    FailedToMakeTableForSBank,

    /// Represents an error where the header for `SBank` cannot be created.
    FailedToCreateHeaderForSBank,

    /// Represents an error where the Excel file for `QBank` cannot be opened or read.
    FailedToOpenQExcel,

    /// Represents an error where the `QBank` cannot be written to the Excel file.
    FailedToWriteQExcel,

    /// Represents an error where the Excel file for `SBank` cannot be opened or read.
    FailedToOpenSExcel,

    /// Represents an error where the `SBank` cannot be written to the Excel file.
    FailedToWriteSExcel,

    /// Represents an error where the `QBank` cannot be received from memory.
    FailedToReceiveQBankFromMemory,

    /// Represents an error where the `QBank` cannot be written to memory.
    FailedToWriteQBankToMemory,

    /// Represents an error where the `SBank` cannot be received from memory.
    FailedToReceiveSBankFromMemory,

    /// Represents an error where the `SBank` cannot be written to memory.
    FailedToWriteSBankToMemory,
    
    /// Represents an error where the exam cannot be generated.
    FailedToGenerateExam,
}


impl ErrorMessageForWASM
{
    pub fn into_wasm(error_message: ErrorMessage) -> ErrorMessageForWASM
    {
        match error_message
        {
            ErrorMessage::InvalidVersion => ErrorMessageForWASM::InvalidVersion,
            ErrorMessage::EmptyQBank => ErrorMessageForWASM::EmptyQBank,
            ErrorMessage::EmptySBank => ErrorMessageForWASM::EmptySBank,
            ErrorMessage::FailedToOpenEmptyDatabaseInMemory => ErrorMessageForWASM::FailedToOpenEmptyDatabaseInMemory,
            ErrorMessage::FailedToOpenDatabase => ErrorMessageForWASM::FailedToOpenDatabase,
            ErrorMessage::FailedToWriteDatabase => ErrorMessageForWASM::FailedToWriteDatabase,
            ErrorMessage::FailedToMakeTableForDatabase => ErrorMessageForWASM::FailedToMakeTableForDatabase,
            ErrorMessage::FailedToOpenDatabaseInMemory => ErrorMessageForWASM::FailedToOpenDatabaseInMemory,
            ErrorMessage::FailedToReceiveDatabaseFromMemory => ErrorMessageForWASM::FailedToReceiveDatabaseFromMemory,
            ErrorMessage::FailedToWriteDatabaseToMemory => ErrorMessageForWASM::FailedToWriteDatabaseToMemory,
            ErrorMessage::FailedToCloseDatabase => ErrorMessageForWASM::FailedToCloseDatabase,
            ErrorMessage::FailedToVacuumDatabase => ErrorMessageForWASM::FailedToVacuumDatabase,
            ErrorMessage::FailedToOpenEmptyQBankInMemory => ErrorMessageForWASM::FailedToOpenEmptyQBankInMemory,
            ErrorMessage::FailedToOpenQBank => ErrorMessageForWASM::FailedToOpenQBank,
            ErrorMessage::FailedToReadHeaderForQBank => ErrorMessageForWASM::FailedToReadHeaderForQBank,
            ErrorMessage::FailedToWriteQBank => ErrorMessageForWASM::FailedToWriteQBank,
            ErrorMessage::FailedToWriteHeaderForQBank => ErrorMessageForWASM::FailedToWriteHeaderForQBank,
            ErrorMessage::FailedToMakeTableForQBank => ErrorMessageForWASM::FailedToMakeTableForQBank,
            ErrorMessage::FailedToCreateHeaderForQBank => ErrorMessageForWASM::FailedToCreateHeaderForQBank,
            ErrorMessage::FailedToOpenSBank => ErrorMessageForWASM::FailedToOpenSBank,
            ErrorMessage::FailedToReadHeaderForSBank => ErrorMessageForWASM::FailedToReadHeaderForSBank,
            ErrorMessage::FailedToWriteSBank => ErrorMessageForWASM::FailedToWriteSBank,
            ErrorMessage::FailedToMakeTableForSBank => ErrorMessageForWASM::FailedToMakeTableForSBank,
            ErrorMessage::FailedToCreateHeaderForSBank => ErrorMessageForWASM::FailedToCreateHeaderForSBank,
            ErrorMessage::FailedToOpenQExcel => ErrorMessageForWASM::FailedToOpenQExcel,
            ErrorMessage::FailedToWriteQExcel => ErrorMessageForWASM::FailedToWriteQExcel,
            ErrorMessage::FailedToOpenSExcel => ErrorMessageForWASM::FailedToOpenSExcel,
            ErrorMessage::FailedToWriteSExcel => ErrorMessageForWASM::FailedToWriteSExcel,
            ErrorMessage::FailedToReceiveQBankFromMemory => ErrorMessageForWASM::FailedToReceiveQBankFromMemory,
            ErrorMessage::FailedToWriteQBankToMemory => ErrorMessageForWASM::FailedToWriteQBankToMemory,
            ErrorMessage::FailedToReceiveSBankFromMemory => ErrorMessageForWASM::FailedToReceiveSBankFromMemory,
            ErrorMessage::FailedToWriteSBankToMemory => ErrorMessageForWASM::FailedToWriteSBankToMemory,
            ErrorMessage::FailedToGenerateExam => ErrorMessageForWASM::FailedToGenerateExam,
        }
    }
}