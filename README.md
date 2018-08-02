# Lambda Rust SAM example


Build (Using Just https://github.com/casey/just)

```
just hello/build
```

Install SAM

Start local server

```
sam local start-api
```

Request:

```
curl http://127.0.0.1:3000/hello
```