# 🦀 ferrolink

A lightweight reverse TCP tunnel written in Rust — expose your localhost to the internet without port forwarding or complex configuration.

> Built as a learning project to understand TCP sockets, async Rust, and reverse tunnel architecture.

---

## The problem

You're developing locally and need to expose `localhost:3000` to the outside world — for a webhook, a client demo, or a colleague testing your app. Your router blocks incoming connections and you don't have a public IP.

ferrolink solves this with a single command.

```
internet → ferrolink-server (VPS) → ferrolink-client (your machine) → localhost:3000
```

Instead of the outside world trying to reach you (impossible without a public IP), **your machine connects first** — that's the "reverse" part. The server uses that open channel to forward incoming requests back to you.

---

## Architecture

```
[Your App]  ←→  [ferrolink-client]  ←——tunnel——→  [ferrolink-server]  ←→  [User]
localhost:3000     your machine                        VPS / public IP
```

Two binaries, two roles:

- **`ferrolink-server`** — runs on a public VPS. Listens on two ports: one for the client tunnel, one for incoming public requests.
- **`ferrolink-client`** — runs on your machine. Connects to the server and bridges traffic to your local app.

---

## Stack

- [Rust](https://www.rust-lang.org/)
- [Tokio](https://tokio.rs/) — async runtime
- [Clap](https://docs.rs/clap) — CLI
- [Tracing](https://docs.rs/tracing) — structured logging

---

## Getting started

> Coming soon as the project evolves.

```bash
# server (run on your VPS)
cargo run -p ferrolink-server

# client (run on your machine)
cargo run -p ferrolink-client -- --local-port 3000 --server your.vps.ip
```

---

## What I'm learning

- Async Rust with Tokio (tasks, streams, ownership across async boundaries)
- Raw TCP sockets — not HTTP, the actual protocol underneath
- Reverse proxy / tunnel architecture
- Rust workspace with multiple binaries

---

## License

MIT