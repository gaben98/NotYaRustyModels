use std::vec::Vec;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Catalog {
    pub items: Vec<CatalogItem>,
    pub categories: Vec<CatalogCategory>,
    pub images: Vec<CatalogImage>,
    pub options: Vec<CatalogItemOptions>,
    pub modifiers: Vec<CatalogItemModifiers>
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

#[derive(Serialize, Deserialize)]
pub struct CatalogItem {
    pub item_name: String,
    pub category_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub html_description: String,
    pub item_option_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogCategory {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogImage {
    pub image_id: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct CatalogItemOptions {
    pub option_id: String,
    
}

#[derive(Serialize, Deserialize)]
pub struct CatalogItemModifiers {
    pub modifier_id: String,
}