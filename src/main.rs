mod csv_utils;
mod user_analysis;

use crate::csv_utils::clean_and_load_csv;
use crate::user_analysis::{analyze_users, identify_super_buyers, degree_distribution, fit_power_law, compute_distance_2_neighbors, build_category_connections, build_product_connections};
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn display_top_results(results: &HashMap<String, usize>, top_n: usize, label: &str) {
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| b.1.cmp(a.1)); 

    println!("\nTop {} {}:", top_n, label);
    for (key, value) in sorted_results.into_iter().take(top_n) {
        println!("{}: {}", key, value);
    }
}

fn display_top_connected_products(product_connections: &HashMap<String, HashSet<String>>, top_n: usize) {
    let mut products: Vec<_> = product_connections
        .iter()
        .map(|(product, connections)| (product, connections.len()))
        .collect();

    products.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nTop {} Products by Connections:", top_n);
    for (product, connection_count) in products.into_iter().take(top_n) {
        println!("Product: {}, Connections: {}", product, connection_count);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "ecommerce_dataset_updated.csv";
    let transactions = clean_and_load_csv(file_path)?;

    let user_summary = analyze_users(&transactions);

    let purchase_threshold = 5; 
    let spending_threshold = 1000.0; 
    let super_buyers = identify_super_buyers(&user_summary, purchase_threshold, spending_threshold);

    
    let super_buyer_counts: HashMap<String, usize> = super_buyers
        .iter()
        .map(|user| (user.clone(), user_summary[user].0))
        .collect();
    if super_buyer_counts.is_empty() {
        println!("\nNo super buyers found with the current thresholds.");
    } else {
        display_top_results(&super_buyer_counts, 5, "Super Buyers");
    }
    //or display_top_results(&super_buyer_counts, 5, "Super Buyers");

    println!("\nDegree Distribution:");
    let (user_degrees, product_degrees) = degree_distribution(&transactions);
    display_top_results(&user_degrees, 5, "Users by Degree");
    display_top_results(&product_degrees, 5, "Products by Degree");

    println!("\nPower-Law Fit:");
    let user_power_law = fit_power_law(&user_degrees);
    let product_power_law = fit_power_law(&product_degrees);
    println!("User Degree Power-Law Exponent: {:.2}", user_power_law);
    println!("Product Degree Power-Law Exponent: {:.2}", product_power_law);

    println!("\nDistance-2 Neighbors:");
    let distance_2 = compute_distance_2_neighbors(&transactions);
    let top_distance_2: HashMap<String, usize> = distance_2
        .iter()
        .map(|(node, neighbors)| (node.clone(), neighbors.len()))
        .collect();
    display_top_results(&top_distance_2, 5, "Nodes by Distance-2 Neighbors");

    println!("\nCategory-Based Connections:");
    let category_connections = build_category_connections(&transactions);
    let category_connection_counts: HashMap<String, usize> = category_connections
        .iter()
        .map(|(user, connections)| (user.clone(), connections.len()))
        .collect();
    display_top_results(&category_connection_counts, 5, "Users by Category-Based Connections");

    println!("\nProduct-Based Connections:");
    let product_connections = build_product_connections(&transactions);
    display_top_connected_products(&product_connections, 5);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::csv_utils::Transaction;
    use crate::user_analysis::{identify_super_buyers, build_category_connections};
    use std::collections::HashMap;

    #[test]
    fn test_super_buyers() {
        let user_summary = HashMap::from([
            ("user1".to_string(), (10, 500.0)),
            ("user2".to_string(), (3, 1200.0)),
            ("user3".to_string(), (2, 800.0)),
            ("user4".to_string(), (6, 1500.0)),
        ]);

        let purchase_threshold = 5;
        let spending_threshold = 1000.0;

        let super_buyers = identify_super_buyers(&user_summary, purchase_threshold, spending_threshold);

        assert!(super_buyers.contains(&"user1".to_string()));
        assert!(super_buyers.contains(&"user2".to_string()));
        assert!(super_buyers.contains(&"user4".to_string()));
        assert!(!super_buyers.contains(&"user3".to_string()));
    }

    #[test]
    fn test_compute_distance_2_neighbors() {
        let transactions = vec![
            Transaction {
                user_id: "user1".to_string(),
                product_id: "product1".to_string(),
                category: "category1".to_string(),
                final_price: 100.0,
            },
            Transaction {
                user_id: "user2".to_string(),
                product_id: "product1".to_string(),
                category: "category1".to_string(),
                final_price: 150.0,
            },
            Transaction {
                user_id: "user2".to_string(),
                product_id: "product2".to_string(),
                category: "category2".to_string(),
                final_price: 200.0,
            },
            Transaction {
                user_id: "user3".to_string(),
                product_id: "product2".to_string(),
                category: "category2".to_string(),
                final_price: 250.0,
            },
        ];
    
        let distance_2_neighbors = compute_distance_2_neighbors(&transactions);
    
        assert!(distance_2_neighbors["user1"].contains("user3"));
        assert!(!distance_2_neighbors["user1"].contains("user2")); 
    
        assert!(distance_2_neighbors["product1"].contains("product2")); 
        assert!(!distance_2_neighbors["product1"].contains("product1")); 
    
        assert!(distance_2_neighbors["user3"].contains("user1")); 
        assert!(!distance_2_neighbors["user3"].contains("user2")); 
    }
    

    #[test]
    fn test_category_connections() {
        let transactions = vec![
            Transaction {
                user_id: "user1".to_string(),
                product_id: "product1".to_string(),
                category: "category1".to_string(),
                final_price: 100.0,
            },
            Transaction {
                user_id: "user2".to_string(),
                product_id: "product2".to_string(),
                category: "category1".to_string(),
                final_price: 150.0,
            },
            Transaction {
                user_id: "user3".to_string(),
                product_id: "product3".to_string(),
                category: "category2".to_string(),
                final_price: 200.0,
            },
        ];

        let category_connections = build_category_connections(&transactions);

        assert!(category_connections["user1"].contains("user2"));
        assert!(!category_connections["user1"].contains("user3"));
    }
}
