
use::stress_tester::benchmarks::benchmark;

#[tokio::main]
async fn main() {
    benchmark(String::from("http://localhost:5000/api/posts/618045a63a5cc9120c2a855b"), 4, 40, 10).await;
}
