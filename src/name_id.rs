// Copyright 2026. PARK Youngho. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NameId
{
    name: String,
    id: String,
}


#[wasm_bindgen]
impl NameId
{
    // pub fn new(name: String, id: String) -> Self
    /// Creates a new `NameId` instance.
    ///
    /// # Arguments
    /// * `name` - The name associated with the ID.
    /// * `id` - The ID associated with the name.
    ///
    /// # Output
    /// `Self` - A new instance of `NameId`.
    ///
    /// # Examples
    /// ```
    /// use qrate_wasm::NameId;
    /// let name_id = NameId::new("Alice".to_string(), "s123".to_string());
    /// assert_eq!(name_id.get_name(), "Alice");
    /// assert_eq!(name_id.get_id(), "s123");
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, id: String) -> Self
    {
        Self { name, id }
    }
    
    // pub fn new_empty() -> Self
    /// Creates a new empty `NameId` instance.
    ///
    /// # Arguments
    /// * `name` - The name associated with the ID.
    /// * `id` - The ID associated with the name.
    ///
    /// # Output
    /// `Self` - A new instance of `NameId`.
    ///
    /// # Examples
    /// ```
    /// use qrate_wasm::NameId;
    /// let name_id = NameId::new_empty();
    /// assert_eq!(name_id.get_name(), "");
    /// assert_eq!(name_id.get_id(), "");
    /// ```
    pub fn new_empty() -> Self
    {
        Self { name: String::new(), id: String::new() }
    }

    // pub fn get_name(&self) -> String
    /// Retrieves the name from the `NameId` instance.
    ///
    /// # Returns
    /// A `String` containing the name.
    ///
    /// # Examples
    /// ```
    /// use qrate_wasm::NameId;
    /// let name_id = NameId::new("Alice".to_string(), "s123".to_string());
    /// assert_eq!(name_id.get_name(), "Alice");
    /// ```
    pub fn get_name(&self) -> String
    {
        self.name.clone()   // Return a clone to avoid ownership issue
    }

    // pub fn get_id(&self) -> String
    /// Retrieves the ID from the `NameId` instance.
    ///
    /// # Returns
    /// A `String` containing the ID.
    ///
    /// # Examples
    /// ```
    /// use qrate_wasm::NameId;
    /// let name_id = NameId::new("Alice".to_string(), "s123".to_string());
    /// assert_eq!(name_id.get_id(), "s123");
    /// ```
    pub fn get_id(&self) -> String
    {
        self.id.clone()     // Return a clone to avoid ownership issue
    }
}