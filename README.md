# Torify For The Windows Terminal

A rust wrapper to "torify" any command line tool on Windows.  
Use it to route `curl`, `wget`, or anything else terminal through Tor just like Linux's `torify`.

![logo](/assets/logo.png)
---

Execute the main torify.exe binary or build your own from scratch.
This project already includes the binary for Tor Windows x86_64 but you can grab others at the [official Tor website](https://www.torproject.org/download/tor). You may also choose between tunning the torify binary included or build one yourself. Building instructions below.


After cloning or building the rust program, add the torify.exe binary to your system path to run it anywhere in your terminal.

- `torify start` — Launches a background Tor proxy (`tor.exe`)  
- `torify stop` — Gracefully stops the Tor process  
- `torify <command>` — Runs any command with SOCKS5 routing through Tor  

---

## Building
1. Clone the project
2. Build with: `cargo build --release`
3. Add "tor.exe" to the same folder of your built binary 

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
