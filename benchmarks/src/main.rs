use anyhow::Result;
use clap::{Parser, Subcommand};
use futures::future::join_all;
use reqwest::Client;
use serde_json::json;
use std::time::{Duration, Instant};
use tabled::{Table, Tabled};
use tokio::time::sleep;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(name = "benchmarks")]
#[command(about = "Performance benchmarks for Axum vs ActixWeb")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run basic health check benchmark
    Health {
        /// Number of concurrent requests
        #[arg(short, long, default_value_t = 100)]
        concurrency: usize,
        /// Total number of requests
        #[arg(short, long, default_value_t = 1000)]
        requests: usize,
    },
    /// Run REST API benchmark
    Rest {
        /// Number of concurrent requests
        #[arg(short, long, default_value_t = 50)]
        concurrency: usize,
        /// Total number of requests
        #[arg(short, long, default_value_t = 500)]
        requests: usize,
    },
    /// Run GraphQL benchmark
    Graphql {
        /// Number of concurrent requests
        #[arg(short, long, default_value_t = 30)]
        concurrency: usize,
        /// Total number of requests
        #[arg(short, long, default_value_t = 300)]
        requests: usize,
    },
    /// Run all benchmarks
    All,
}

#[derive(Tabled)]
struct BenchmarkResult {
    framework: String,
    endpoint: String,
    total_requests: usize,
    concurrency: usize,
    total_time_ms: u128,
    avg_response_time_ms: f64,
    requests_per_second: f64,
    success_rate: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "benchmarks=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Health { concurrency, requests } => {
            run_health_benchmark(*concurrency, *requests).await?;
        }
        Commands::Rest { concurrency, requests } => {
            run_rest_benchmark(*concurrency, *requests).await?;
        }
        Commands::Graphql { concurrency, requests } => {
            run_graphql_benchmark(*concurrency, *requests).await?;
        }
        Commands::All => {
            info!("Running all benchmarks...");
            run_health_benchmark(100, 1000).await?;
            run_rest_benchmark(50, 500).await?;
            run_graphql_benchmark(30, 300).await?;
        }
    }

    Ok(())
}

async fn run_health_benchmark(concurrency: usize, total_requests: usize) -> Result<()> {
    info!("Running health check benchmark...");
    
    // Wait for servers to be ready
    wait_for_servers().await?;
    
    let axum_result = benchmark_endpoint(
        "http://localhost:3000/health",
        "GET",
        None,
        concurrency,
        total_requests,
        "Axum",
        "Health Check",
    ).await?;

    let actix_result = benchmark_endpoint(
        "http://localhost:3001/health", 
        "GET",
        None,
        concurrency,
        total_requests,
        "ActixWeb",
        "Health Check",
    ).await?;

    let table = Table::new(vec![axum_result, actix_result]);
    println!("{}", table);

    Ok(())
}

async fn run_rest_benchmark(concurrency: usize, total_requests: usize) -> Result<()> {
    info!("Running REST API benchmark...");
    
    wait_for_servers().await?;

    // Test product creation
    let product_data = json!({
        "name": "Test Product",
        "description": "A test product for benchmarking",
        "price": 1999,
        "inventory": 100
    });

    let axum_result = benchmark_endpoint(
        "http://localhost:3000/api/products",
        "POST",
        Some(product_data.clone()),
        concurrency,
        total_requests,
        "Axum",
        "Create Product",
    ).await?;

    let actix_result = benchmark_endpoint(
        "http://localhost:3001/api/products",
        "POST", 
        Some(product_data),
        concurrency,
        total_requests,
        "ActixWeb",
        "Create Product",
    ).await?;

    let table = Table::new(vec![axum_result, actix_result]);
    println!("{}", table);

    Ok(())
}

