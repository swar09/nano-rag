
import photon_db
import json
import torch
import sys
import os
import time
import argparse
import numpy as np
import csv
from sentence_transformers import SentenceTransformer
from tqdm import tqdm

# Constants
EMBEDDING_MODEL = 'all-MiniLM-L6-v2'
REPORT_FILENAME = "benchmark_report.csv"
DB_FILENAME = "benchmark_vector_db.pho"
META_FILENAME = "benchmark_meta.json"

def calculate_recall(approx_results, ground_truth, k):
    """
    Calculates Recall@K.
    approx_results: list of list of (dist, id) from HNSW
    ground_truth: list of list of (dist, id) from Brute Force
    """
    total_recall = 0
    for i in range(len(approx_results)):
        approx_ids = set([res[1] for res in approx_results[i]])
        gt_ids = set([res[1] for res in ground_truth[i][:k]])
        
        # Intersection count
        intersect = approx_ids.intersection(gt_ids)
        if len(gt_ids) > 0:
            total_recall += len(intersect) / len(gt_ids)
        else:
            total_recall += 0
            
    return total_recall / len(approx_results)

def load_text_data(path, limit=None):
    texts = []
    if not path or not os.path.exists(path):
        return []
        
    if os.path.isfile(path):
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            for line in f:
                line = line.strip()
                if len(line) > 20:
                    texts.append(line)
                if limit and len(texts) >= limit: break
    elif os.path.isdir(path):
        import glob
        files = glob.glob(os.path.join(path, "*.txt"))
        for file_path in files:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
                paragraphs = [p.strip() for p in content.split('\n\n') if len(p.strip()) > 20]
                texts.extend(paragraphs)
                if limit and len(texts) >= limit: break
    return texts

def generate_synthetic_data(n, dim):
    """Generates random normalized vectors."""
    print(f"Generating {n} random vectors of dimension {dim}...")
    vectors = np.random.rand(n, dim).astype(np.float32)
    # Normalize
    norms = np.linalg.norm(vectors, axis=1, keepdims=True)
    vectors = vectors / norms
    return vectors

