use core::hash::Hash;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct Order {
    pub items: Vec<LineItem>,
    pub idempotency_key: String
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct LineItem {
    pub catalog_object_id: String,
    pub active_modifier_ids: Vec<String>,
    pub text_modifier_values: Vec<(String, String)>,
    pub quantity: usize
}