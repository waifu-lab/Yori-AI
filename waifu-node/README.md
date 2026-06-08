# waifu-node

`waifu-node` is the local agent runtime for Yori AI. It connects to the `waifu` backend and exposes trusted desktop capabilities such as screenshots, window inspection, command execution, system information, audio device listing, keyboard input, mouse control, scrolling, and text entry.

This binary is intended to run beside the Yori AI desktop app or a compatible `waifu` backend. It is not a standalone chat backend.



## Capabilities

The node currently advertises these capability groups:

- `screen`: capture screenshots, read the active window, and list visible windows.
- `exec`: run system commands with arguments, timeout, and working directory.
- `sysinfo`: collect CPU, memory, disk, network, OS, and hostname information.
- `audio`: list audio input and output devices.
- `input`: send keyboard events, mouse movement, clicks, scrolling, and typed text.

Because `exec` and `input` can affect the local machine, only connect `waifu-node` to a backend you trust.

## Usage

Run with the default WebSocket endpoint:

```sh
cargo run -p waifu-node
```

Run with an explicit backend URL:

```sh
cargo run -p waifu-node -- --backend-url ws://127.0.0.1:8765/node
```

Run with a display name and token:

```sh
cargo run -p waifu-node -- --display-name my-desktop --token <token>
```

Run in IPC mode:

```sh
cargo run -p waifu-node -- --mode ipc
```

IPC mode connects to `\\.\pipe\waifu-node` on Windows and `/tmp/waifu-node.sock` on Unix-like systems.

## CLI Options

```text
--mode <ws|ipc>              Connection mode. Defaults to ws.
--backend-url <url>          Backend WebSocket URL. Defaults to ws://127.0.0.1:8765/node.
--display-name <name>        Human-readable node name. Defaults to the host name.
--token <token>              Authentication token sent in node_hello.
--reconnect-ms <ms>          Initial reconnect delay in milliseconds. Defaults to 3000.
```

## Skill Storage

Synced skills are stored in a platform-specific `waifu/skills` directory:

- Windows: `%APPDATA%\waifu\skills`
- macOS: `~/Library/Application Support/waifu/skills`
- Linux and other Unix-like systems: `~/.local/share/waifu/skills`

Each synced skill stores a `.manifest.json` file with the skill ID, version, sync time, and verified file hashes.

## Development

Build the node:

```sh
cargo build -p waifu-node
```

