use std::collections::HashMap;

use graph_search::models::{Product, ProductData};
use graph_search::search::search_products;

fn create_test_product(
    id: i32,
    name: &str,
    category: &str,
) -> Product {
    Product {
        id,
        name: name.to_string(),
        description: String::from("produto teste"),
        data: ProductData {
            price: 100.0,
            category: category.to_string(),
        },
        edges: Vec::new(),
    }
}

#[test]
fn test_search_by_name() {
    let mut products = HashMap::new();

    let p1 = create_test_product(1, "Camera Profissional", "cameras");
    let p2 = create_test_product(2, "Lente 50mm", "lentes");

    products.insert(p1.id, p1);
    products.insert(p2.id, p2);

    let results = search_products("camera", &products);

    assert!(!results.is_empty());
    assert_eq!(results[0].name, "Camera Profissional");
}

#[test]
fn test_search_by_category() {
    let mut products = HashMap::new();

    let p1 = create_test_product(1, "Tripé", "acessorios");
    let p2 = create_test_product(2, "Microfone", "audio");

    products.insert(p1.id, p1);
    products.insert(p2.id, p2);

    let results = search_products("audio", &products);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Microfone");
}

#[test]
fn test_search_no_results() {
    let mut products = HashMap::new();

    let p1 = create_test_product(1, "Tripé", "acessorios");
    products.insert(p1.id, p1);

    let results = search_products("notebook", &products);

    assert!(results.is_empty());
}