use crate::models::Product;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn load_products(dir_path: &str) -> Result<HashMap<i32, Product>, Box<dyn std::error::Error>> {
    let mut products = HashMap::new();
    let path = Path::new(dir_path);

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();
        if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_content = fs::read_to_string(&file_path)?;
            let product: Product = serde_json::from_str(&file_content)?;
            products.insert(product.id, product);
        }
    }

    Ok(products)
}
