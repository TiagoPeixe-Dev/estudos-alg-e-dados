# Megastore Graph Search

Sistema de busca em grafo desenvolvido em Rust para simular um mecanismo inteligente de recomendação de produtos.

## Objetivo

O projeto tem como objetivo demonstrar o funcionamento de buscas em estruturas de dados baseadas em grafos, aplicadas a um cenário de e-commerce.

Cada produto é representado como um nó, e suas relações com outros produtos são representadas como arestas (edges), permitindo expandir os resultados de busca além da correspondência direta.

---

## Como funciona

O sistema realiza a busca em duas etapas:

### 1. Busca direta
O usuário digita um termo, e o sistema procura correspondências em:

- Nome do produto
- Categoria
- Descrição

Esses resultados são exibidos como:

[BUSCA DIRETA]

---

### 2. Expansão no grafo

Após encontrar os produtos iniciais, o sistema percorre o grafo e adiciona produtos relacionados com base em:

- COMPRADO_JUNTO
- COMPATIVEL_COM

Esses resultados aparecem como:

[RELACIONADO]

---

## Estrutura do projeto

src/
 ├── main.rs
 ├── search.rs
 ├── file_loader.rs
 └── models.rs

data/
 └── arquivos JSON com os produtos

---

## Modelo de dados

Cada produto segue o formato:

{
  "id": 123,
  "name": "Câmera A7S",
  "description": "Câmera profissional...",
  "data": {
    "price": 15000.0,
    "category": "câmeras"
  },
  "edges": [
    {
      "to_id": 888,
      "relationship": "COMPATIVEL_COM"
    }
  ]
}

---

## Como executar

cargo run

---

## Comandos disponíveis

search <termo>
list
help
exit

---

## Diferenciais do projeto

- Busca com normalização de texto (ignora acentos)
- Classificação de resultados (direto vs relacionado)
- Expansão baseada em grafo
- Estrutura modular em Rust
- Simulação de recomendação de produtos

---

## Exemplo de uso

> search camera

[BUSCA DIRETA]
Câmera A7S

[RELACIONADO]
Lente 50mm
Tripé
Microfone

---

## Conclusão

O projeto demonstra como grafos podem ser utilizados para enriquecer sistemas de busca, indo além de simples correspondência textual e oferecendo recomendações baseadas em relacionamento entre dados.