def main():
    parser = argparse.ArgumentParser(description="Comprehensive Photon DB Benchmark")
    parser.add_argument("--data", type=str, help="Path to text data (optional if synthetic)")
    parser.add_argument("--limit", type=int, default=5000, help="Max text items to process")
    parser.add_argument("--synthetic", type=int, help="Number of synthetic vectors to generate if no data provided")
    parser.add_argument("--dim", type=int, default=384, help="Dimension for synthetic data")
    
    parser.add_argument("--m", type=int, default=16, help="HNSW M parameters")
    parser.add_argument("--ef_construction", type=int, default=200, help="HNSW ef_construction")
    parser.add_argument("--ef_search", type=int, default=100, help="HNSW ef_search")
    parser.add_argument("--k", type=int, default=10, help="Top-K for recall calculation")
    parser.add_argument("--queries", type=int, default=50, help="Number of items to use as queries")
    parser.add_argument("--save", action="store_true", help="Save DB to disk")
    
    args = parser.parse_args()
    
    corpus_embeddings = None
    query_embeddings = None
    texts = []
    
    # 1. Data Loading / Generation
    if args.data:
        print(f"Loading data from {args.data}...")
        full_texts = load_text_data(args.data, args.limit + args.queries)
        if len(full_texts) < args.queries + 10:
             print("Warning: Not enough text data. Switching to synthetic generation for remainder?")
        else:
             texts = full_texts[:-args.queries]
             query_texts = full_texts[-args.queries:]
             
             print("Generating embeddings (CPU mode)...")
             model = SentenceTransformer(EMBEDDING_MODEL, device="cpu")
             args.dim = model.get_sentence_embedding_dimension() # Override dim
             
             corpus_embeddings = model.encode(texts, convert_to_numpy=True, show_progress_bar=True)
             query_embeddings = model.encode(query_texts, convert_to_numpy=True, show_progress_bar=True)

    if corpus_embeddings is None:
        if args.synthetic:
            print(f"Using Synthetic Data: {args.synthetic} vectors.")
            corpus_embeddings = generate_synthetic_data(args.synthetic, args.dim)
            query_embeddings = generate_synthetic_data(args.queries, args.dim)
            texts = [f"Synthetic Doc {i}" for i in range(args.synthetic)]
        else:
            print("Error: Must provide --data or --synthetic [N]")
            sys.exit(1)
            
    print(f"Index items: {len(corpus_embeddings)} | Query items: {len(query_embeddings)} | Dim: {args.dim}")
    
    # 2. Benchmark Indexing
    print("Benchmarking Indexing...")
    db = photon_db.PyHNSW(len(corpus_embeddings), args.dim, args.m, args.ef_construction)
    
    start_index = time.time()
    for vec in tqdm(corpus_embeddings, desc="Indexing"):
        db.insert(vec.tolist(), args.m, args.m * 2, args.ef_construction, 1.0)
    end_index = time.time()
    
    index_time_total = end_index - start_index
    index_time_per_item = index_time_total / len(corpus_embeddings) if len(corpus_embeddings) > 0 else 0
    print(f"Indexing Time: {index_time_total:.4f}s ({1/index_time_per_item:.2f} items/s)")
    
    if args.save:
        print(f"Saving to {DB_FILENAME}...")
        db.save(DB_FILENAME)
        if texts and not args.synthetic:
            with open(META_FILENAME, 'w') as f:
                json.dump(texts, f)
    
    # 3. Benchmark Search
    print("Benchmarking Search...")
    latencies = []
    hnsw_results = []
    
    for q_vec in query_embeddings:
        t_start = time.time()
        res = db.search(q_vec.tolist(), args.k, args.ef_search)
        t_end = time.time()
        latencies.append((t_end - t_start) * 1000)
        hnsw_results.append(res)
        
    avg_latency = np.mean(latencies)
    p95_latency = np.percentile(latencies, 95)
    print(f"Avg Latency: {avg_latency:.4f}ms | P95: {p95_latency:.4f}ms")
    
    # 4. Ground Truth
    print("Calculating Recall (Brute Force)...")
    gt_results = []
    # Manual BF using numpy for reliability
    for q_vec in tqdm(query_embeddings, desc="Brute Force"):
        dists = np.sum((corpus_embeddings - q_vec)**2, axis=1)
        nearest_indices = np.argsort(dists)[:args.k]
        # HNSW returns (dist, id), so we format BF output the same
        res = [(dists[i], i) for i in nearest_indices]
        gt_results.append(res)
        
    recall = calculate_recall(hnsw_results, gt_results, args.k)
    print(f"Recall@{args.k}: {recall:.4f}")
    
    # 5. Save Report
    file_exists = os.path.isfile(REPORT_FILENAME)
    with open(REPORT_FILENAME, 'a', newline='') as csvfile:
        fieldnames = [
            'timestamp', 'dataset_size', 'dim', 'M', 'ef_construction', 'ef_search', 
            'recall_at_k', 'index_time_s', 'items_per_sec', 'avg_latency_ms', 'p95_latency_ms'
        ]
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        if not file_exists: writer.writeheader()
        writer.writerow({
            'timestamp': time.strftime("%Y-%m-%d %H:%M:%S"),
            'dataset_size': len(corpus_embeddings),
            'dim': args.dim,
            'M': args.m,
            'ef_construction': args.ef_construction,
            'ef_search': args.ef_search,
            'recall_at_k': round(recall, 4),
            'index_time_s': round(index_time_total, 4),
            'items_per_sec': round(1/index_time_per_item, 2) if index_time_per_item > 0 else 0,
            'avg_latency_ms': round(avg_latency, 4),
            'p95_latency_ms': round(p95_latency, 4)
        })
    print(f"\nReport saved to {REPORT_FILENAME}")

if __name__ == "__main__":
    main()
