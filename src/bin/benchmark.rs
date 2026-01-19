// use colored::*;
use colored_text::Colorize;
use nano_rag::HNSW;
use std::time::{Duration, Instant};

const DIM: usize = 1536; 
const N_VECTORS: usize = 1_000_000; 
const N_QUERIES: usize = 10;

fn main() {
    println!("{}", r#"
                                                        
                                                       
                                                       
                                                       
                  :#################-                  
              =#########################+              
             #############################             
             :###########################=             
                =#####################=                
             ##=         .:::.         =##             
             #########*+===-====**########             
             #############################             
              -+#######################*=              
             #:    :=++++*****++++=:    :*             
             ########=-.       .-=########             
             #############################             
              :#########################=              
             +:      -=+**#****+=-.     :=             
             ######+-::.       ..:-+######             
             #############################             
               +#######################+.              
             #=      .:-=======-::      :#             
             #########+=:     :=+*########             
             #############################             
              ###########################              
                  +#################*.                 
                                                       
                                                       
                                                       
                                                       
    NANO RAG DATABASE
    Author: Swarnit Ghule
    GitHub: https://github.com/swar09/nano-rag
    "#.cyan().bold());

    let mut graph = HNSW::new(N_VECTORS, DIM);
    
    println!("\n    {}", format!("Initializing NANO-RAG DB (Inserting {} vectors)...", N_VECTORS).blue().bold());
    
    // Generate and insert vectors
    for _ in 0..N_VECTORS {
        let vec: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();
        let id = graph.vectors.insert(&vec);
        graph.insert(id, 16, 32, 100, 0.5); 
    }
    
    println!("    {}", "Starting Benchmark...".blue().bold());
    
    let mut duration_bf = Duration::ZERO;
    let mut duration_hnsw = Duration::ZERO;
    let mut correct_matches = 0;
    
    for _ in 0..N_QUERIES {
        let query: Vec<f32> = (0..DIM).map(|_| rand::random::<f32>()).collect();
        
        // Brute Force
        let start = Instant::now();
        let bf_results = graph.brute_force_search(&query, 1);
        duration_bf += start.elapsed();
        
        // HNSW Search
        let start = Instant::now();
        let hnsw_results = graph.search(&query, 1, 64);
        duration_hnsw += start.elapsed();
        
        if !bf_results.is_empty() && !hnsw_results.is_empty() {
             if bf_results[0].1 == hnsw_results[0].1 {
                 correct_matches += 1;
             }
        }
    }
    
    println!("    {}: {}", "Brute Force Avg".blue().bold(), format!("{:?}", duration_bf / N_QUERIES as u32).green().bold());
    println!("    {}: {}", "HNSW Search Avg".blue().bold(), format!("{:?}", duration_hnsw / N_QUERIES as u32).green().bold());
    println!("    {}: {}", "Recall".blue().bold(), format!("{:.2}%", (correct_matches as f64 / N_QUERIES as f64) * 100.0).green().bold());
}