async fn run_graphql_benchmark(concurrency: usize, total_requests: usize) -> Result<()> {
    info!("Running GraphQL benchmark...");
    
    wait_for_servers().await?;

    let query_data = json!({
        "query": "{ products { id name price inventory } }"
    });

    let axum_result = benchmark_endpoint(
        "http://localhost:3000/graphql",
        "POST",
        Some(query_data.clone()),
        concurrency,
        total_requests,
        "Axum",
        "GraphQL Query",
    ).await?;

    let actix_result = benchmark_endpoint(
        "http://localhost:3001/graphql",
        "POST",
        Some(query_data),
        concurrency,
        total_requests,
        "ActixWeb", 
        "GraphQL Query",
    ).await?;

    let table = Table::new(vec![axum_result, actix_result]);
    println!("{}", table);

    Ok(())
}

async fn benchmark_endpoint(
    url: &str,
    method: &str,
    body: Option<serde_json::Value>,
    concurrency: usize,
    total_requests: usize,
    framework: &str,
    endpoint_name: &str,
) -> Result<BenchmarkResult> {
    let client = Client::new();
    let requests_per_worker = total_requests / concurrency;
    let mut tasks = Vec::new();

    let start_time = Instant::now();

    for _ in 0..concurrency {
        let client_clone = client.clone();
        let url_clone = url.to_string();
        let method_clone = method.to_string();
        let body_clone = body.clone();

        let task = tokio::spawn(async move {
            let mut successes = 0;
            let mut total_response_time = Duration::default();

            for _ in 0..requests_per_worker {
                let req_start = Instant::now();
                
                let request = match method_clone.as_str() {
                    "GET" => client_clone.get(&url_clone),
                    "POST" => {
                        let mut req = client_clone.post(&url_clone);
                        if let Some(json_body) = &body_clone {
                            req = req.json(json_body);
                        }
                        req
                    }
                    _ => client_clone.get(&url_clone),
                };

                match request.send().await {
                    Ok(response) if response.status().is_success() => {
                        successes += 1;
                        total_response_time += req_start.elapsed();
                    }
                    Ok(_) => {
                        // Non-success status code
                        total_response_time += req_start.elapsed();
                    }
                    Err(_) => {
                        // Request failed
                        total_response_time += req_start.elapsed();
                    }
                }
            }

            (successes, total_response_time)
        });

        tasks.push(task);
    }

    let results = join_all(tasks).await;
    let total_time = start_time.elapsed();

    let mut total_successes = 0;
    let mut total_response_time = Duration::default();

    for result in results {
        let (successes, response_time) = result?;
        total_successes += successes;
        total_response_time += response_time;
    }

    let success_rate = (total_successes as f64 / total_requests as f64) * 100.0;
    let avg_response_time_ms = total_response_time.as_millis() as f64 / total_requests as f64;
    let requests_per_second = total_requests as f64 / total_time.as_secs_f64();

    Ok(BenchmarkResult {
        framework: framework.to_string(),
        endpoint: endpoint_name.to_string(),
        total_requests,
        concurrency,
        total_time_ms: total_time.as_millis(),
        avg_response_time_ms,
        requests_per_second,
        success_rate,
    })
}

async fn wait_for_servers() -> Result<()> {
    let client = Client::new();
    let max_retries = 30;
    let retry_delay = Duration::from_secs(1);

    for i in 0..max_retries {
        let axum_ready = client.get("http://localhost:3000/health").send().await.is_ok();
        let actix_ready = client.get("http://localhost:3001/health").send().await.is_ok();

        if axum_ready && actix_ready {
            info!("Both servers are ready!");
            return Ok(());
        }

        if i == max_retries - 1 {
            if !axum_ready {
                warn!("Axum server not responding at http://localhost:3000");
            }
            if !actix_ready {
                warn!("ActixWeb server not responding at http://localhost:3001");
            }
            return Err(anyhow::anyhow!("Servers not ready after {} retries", max_retries));
        }

        info!("Waiting for servers... (attempt {}/{})", i + 1, max_retries);
        sleep(retry_delay).await;
    }

    Ok(())
}