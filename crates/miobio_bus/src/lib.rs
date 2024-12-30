pub trait MessageBus {
    async fn connect(&mut self) -> Result<(), std::io::Error>;
    fn next_message(&mut self) -> Option<String>;
}

pub struct Bus {}

pub struct Subscription {}

pub enum Command {
    Quit
}

pub enum Topic {
    Commands(Command),
}

impl Bus {
    pub fn new() -> Bus {
        Bus {
        }
    }

    pub fn subscribe(&mut self, topic: Topic) -> Subscription{
        Subscription {
        }
    }
}