use trafikverket::DataFetcher;

#[tokio::main]
pub async fn main() {
    println!("Hello, world!");
    let data_fetcher = DataFetcher::create();
    data_fetcher.start().await;
}