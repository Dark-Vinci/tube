use {
    crate::config::config::Config,
    lapin::{
        options::{BasicPublishOptions, QueueDeclareOptions},
        types::FieldTable,
        BasicProperties, Channel, Connection, ConnectionProperties,
    },
    sdk::constants::helper::AUTH_SERVICE_QUEUE,
};

#[derive(Debug)]
pub struct Rabbit(Channel);

impl Rabbit {
    pub async fn new(c: &Config) -> Result<Self, String> {
        let uri: &str = &format!(
            "amqp://{0}:{1}@{2}:{3}",
            c.rabbitmq_username, c.rabbitmq_password, c.rabbitmq_host, c.rabbitmq_port
        );

        let options = ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio);

        let connection = Connection::connect(uri, options).await;

        if let Err(e) = connection {
            return Err(e.to_string());
        }

        let channel = connection.unwrap().create_channel().await;

        if let Err(e) = channel {
            return Err(e.to_string());
        }

        let channel = channel.unwrap();

        let _ = channel
            .queue_declare(
                AUTH_SERVICE_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        Ok(Self(channel))
    }

    pub async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        payload: Vec<u8>,
    ) -> bool {
        let a = self
            .0
            .basic_publish(
                exchange,
                routing_key,
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await;

        if let Err(_err) = a {
            return false;
        }

        true
    }
}
