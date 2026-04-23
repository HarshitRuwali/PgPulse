# PgPulse
A Rust service/CLI that monitors your primary and read replica.


## Local setup 
For local environment, spin up the postgres docker containers via
docker-compose.yml
```bash
docker-compose up -d
```


## Configuration
Update the file `config.yaml` file with the connection details of your primary and read replica.


## Run the service
Pass the config file as the cli argument: 
```bash
cargo run -- --config config.yaml
```


## References:
 - [What to Look for if Your PostgreSQL Replication is Lagging](https://severalnines.com/blog/what-look-if-your-postgresql-replication-lagging/)
 - [Practical PostgreSQL Logical Replication: Setting Up an Experimentation Environment Using Docker](https://dev.to/ietxaniz/practical-postgresql-logical-replication-setting-up-an-experimentation-environment-using-docker-4h50)
