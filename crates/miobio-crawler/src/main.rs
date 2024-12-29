use miobio_core::Logging;

mod work;
use work::{Work, WorkQueue};

#[tokio::main]
async fn main() {
    Logging::init();
    Logging::info("Miobio-crawler starting up");
    ctrlc::set_handler(move || {
        WorkQueue::get().quit();
        println!("Ctrl+C pressed. Exiting...");
    })
    .expect("Error setting Ctrl+C handler");
    loop_function().await;
    Logging::info("Miobio-crawler shutting down");
}

async fn loop_function() {
    loop {
        let work = WorkQueue::get().pop();
        match work {
            Some(Work::Download { url }) => {
                Logging::info(&format!("Downloading {}", url));
                tokio::task::spawn(async {
                    let response = reqwest::get(url).await;
                    match response {
                        Ok(response) => {
                            Logging::info(&format!("Response: {}", response.status()));
                            
                        }
                        Err(e) => {
                            Logging::error(&format!("Error: {}", e));
                        }
                    }
                });
            }
            Some(Work::Analyse { path }) => {
                Logging::info(&format!("Analysing {}", path));
            }
            Some(Work::Quit) => {
                break;
            }
            None => {
                Logging::info("No work to do, sleeping");
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        }
    }
}
