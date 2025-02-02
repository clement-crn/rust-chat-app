# Use official PostgreSQL image from Docker Hub
FROM postgres:latest

# Set environment variables for PostgreSQL configuration
ENV POSTGRES_USER=${POSTGRES_USER}
ENV POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
ENV POSTGRES_DB=${POSTGRES_DB}

# Expose the default PostgreSQL port
EXPOSE 5432

# Run the default entrypoint for PostgreSQL
CMD ["docker-entrypoint.sh", "postgres"]

