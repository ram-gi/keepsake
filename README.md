# Keepsake

A self-hosted photo library for the homelab.

## Structure

| Path      | Description                     |
|-----------|---------------------------------|
| `server/` | Rust API server (Axum + Tokio)  |

## Server

Requires [Rust](https://rustup.rs) (stable).

```sh
cd server
cargo run
```

The server listens on port `8080` by default. Override with `PORT`:

```sh
PORT=3000 cargo run
```

### Endpoints

| Method | Path         | Response                         |
|--------|--------------|----------------------------------|
| GET    | `/api/hello` | `{"message":"Hello, world!"}`    |

## License

[MIT](LICENSE)
