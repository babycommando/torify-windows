# Torify For The Windows Terminal

A rust wrapper to "torify" any command line tool on Windows.  
Use it to route `curl`, `wget`, or anything else terminal through Tor just like Linux's `torify`.

---

Add the torify.exe binary to your system path.

- `torify start` — Launches a background Tor proxy (`tor.exe`)  
- `torify stop` — Gracefully stops the Tor process  
- `torify <command>` — Runs any command with SOCKS5 routing through Tor  

---

## Usage

```bash
# Start the background Tor proxy
torify start

# Run a command anonymously
torify curl https://api.ipify.org

# Stop the Tor proxy when done
torify stop
```
