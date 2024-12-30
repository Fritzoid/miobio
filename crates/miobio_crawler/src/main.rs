use miobio_core::Logging;
use miobio_rabbit::RabbitBus;
use miobio_bus::MessageBus;

mod work;
use work::{Work, WorkQueue};

#[tokio::main]
async fn main() {
    Logging::init();
    Logging::info("Miobio-crawler starting up");
    ctrlc::set_handler(move || {
        WorkQueue::get().quit();
        Logging::info("Ctrl+C pressed. Exiting...");
    })
    .expect("Error setting Ctrl+C handler");

    let rb = RabbitBus::new();

    let jh = tokio::spawn(async move {
        message_listener(rb).await;
    });

    loop_function().await;
    jh.abort();
    Logging::info("Miobio-crawler shutting down");
}

async fn loop_function() {
    loop {
        let work = WorkQueue::get().pop();
        match work {
            Some(Work::Download { url }) => {
                Logging::info(&format!("Downloading {}", url));
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

async fn message_listener(mut rb: impl MessageBus) {

    rb.connect().await.unwrap();

    loop {
        if let Some(message) = rb.next_message().await {
            // Parse the message and create a Work item
            let work = parse_message_to_work(message);
            // Add the Work item to the WorkQueue
            WorkQueue::get().push(work);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

fn parse_message_to_work(message: String) -> Work {
    // Implement your message parsing logic here
    // For example, you can parse the message to determine the type of work
    Work::Download { url: message }
}
