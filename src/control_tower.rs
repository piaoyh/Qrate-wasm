// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.


use wasm_bindgen::prelude::*;
use qrate::{ QBank, SBank };
use qrate::generator::Generator;
use crate::{ LoadFile, ErrorMessage };

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

    #[inline]
    pub fn set_qbank_from_bytes(&mut self, data: &[u8]) -> Result<(), ErrorMessage>
    {
        let qbank = LoadFile::load_qbank_from_bytes(data)?;
        self.qbank = Some(qbank);
        Ok(())
    }

    pub fn set_sbank_from_bytes(&mut self, _data: &[u8]) -> Result<(), ErrorMessage>
    {
        let qbank = LoadFile::load_sbank_from_bytes(data)?;
        self.sbank = Some(sbank);
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
