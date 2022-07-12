use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct FormProduct {
    pub id: Option<i64>,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormVariant {
    pub id: Option<i64>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormVariantValue {
    pub variant: FormVariant,
    pub values: Vec<Option<String>>
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct FormCompleteProduct {
    pub product: FormProduct,
    pub variants: Vec<FormVariantValue>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariant {
    pub id: i32,
    pub variant_id: i32,
    pub product_id: i32,
    pub value: Option<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    pub id: i32,
    pub name: String,
}

pub type ProductList = Vec<(Product, Vec<(ProductVariant, Variant)>)>;