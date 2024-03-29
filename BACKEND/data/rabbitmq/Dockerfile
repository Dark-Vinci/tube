FROM rabbitmq:latest

# Expose necessary ports
EXPOSE 5672 15672

# Set environment variables (optional)
ENV RABBITMQ_DEFAULT_USER=tomiwa
ENV RABBITMQ_DEFAULT_PASS=tomiwa

# Enable RabbitMQ Management Plugin
RUN rabbitmq-plugins enable rabbitmq_management