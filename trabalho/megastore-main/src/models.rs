use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Edge {
    pub to_id: i32,
    pub relationship: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ProductData {
    pub price: f64,
    pub category: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub data: ProductData,
    pub edges: Vec<Edge>,
}
