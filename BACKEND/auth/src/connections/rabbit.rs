use {
    crate::config::config::Config,
    lapin::{
        options::QueueDeclareOptions, types::FieldTable, Connection, ConnectionProperties,
    },
    sdk::constants::helper::AUTH_SERVICE_QUEUE,
    tokio_async_drop::tokio_async_drop,
};

#[derive(Debug)]
pub struct Rabbit {
    con: Connection,
}

impl Drop for Rabbit {
    fn drop(&mut self) {
        tokio_async_drop!({ self.con.close(000, "close").await.unwrap() });
    }
}

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

        let connection = connection.unwrap();

        let channel: Result<_, _> = connection.create_channel().await;

        if let Err(e) = channel {
            return Err(e.to_string());
        }

        let _ = channel
            .unwrap()
            .queue_declare(
                AUTH_SERVICE_QUEUE,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        Ok(Self { con: connection })
    }
}
