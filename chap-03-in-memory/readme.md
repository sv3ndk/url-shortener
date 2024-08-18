# chap 3 - in-memory url shortener

My own re-implementation of [chap 3 of the rust lambda book](https://github.com/rust-lambda/code-samples/tree/main/chapter-03).

# Usage

## Local requirements

- Local Rust setup
- Cargo Lambda
- AWS SAM CLI

## Tests

```sh
cargo test --bins
```

## Deployment

Build and deploy:

```sh
sam build --beta-features
sam deploy
```

Obtain the service URL:

```sh
aws cloudformation describe-stacks \
  --stack-name rust-lambda-chap03-url-shortener \
  --query 'Stacks[0].Outputs[?OutputKey==`UrlShortenerEndpoint`].OutputValue' \
  --output text
```

Tail logs:

```sh
sam logs --stack-name rust-lambda-chap03-url-shortener --name ShortenerFunction --tail
```

Create a link id:

```sh
# create a link
curl -X POST https://<see-stack-output>  \
     -H "Content-Type: application/json" \
     -d '{"url": "https://svend.kelesia.com"}'
```

retrieve it:

```sh
curl -D -  'https://<see-stack-output>?link_id=r5hjpud6pddnkdb67nluofyd'

HTTP/2 302
date: Sun, 18 Aug 2024 05:28:57 GMT
content-length: 0
location: https://svend.kelesia.com
apigw-requestid: csH4BgH7liAEJVQ=
```
