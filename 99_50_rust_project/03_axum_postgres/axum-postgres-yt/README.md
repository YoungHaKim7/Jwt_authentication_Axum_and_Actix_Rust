# Is This the BEST Rust Web Framework? [AXUM + Postgres]
- https://youtu.be/sIkbTL5XskY?si=0mTLHIabRPBQi7V1

# Result

```bash
docker compose up -d


```


# docker-compose.yml

```yml
version: "3"
services:
  postgres:
    image: postgres:latest
#    contaner_name: postgres
    ports:
      - "6500:5432"
    volumes:
      - progresDB:/var/lib/postgresql/data
    env_file:
      - ./.env
  pgAdmin:
    image: dpage/pgadmin4
    container_name: pgAdmin
    env_file:
      - ./.env
    ports:
      - "5050:80"
volumes:
  progresDB:



```
