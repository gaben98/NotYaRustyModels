use core::hash::Hash;
use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct Catalog {
    pub items: Vec<CatalogItem>,
    pub categories: Vec<CatalogCategory>,
    pub images: Vec<CatalogImage>,
    pub options: Vec<CatalogItemOptionList>,
    pub modifiers: Vec<CatalogItemModifierList>
}

impl Default for Catalog {
    fn default() -> Self {
        Catalog {
            items: vec![],
            categories: vec![],
            images: vec![],
            options: vec![],
            modifiers: vec![]
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItem {
    pub item_name: String,
    pub primary_category_id: Option<String>,
    pub category_ids: Vec<String>,
    pub price: i32,
    pub image_ids: Vec<String>,
    pub html_description: String,
    pub item_option_list_ids: Vec<String>,
    pub item_modifier_ids: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Hash, Clone)]
pub struct CatalogCategory {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogImage {
    pub image_id: String,
    pub image_url: String
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItemOptionList {
    pub option_list_id: String,
    pub option_name: String,
    pub option_value_ids: Vec<String>
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItemOptionValue {
    pub option_value_id: String,
    pub option_value_name: String
}

pub enum CatalogItemModifier {
    List(CatalogItemModifierList),
    Text(CatalogItemTextModifier)
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItemTextModifier {
    pub text_modifier_id: String,
    pub max_length: Option<i32>,
    pub required: bool
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItemModifierList {
    pub modifier_list_id: String,
    pub modifier_value_ids: Vec<String>,
    pub min_selected_modifiers: i64,
    pub max_selected_modifiers: i64
}

#[derive(Serialize, Deserialize, PartialEq, Hash)]
pub struct CatalogItemModifierValue {
    pub modifier_value_id: String,
    pub modifier_value_name: String,
    pub price: i32
}