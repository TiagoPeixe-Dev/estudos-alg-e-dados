mod file_loader;
mod models;
mod search;

use models::Product;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

// mesma função do search.rs
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

fn print_help() {
    println!("Comandos disponíveis:");
    println!("  search <termo>  Busca por nome ou descrição");
    println!("  list            Lista todos os produtos");
    println!("  help            Exibe esta ajuda");
    println!("  exit            Encerra o programa");
}

fn main() {
    println!("Carregando produtos...");
    let products_map: HashMap<i32, Product> = match file_loader::load_products("./data") {
        Ok(map) => {
            println!("{} produtos carregados. Motor de busca pronto.", map.len());
            map
        }
        Err(e) => {
            eprintln!("Erro ao carregar produtos: {}", e);
            return;
        }
    };

    loop {
        print!("> ");
        if io::stdout().flush().is_err() {
            eprintln!("Erro ao atualizar o terminal");
            break;
        }

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Erro ao ler comando");
            continue;
        }

        let trimmed_input = input.trim();
        let mut parts = trimmed_input.splitn(2, char::is_whitespace);
        let command = parts.next().unwrap_or("").trim();
        let args = parts.next().unwrap_or("").trim();

        match command {
            "search" => {
                if args.is_empty() {
                    println!("Uso: search <termo>");
                    continue;
                }

                let results = search::search_products(args, &products_map);

                if results.is_empty() {
                    println!("Nenhum produto encontrado para '{}'.", args);
                    continue;
                }

                let query_norm = normalize_text(args);

                let mut direct_ids: HashSet<i32> = HashSet::new();
                for product in &results {
                    if normalize_text(&product.name).contains(&query_norm)
                        || normalize_text(&product.data.category).contains(&query_norm)
                        || normalize_text(&product.description).contains(&query_norm)
                    {
                        direct_ids.insert(product.id);
                    }
                }

                for product in results {
                    if direct_ids.contains(&product.id) {
                        println!("\n[BUSCA DIRETA]");
                    } else {
                        println!("\n[RELACIONADO]");
                    }

                    println!(
                        "ID: {} | Nome: {} | Categoria: {} | Preço: {:.2}",
                        product.id, product.name, product.data.category, product.data.price
                    );
                }
            }

            "list" => {
                println!("\nProdutos disponíveis:");
                for product in products_map.values() {
                    println!("- {} ({})", product.name, product.data.category);
                }
            }

            "exit" => {
                println!("Saindo...");
                break;
            }

            "help" => {
                print_help();
            }

            "" => {}

            _ => {
                if command.contains("search") || command.contains("serach") {
                    println!("Você quis dizer 'search'?");
                } else {
                    println!(
                        "Comando desconhecido: '{}'. Use 'help' para listar os comandos.",
                        command
                    );
                }
            }
        }
    }
}