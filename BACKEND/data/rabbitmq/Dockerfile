FROM rabbitmq:latest

# Expose necessary ports
EXPOSE 5672 15672

# Set environment variables (optional)
ENV RABBITMQ_DEFAULT_USER=miwa
ENV RABBITMQ_DEFAULT_PASS=miwa

# Enable RabbitMQ Management Plugin
RUN rabbitmq-plugins enable rabbitmq_management