// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum ErrorMessage
{
    FailedToOpenQBank,
    FailedToOpenSBank,
    FailedToOpenQExcel,
    FailedToOpenSExcel,
    FailedToRecevieQBankFromMemory,
    FailedToRecevieSBankFromMemory,
}
