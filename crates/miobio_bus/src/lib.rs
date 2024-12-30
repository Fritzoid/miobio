use serde::{Deserialize, Serialize};

pub trait MessageBus {
    async fn connect(&mut self) -> Result<(), std::io::Error>;
    async fn next_message(&mut self) -> Option<String>;
}

const COMMAND_REQUEST_QUEUE: &str = "command_request_queue";
const COMMAND_RESPONSE_QUEUE: &str = "command_response_queue";

#[derive(Serialize, Deserialize, Debug)]
pub struct QuitMessage;

#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadMessage(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct AnalyseMessage(String);
