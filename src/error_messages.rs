// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;

/// An enumeration of error messages that can occur in the Qrate application.
/// This enum is designed to be used in a WebAssembly context, allowing it to
/// be easily manipulated from JavaScript. Each variant represents a specific
/// error that may occur during the execution of the application, such as
/// issues with opening files, receiving data from memory,
#[wasm_bindgen]
pub enum ErrorMessage
{
    /// Represents an error where the version of the application is invalid.
    InvalidVersion,

    /// Represents an error where the application failed to open a question bank file.
    FailedToOpenQBank,

    /// Represents an error where the application failed to open a student bank file.
    FailedToOpenSBank,

    /// Represents an error where the application failed to open a question Excel file.
    FailedToOpenQExcel,

    /// Represents an error where the application failed to open a student Excel file.
    FailedToOpenSExcel,

    /// Represents an error where the application failed to receive a question bank from memory.
    FailedToReceiveQBankFromMemory,

    /// Represents an error where the application failed to receive a student bank from memory.
    FailedToReceiveSBankFromMemory,

    /// Represents an error where the application failed to write a question bank to memory.
    FailedToWriteQBankToMemory,

    /// Represents an error where the application failed to write a student bank to memory.
    FailedToWriteSBankToMemory,

    /// Represents an error where the application failed to generate an exam.
    FailedToGenerateExam,
}
