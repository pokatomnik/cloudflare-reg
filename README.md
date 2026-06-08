# 🚀 cloudflare-reg

> A tiny Rust CLI that registers your device with Cloudflare WARP and spits out WireGuard-ready config. No fuss, just keys and IPs.

---

## 🤔 What is this?

`cloudflare-reg` is a command-line utility that talks to Cloudflare's WARP registration API, generates a fresh WireGuard keypair, and hands you back everything you need to plug into your WireGuard client. Two sentences, as promised. 😎

---

## 🎯 What is this project for?

Running your own WireGuard client with Cloudflare's WARP infrastructure — without the official client. Here's why you'd want this:

- **Bypass geo-restrictions** using Cloudflare's global edge network as a VPN exit point.
- **Use WARP on unsupported platforms** — your router, a headless server, a potato, whatever runs WireGuard.
- **Automate WARP registration** in scripts, CI/CD, or infrastructure-as-code setups.
- **Stay lightweight** — no bloated official client, just a single binary that gives you the config.
- **Proxy support** — register through SOCKS5/HTTP proxies when you're behind one.

You get: a private key, a peer public key, an endpoint, and IP addresses. Feed those into any WireGuard client and you're golden. 🎉

---

## 🛠️ Tech stack

| Tech                          | What it does                                                                      |
| ----------------------------- | --------------------------------------------------------------------------------- |
| 🦀 **Rust** (edition 2024)    | The whole thing. Fast, safe, crustacean-powered.                                  |
| 🎛️ **clap** (v4)              | Beautiful CLI argument parsing with derive macros.                                |
| 🌐 **reqwest** (blocking)     | HTTP client for the registration API. Supports `rustls`, system proxy, and SOCKS. |
| 🔐 **x25519-dalek**           | WireGuard-compatible X25519 key generation. Real crypto, not magic.               |
| 📦 **serde** / **serde_json** | JSON serialization for API requests and responses.                                |
| 🔢 **base64**                 | Encoding keys so they look nice and WireGuard-friendly.                           |
| ⏰ **chrono**                 | Timestamps because Cloudflare wants to know when you agreed to ToS.               |
| 🧰 **anyhow**                 | Error handling that doesn't make you cry.                                         |

---

## 📦 Usage

### Build from source

```bash
cargo build --release
# binary lives at target/release/cloudflare-reg
```

### Basic run

```bash
./cloudflare-reg
```

**Output example:**

```
Cloudflare registration

Private key*: gN9...abc=
Public key: 4fT...xyz=
Peer public key*: bmX...def=
IP addresses*: 172.16.0.2,2606:4700:110:...
Endpoint*: engage.cloudflareclient.com:2408

Values with * must be used in your client
```

### With a proxy 🕵️

Got a SOCKS5 or HTTP proxy? No problem:

```bash
./cloudflare-reg --proxy socks5h://127.0.0.1:1080
# or -p for short
./cloudflare-reg -p http://proxy.example.com:8080
```

### CLI reference

| Flag        | Short | Description                                                |
| ----------- | ----- | ---------------------------------------------------------- |
| `--proxy`   | `-p`  | Proxy URI. Works with `socks5h://`, `http://`, `https://`. |
| `--help`    | `-h`  | Prints help. You know the drill.                           |
| `--version` | `-V`  | Shows version.                                             |

### WireGuard config snippet

Take the output and drop it into your WireGuard config:

```ini
[Interface]
PrivateKey = <Private key* from output>
Address = <IP addresses* from output>

[Peer]
PublicKey = <Peer public key* from output>
Endpoint = <Endpoint* from output>
AllowedIPs = 0.0.0.0/0, ::/0
```

That's it. You're now surfing through Cloudflare's network. 🌊

---

## 📜 License

MIT — do whatever you want, just don't sue me. 🤙
