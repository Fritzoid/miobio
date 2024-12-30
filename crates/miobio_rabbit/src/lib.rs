use miobio_bus::MessageBus;
use amqprs::connection::{Connection, OpenConnectionArguments};

pub struct RabbitBus {
    pub name: String,
    pub connection_args: OpenConnectionArguments,
    pub connection: Option<Connection>,
}

impl RabbitBus {
    pub fn new() -> RabbitBus {
        RabbitBus {
            name: "RabbitMQ".to_string(),
            connection_args: OpenConnectionArguments::new(
                "rabbitmq",
                5672,
                "guest",
                "guest",
            ),
            connection: None,
        }
    }
}

impl MessageBus for RabbitBus {
    async fn connect(&mut self) -> Result<(), std::io::Error> {
        self.connection = Some(Connection::open(&self.connection_args).await.unwrap());
        Ok(())
    }

    async fn next_message(&mut self) -> Option<String> {
        Some("Hello, World!".to_string())
    }
}