use crate::models::Product;
use std::collections::{HashMap, HashSet};

// Função para normalizar texto (remove acentos e deixa minúsculo)
fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            'á' | 'à' | 'ã' | 'â' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'õ' | 'ô' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}

// Estrutura de produto com score
struct ScoredProduct<'a> {
    product: &'a Product,
    score: i32,
}

// Função principal de busca em grafo
pub fn search_products<'a>(
    query: &str,
    products: &'a HashMap<i32, Product>,
) -> Vec<&'a Product> {
    let query_normalized = normalize_text(query);
    let mut results: Vec<ScoredProduct> = Vec::new();
    let mut visited: HashSet<i32> = HashSet::new();

    // Busca inicial: nome, categoria, descrição
    for product in products.values() {
        let name = normalize_text(&product.name);
        let category = normalize_text(&product.data.category);
        let description = normalize_text(&product.description);

        let mut score = 0;
        if name.contains(&query_normalized) {
            score += 3;
        }
        if category.contains(&query_normalized) {
            score += 2;
        }
        if description.contains(&query_normalized) {
            score += 1;
        }

        if score > 0 {
            results.push(ScoredProduct { product, score });
            visited.insert(product.id);
        }
    }

    // Expansão em grafo: adiciona produtos relacionados
    let mut i = 0;
    while i < results.len() {
        let current = &results[i].product;

        for edge in &current.edges {
            if !visited.contains(&edge.to_id) {
                if let Some(related_product) = products.get(&edge.to_id) {
                    // Cada relacionamento aumenta score dependendo do tipo
                    let mut score = 1;
                    if edge.relationship == "COMPRADO_JUNTO" {
                        score += 2;
                    } else if edge.relationship == "COMPATIVEL_COM" {
                        score += 1;
                    }

                    results.push(ScoredProduct {
                        product: related_product,
                        score,
                    });
                    visited.insert(related_product.id);
                }
            }
        }

        i += 1;
    }

    // Ordena pelos produtos com maior score
    results.sort_by(|a, b| b.score.cmp(&a.score));

    // Retorna apenas os produtos
    results.into_iter().map(|s| s.product).collect()
}