// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.

use qrate::qbank::QBank;
use qrate::sbank::SBank;

pub struct LoadFile;

impl LoadFile
{
    pub async fn load_qbank_from_bytes(data: &[u8], extension: &str) -> Result<QBank, String>
    {
        // TODO: Implement byte-based QBank loading
        Err("Not implemented".into())
    }

    pub async fn load_sbank_from_bytes(data: &[u8], extension: &str) -> Result<SBank, String>
    {
        // TODO: Implement byte-based SBank loading
        Err("Not implemented".into())
    }
}
