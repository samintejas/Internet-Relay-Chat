# Rust Chat (Simple TCP Broadcast)

A minimal chat application in Rust, built from scratch to understand networking, concurrency, and shared state management.  
It works like a very simple **IRC-style chat server**: multiple clients connect, send text messages, and see each other’s messages.

---

## 🚀 How It Works

- **Server**
  - Listens on a host/port (`127.0.0.1:7878` by default).
  - Accepts multiple clients.
  - Spawns a thread per client to read incoming lines.
  - Broadcasts each message to all connected clients.
  - Cleans up disconnected clients.

- **Client**
  - Connects to the server.
  - Spawns two loops:
    - `stdin → socket` (send messages you type).
    - `socket → stdout` (print messages from others).
  - Multiple clients can run simultaneously and chat.

---

## 🛠️ Run Locally

Clone and build:

```bash
git clone https://github.com/yourname/rust-chat.git
cd rust-chat
cargo build
```

### Start the server

```bash
cargo run --bin server
```

### Start the client

```bash
cargo run --bin client
```

## 🔮 Next Steps

This chat is just the foundation. Here’s how you can evolve it:

1. **Nicknames**
   - Let clients set a nickname (`/nick samin`) instead of showing raw IP addresses.
   - Messages become more readable (`[samin]: hello`).

2. **Channels / Rooms**
   - Add support for joining rooms (`/join rust`) so not all clients see all messages.
   - Useful for separating discussions.

3. **Private Messages**
   - Command `/msg <nick> <text>` to send direct messages between clients.
   - Requires server to track users by name.

4. **User Join/Leave Notifications**
   - Broadcast when someone connects or disconnects (`samin joined`, `tejas left`).
   - Improves awareness in the chat.

5. **Better Protocol**
   - Instead of raw lines, define a simple protocol like:
     ```
     MSG <nick> <content>
     JOIN <room>
     LEAVE <room>
     ```
   - Easier to parse, extend, and debug.

6. **Configuration**
   - Make host/port configurable with CLI args or environment variables.
   - Example: `cargo run --bin server -- 0.0.0.0 9000`.

7. **Security**
   - Use TLS (`rustls`) for encrypted connections.
   - Prevents eavesdropping when chatting over the internet.

8. **Polishing**
   - Replace `println!` with a proper logger (`env_logger`, `tracing`).
   - Add timestamps and levels (INFO, ERROR).
   - Improve error handling instead of unwrapping everywhere.

---

## 🎯 Why This Matters

- Builds intuition for **network programming** with TCP streams.
- Shows **safe concurrency** using `Arc<Mutex<…>>`.
- Forms the backbone of real-time systems (chat, multiplayer games, collab editors).
- Can grow into a **minimal IRC clone** or even a **custom protocol playground**.

---

## 📌 License

MIT
