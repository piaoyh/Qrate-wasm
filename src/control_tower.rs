// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use qrate::qbank::QBank;
use qrate::sbank::SBank;
use qrate::generator::Generator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ControlTower
{
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
            qbank: None,
            sbank: None,
            generator: None,
        }
    }

    pub fn set_qbank_from_bytes(&mut self, _data: &[u8], _extension: &str) -> Result<(), String>
    {
        // TODO: Implement byte-based QBank loading
        Ok(())
    }

    pub fn set_sbank_from_bytes(&mut self, _data: &[u8], _extension: &str) -> Result<(), String>
    {
        // TODO: Implement byte-based SBank loading
        Ok(())
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
}
