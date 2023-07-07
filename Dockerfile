FROM postgres:latest

# Set the password for the default postgres user
ENV POSTGRES_PASSWORD=password

# Expose the PostgreSQL port (default is 5432)
EXPOSE 5432

# Allow connections from outside the container
RUN echo "host all  all    0.0.0.0/0  md5" >> /var/lib/postgresql/data/pg_hba.conf && \
    echo "listen_addresses='*'" >> /var/lib/postgresql/data/postgresql.conf
