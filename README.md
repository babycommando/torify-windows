# Torify

🔒 A tiny Rust-powered wrapper to "torify" any command line tool on Windows.  
Use it to route `curl`, `wget`, or anything else through Tor — just like Linux's `torify`.

---

## 🧪 Features

- `torify start` — Launches a background Tor proxy (`tor.exe`)  
- `torify stop` — Gracefully stops the Tor process  
- `torify <command>` — Runs any command with SOCKS5 routing through Tor  
- Lightweight, portable, and fast.

---

## 🔧 Usage

```bash
# Start the background Tor proxy
.\torify.exe start

# Run a command anonymously
.\torify.exe curl https://api.ipify.org

# Stop the Tor proxy when done
.\torify.exe stop
```
