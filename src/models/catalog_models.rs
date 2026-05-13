use std::vec::Vec;

use serde::Serialize;

#[derive(Serialize)]
pub struct Catalog {
    pub items: Vec<CatalogItem>,
    pub categories: Vec<CatalogCategory>,
    pub images: Vec<CatalogImage>,
    pub options: Vec<CatalogItemOptions>,
    pub modifiers: Vec<CatalogItemModifiers>
}

#[derive(Serialize)]
pub struct CatalogItem {
    pub item_name: String,
    pub category_ids: Vec<String>,
    pub image_ids: Vec<String>,
    pub html_description: String,
    pub item_option_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct CatalogCategory {
    pub category_id: String,
    pub category_name: String,
}

#[derive(Serialize)]
pub struct CatalogImage {
    pub image_id: String,
    pub image_url: String,
}

#[derive(Serialize)]
pub struct CatalogItemOptions {
    pub option_id: String,
    
}

#[derive(Serialize)]
pub struct CatalogItemModifiers {
    pub modifier_id: String,
}