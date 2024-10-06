# Jwt_authentication_Axum_and_Actix_Rust
https://youtu.be/n2M4A4mO0QU?si=wxij-2saqjrFtbnK

<hr>

# Original source
- https://github.com/cudidotdev/JWT-Authentication-with-Rust-Axum-and-Actix
- Fork
  - https://github.com/YoungHaKim7/JWT-Authentication-with-Rust-Axum-and-Actix


<hr/>

# curl로 테스트

- curl test 하니 잘 된다
  - https://stackoverflow.com/questions/7172784/how-do-i-post-json-data-with-curl

```bash

$ curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"username":"xyz","password":"xyz"}' \
        http://localhost:3000/login
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ4eXoiLCJleHAiOjE3MjgxOTYwNzN9._lTFxkiMc1zucEmPmfn-WY_feLgXxN5GKql0XiW6k7A"}⏎    
```


<hr />

# Backend In Rust - 01 Building a basic server | Red Rustacean 
- https://youtu.be/kcWsEEVvW88?si=BW7Jbq0_mGN_iSFe

# Backend in Rust 02 - JWT Authentication, Sea ORM, Axum, Rust | Red Rustacean 
- https://youtu.be/QqPqlUqxW2A?si=o6kHYQCTm7YTVIkp

<hr />

# Building Web APIs With Rust and Axum - An Introduction | Rainer Stropek
- https://youtu.be/q53xalVoc6w?si=lxTa2V_dgqqRel6a
  - github
    - https://github.com/rstropek/rust-api-fxs

<hr />

# (axum)JWT Authentication in Rust | A Step-by-Step Guide | Semicolon
- https://youtu.be/p2ljQrRl0Mg?si=5mYkblL4qzpHa8sO
