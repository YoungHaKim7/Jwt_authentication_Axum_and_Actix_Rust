# run
```
cargo run  
```
- curl test 하니 잘 된다
  - https://stackoverflow.com/questions/7172784/how-do-i-post-json-data-with-curl

```bash

$ curl --header "Content-Type: application/json" \
        --request POST \
        --data '{"username":"xyz","password":"xyz"}' \
        http://localhost:3000/login
{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ4eXoiLCJleHAiOjE3MjgxOTYwNzN9._lTFxkiMc1zucEmPmfn-WY_feLgXxN5GKql0XiW6k7A"}⏎    
```

