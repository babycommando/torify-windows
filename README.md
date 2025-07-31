# Torify For The Windows Terminal

A rust wrapper to "torify" any command line tool on Windows.  
Use it to route `curl`, `wget`, or anything else terminal through Tor just like Linux's `torify`.

[logo](/assets/logo.png)
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

Some programs might require to pass flags to it so they can be wrapped with the Socks. For example to open Microsoft Edge you would do as follows:
```
torify edge --proxy-server="socks5://127.0.0.1:9050"
```
note: make sure you have edge in your PATH as well.
