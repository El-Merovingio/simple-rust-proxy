use warp::Filter;
use reqwest::Client;
use warp::hyper::Body;
use warp::reject::Reject;
use std::env;
use owo_colors::OwoColorize;

#[derive(Debug)]
struct CustomError;
impl Reject for CustomError {}

#[tokio::main]
async fn main() {
    // Get the URL from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("");
        println!("{}", "[i] Usage: cargo run <url>".bold().yellow());
        println!("");
        std::process::exit(1);
    }
    let base_url = args[1].clone();

    // Validate the URL
    if !base_url.starts_with("http://") {
        println!("");
        println!("{}", "[-] Error: URL must start with 'http://'".bold().red());
        println!("");
        std::process::exit(1);
    }

    let client = Client::new();

    let proxy = warp::any()
        .and(warp::header::headers_cloned())
        .and(warp::method())
        .and(warp::path::full())
        .and(warp::body::bytes())
        .and_then(move |headers, method, path: warp::filters::path::FullPath, body| {
            let client = client.clone();
            let base_url = base_url.clone();
            async move {
                let url = format!("{}{}", base_url, path.as_str());
                let req = client.request(method, &url).headers(headers).body(body);
                match req.send().await {
                    Ok(res) => {
                        let status = res.status();
                        let headers = res.headers().clone();
                        match res.bytes().await {
                            Ok(body) => {
                                let mut response = warp::http::Response::builder()
                                    .status(status)
                                    .body(Body::from(body))
                                    .map_err(|_| warp::reject::custom(CustomError))?;
                                *response.headers_mut() = headers;
                                Ok::<_, warp::reject::Rejection>(response)
                            },
                            Err(_) => Err(warp::reject::custom(CustomError)),
                        }
                    },
                    Err(_) => Err(warp::reject::custom(CustomError)),
                }
            }
        });

    warp::serve(proxy).run(([0, 0, 0, 0], 3030)).await;
}
