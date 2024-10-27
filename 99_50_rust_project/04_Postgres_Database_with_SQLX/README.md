# Rust to Postgres Database with SQLX - Rust Lang Tutorial 2021 | Jeremy Chone
- https://youtu.be/VuVOyUbFSI0?si=D24hFviOus79F734

# Error: failed to connect to database: password authentication failed in Rust
- https://stackoverflow.com/questions/63537490/error-failed-to-connect-to-database-password-authentication-failed-in-rust


# Run Docker (postgres)(docker)

```bash
$ docker run -p 5432:5432 --name test-postgress -e POSTGRES_PASSWORD=1234 -d postgres:latest
```

- docker 진입해서 postgres 들어가기

```bash
$ docker exec -it cfdae bash

root@cfdae7ed6feb:/# psql -U postgres

```

- postgres 명령어 
  - https://m.blog.naver.com/theswice/222042708567

```
postgres-# \du
                             List of roles
 Role name |                         Attributes
-----------+------------------------------------------------------------
 postgres  | Superuser, Create role, Create DB, Replication, Bypass RLS


# 사용자 조회
\du
  
```

```
postgres-# \d
              List of relations
 Schema |     Name      |   Type   |  Owner
--------+---------------+----------+----------
 public | ticket        | table    | postgres
 public | ticket_id_seq | sequence | postgres
(2 rows)

postgres-# \d ticket
                            Table "public.ticket"
 Column |  Type  | Collation | Nullable |              Default
--------+--------+-----------+----------+------------------------------------
 id     | bigint |           | not null | nextval('ticket_id_seq'::regclass)
 name   | text   |           |          |
```

