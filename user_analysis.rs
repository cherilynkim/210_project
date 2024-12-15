use std::collections::{HashMap, HashSet};
use crate::csv_utils::Transaction;

pub fn analyze_users(transactions: &[Transaction]) -> HashMap<String, (usize, f64)> {
    let mut user_summary = HashMap::new();

    for transaction in transactions {
        let entry = user_summary
            .entry(transaction.user_id.clone())
            .or_insert((0, 0.0));
        entry.0 += 1; 
        entry.1 += transaction.final_price; 
    }

    user_summary
}

pub fn identify_super_buyers(
    user_summary: &HashMap<String, (usize, f64)>,
    purchase_threshold: usize,
    spending_threshold: f64,
) -> Vec<String> {
    user_summary
        .iter()
        .filter(|(_, &(total_purchases, total_spending))| {
            total_purchases >= purchase_threshold || total_spending >= spending_threshold
        })
        .map(|(user_id, _)| user_id.clone())
        .collect()
}

pub fn degree_distribution(transactions: &[Transaction]) -> (HashMap<String, usize>, HashMap<String, usize>) {
    let mut user_degrees = HashMap::new();
    let mut product_degrees = HashMap::new();

    for transaction in transactions {
        *user_degrees.entry(transaction.user_id.clone()).or_insert(0) += 1;
        *product_degrees.entry(transaction.product_id.clone()).or_insert(0) += 1;
    }

    (user_degrees, product_degrees)
}

pub fn fit_power_law(degrees: &HashMap<String, usize>) -> f64 {
    let degree_values: Vec<f64> = degrees.values().map(|&v| v as f64).collect();
    let n = degree_values.len() as f64;
    let k_min = degree_values.iter().cloned().fold(f64::INFINITY, f64::min);

    let log_sum: f64 = degree_values.iter().map(|&k| (k / k_min).ln()).sum();
    1.0 + n / log_sum
}

pub fn compute_distance_2_neighbors(transactions: &[Transaction]) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::new();

    for transaction in transactions {
        graph.entry(transaction.user_id.clone())
            .or_insert_with(HashSet::new)
            .insert(transaction.product_id.clone());

        graph.entry(transaction.product_id.clone())
            .or_insert_with(HashSet::new)
            .insert(transaction.user_id.clone());
    }

    let mut distance_2_neighbors = HashMap::new();

    for (node, neighbors) in &graph {
        let mut distance_2 = HashSet::new();
        for neighbor in neighbors {
            if let Some(second_neighbors) = graph.get(neighbor) {
                for second_neighbor in second_neighbors {
                    if second_neighbor != node {
                        distance_2.insert(second_neighbor.clone());
                    }
                }
            }
        }
        distance_2_neighbors.insert(node.clone(), distance_2);
    }

    distance_2_neighbors
}