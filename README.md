# pokefun-truelayer

## Usage

First, install and run the API server

### Cargo
```
cargo install --git https://github.com/conradludgate/pokefun-truelayer
pokefun-truelayer
```

### Docker
```
docker run --rm -it -p 8080:8080 ghcr.io/conradludgate/pokefun-truelayer:latest
```

Then make a request to the APIs

### xh
```
xh localhost:8080/pokemon/mewtwo
xh localhost:8080/pokemon/translated/mewtwo
```

### curl
```
curl 'http://localhost:8080/pokemon/mewtwo'
curl 'http://localhost:8080/pokemon/translated/mewtwo'
```

## Todo

[_] Caching - The Pokemon/Translation APIs are highly cachable
