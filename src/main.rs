mod csv_utils;
mod user_analysis;

use crate::csv_utils::clean_and_load_csv;
use crate::user_analysis::{analyze_users, identify_super_buyers, degree_distribution, fit_power_law, compute_distance_2_neighbors, build_category_connections, build_product_connections};
use std::error::Error;
use std::collections::{HashMap, HashSet};

fn display_top_results(results: &HashMap<String, usize>, top_n: usize, label: &str) {
    let mut sorted_results: Vec<_> = results.iter().collect();
    sorted_results.sort_by(|a, b| b.1.cmp(a.1)); // Sort by value descending

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

    products.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by number of connections descending

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
    display_top_results(&super_buyer_counts, 5, "Super Buyers");

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