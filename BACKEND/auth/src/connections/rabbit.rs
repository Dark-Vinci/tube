use {
    crate::config::config::Config,
    lapin::{
        options::{BasicPublishOptions, QueueDeclareOptions},
        publisher_confirm::PublisherConfirm,
        types::FieldTable,
        BasicProperties, Channel, Connection, ConnectionProperties,
    },
    sdk::constants::helper::AUTH_SERVICE_QUEUE,
    tokio_async_drop::tokio_async_drop,
};

#[derive(Debug)]
pub struct Rabbit {
    con: Channel,
}

impl Drop for Rabbit {
    fn drop(&mut self) {
        tokio_async_drop!({ self.con.close(000, "close").await.unwrap() });
    }
}

#[async_trait::async_trait]
pub trait RabbitTrait {
    async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        options: BasicPublishOptions,
        payload: Vec<u8>,
        properties: BasicProperties,
    ) -> Result<PublisherConfirm, lapin::Error>;
}

#[async_trait::async_trait]
impl RabbitTrait for Rabbit {
    async fn publish(
        &self,
        exchange: &str,
        routing_key: &str,
        options: BasicPublishOptions,
        payload: Vec<u8>,
        properties: BasicProperties,
    ) -> Result<PublisherConfirm, lapin::Error> {
        return self
            .con
            .basic_publish(exchange, routing_key, options, &payload, properties)
            .await;
    }
}

impl Rabbit {
    pub async fn new(c: &Config) -> Result<Box<dyn RabbitTrait + Send + Sync>, String> {
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

        let _ = channel
            .clone()
            .unwrap()
            .queue_declare(
                AUTH_SERVICE_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        Ok(Box::new(Self {
            con: channel.unwrap(),
        }))
    }
}
